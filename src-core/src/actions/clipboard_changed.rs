use std::convert::Infallible;
use std::fs;
use std::sync::atomic::AtomicI32;
use std::time::Duration;

use getset::Getters;
use misty_vm::async_task::MistyAsyncTaskTrait;
use misty_vm::client::AsReadonlyMistyClientHandle;
use misty_vm::services::MistyServiceTrait;
use misty_vm::states::MistyStateTrait;
use misty_vm::{client::MistyClientHandle, controllers::MistyControllerContext};
use misty_vm::{MistyAsyncTask, MistyState};

use crate::app_setup::Clipboard;
use crate::libs::common::ClipboardPaster;
use crate::models::clipboard::{
    batch_remove_clipboard_items, load_clipboard_histories, remove_all_clipboard_items,
    save_clipboard_item_image, save_clipboard_item_text, ClipboardHistoryItem,
    ClipboardHistoryItemImage, ClipboardHistoryItemText,
};
use crate::result::KCResult;
use crate::shell::IShellHandle;
use crate::utils::{raw_bytes_to_base64_png, raw_bytes_to_png};

use super::preference::get_limit_count;

pub enum OnClipboardActionArg {
    Text(String),
    Image {
        width: usize,
        height: usize,
        bytes: Vec<u8>,
    },
}

#[derive(Default, MistyState, Getters)]
pub struct ClipboardHistoryListState {
    #[getset(get = "pub")]
    list: Vec<ClipboardHistoryItem>,
    alloc: AtomicI32,
}

#[derive(Default, MistyState, Getters)]
pub struct ClipboardHistoryActiveState {
    #[getset(get = "pub")]
    active_id: i32,
    #[getset(get = "pub")]
    item: Option<ClipboardHistoryItem>,
}

#[derive(MistyAsyncTask)]
struct DelayPasteAsyncTask;

struct ActiveLoc {
    len: usize,
    id: i32,
    index: usize,
}

fn alloc_list_item_id(cx: MistyClientHandle) -> i32 {
    ClipboardHistoryListState::update(cx, |state| {
        let id = state
            .alloc
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        id
    })
}

fn active_clipboard_by_id(cx: MistyClientHandle, id: i32) {
    let item = ClipboardHistoryListState::map(cx, |state| {
        state
            .list()
            .iter()
            .find(|v| v.id() == id)
            .map(|v| v.clone())
    });
    ClipboardHistoryActiveState::update(cx, |state| {
        state.active_id = id;
        state.item = item;
    });
}

fn find_active_clipboard_history_loc(cx: MistyClientHandle) -> Option<ActiveLoc> {
    let len = ClipboardHistoryListState::map(cx, |state| state.list.len());
    let id = ClipboardHistoryActiveState::map(cx, |state| state.active_id);
    let index = ClipboardHistoryListState::map(cx, |state| {
        state
            .list
            .iter()
            .enumerate()
            .find(|(_, v)| v.id() == id)
            .map(|(i, _)| i)
    });

    return if let Some(index) = index {
        Some(ActiveLoc { len, id, index })
    } else {
        None
    };
}

pub fn init_clipboard_histories(cx: MistyClientHandle) -> KCResult<()> {
    let histories = load_clipboard_histories(cx.readonly_handle())?;
    let max_id = histories.iter().fold(0, |prev, curr| prev.max(curr.id()));
    tracing::info!("load histories {}", histories.len());

    if !histories.is_empty() {
        ClipboardHistoryActiveState::update(cx, |state| {
            state.active_id = histories[0].id();
            state.item = Some(histories[0].clone());
        });
    }
    ClipboardHistoryListState::update(cx, |state| {
        state.list = histories;
        state
            .alloc
            .store(max_id + 1, std::sync::atomic::Ordering::SeqCst);
    });
    Ok(())
}

pub fn next_clipboard_active(cx: MistyClientHandle) -> KCResult<()> {
    let loc = find_active_clipboard_history_loc(cx);
    if let Some(loc) = loc {
        if loc.index + 1 < loc.len {
            let id = ClipboardHistoryListState::map(cx, |state| {
                state.list.get(loc.index + 1).unwrap().id()
            });
            active_clipboard_by_id(cx, id);
        }
    }
    Ok(())
}

pub fn previous_clipboard_active(cx: MistyClientHandle) -> KCResult<()> {
    let loc = find_active_clipboard_history_loc(cx);
    if let Some(loc) = loc {
        if loc.index >= 1 {
            let id = ClipboardHistoryListState::map(cx, |state| {
                state.list.get(loc.index - 1).unwrap().id()
            });
            active_clipboard_by_id(cx, id);
        }
    }
    Ok(())
}

pub fn retain_ncount_clipboard_histories(cx: MistyClientHandle, n: usize) -> KCResult<()> {
    let removed: Vec<ClipboardHistoryItem> = ClipboardHistoryListState::update(cx, |state| {
        if n < state.list.len() {
            state.list.drain(n..).collect()
        } else {
            Default::default()
        }
    });

    let ids = removed.into_iter().map(|v| v.id()).collect();
    batch_remove_clipboard_items(cx, ids)?;
    Ok(())
}

pub fn active_or_paste_from_clipboard_by_index(
    cx: MistyClientHandle,
    app: impl IShellHandle,
    index: usize,
) -> KCResult<()> {
    let active_id = ClipboardHistoryActiveState::map(cx, |state| state.active_id);
    let target_id =
        ClipboardHistoryListState::map(cx, |state| state.list.get(index).map(|v| v.id()));
    if !target_id.is_some() {
        return Ok(());
    }
    let target_id = target_id.unwrap();
    if active_id == target_id {
        return paste_from_clipboard_by_active(cx, app);
    } else {
        active_clipboard_by_id(cx, target_id);
        return Ok(());
    }
}

pub fn paste_from_clipboard_by_active(
    cx: MistyClientHandle,
    shell: impl IShellHandle,
) -> KCResult<()> {
    let (id, item) =
        ClipboardHistoryActiveState::map(cx, |state| (state.active_id, state.item.clone()));
    if item.is_none() {
        return Ok(());
    }
    let item = item.unwrap();

    // remove active item from list
    batch_remove_clipboard_items(cx, vec![id])?;
    ClipboardHistoryListState::update(cx, |state| {
        state.list = state
            .list
            .clone()
            .into_iter()
            .filter(|v| v.id() != id)
            .collect();
    });
    // reset active state
    ClipboardHistoryActiveState::update(cx, |state| {
        state.active_id = 0;
        state.item = None;
    });

    match item {
        ClipboardHistoryItem::Text(item) => {
            Clipboard::of(cx).write_text(item.text().to_string());
        }
        ClipboardHistoryItem::Image(item) => {
            let (width, height) = item.dimensions();
            Clipboard::of(cx).write_image(width as usize, height as usize, item.bytes());
        }
    }
    shell.hide();

    DelayPasteAsyncTask::spawn_once(cx, move |cx| async move {
        tokio::time::sleep(Duration::from_millis(100)).await;

        cx.schedule(|cx| {
            ClipboardPaster::of(cx).paste();
            Ok::<_, std::convert::Infallible>(())
        });
        Ok::<_, std::convert::Infallible>(())
    });
    Ok(())
}

pub fn handle_clipboard_changed(cx: MistyClientHandle, arg: OnClipboardActionArg) -> KCResult<()> {
    let limit = get_limit_count(cx.readonly_handle());
    match arg {
        OnClipboardActionArg::Text(text) => {
            retain_ncount_clipboard_histories(cx, limit - 1)?;
            let id = alloc_list_item_id(cx);
            let item = ClipboardHistoryItemText::new(id, text.clone());
            save_clipboard_item_text(cx.readonly_handle(), item.clone())?;
            ClipboardHistoryListState::update(cx, |state| {
                state.list.insert(0, ClipboardHistoryItem::Text(item));
            });
            active_clipboard_by_id(cx, id);
            Ok(())
        }
        OnClipboardActionArg::Image {
            width,
            height,
            bytes,
        } => {
            let resource =
                cx.resource_manager()
                    .insert(raw_bytes_to_base64_png(bytes.clone(), width, height));
            retain_ncount_clipboard_histories(cx, limit - 1)?;
            let id = alloc_list_item_id(cx);
            let item =
                ClipboardHistoryItemImage::new(id, width as i32, height as i32, bytes, resource);
            save_clipboard_item_image(cx.readonly_handle(), item.clone())?;
            ClipboardHistoryListState::update(cx, |state| {
                state.list.insert(0, ClipboardHistoryItem::Image(item));
            });
            active_clipboard_by_id(cx, id);
            Ok(())
        }
    }
}

pub fn action_change_clipboard_active(
    cx: MistyControllerContext,
    id: i32,
) -> Result<(), Infallible> {
    let cx = cx.handle();
    active_clipboard_by_id(cx, id);
    Ok(())
}

pub fn action_paste_clipboard_if_active(
    cx: MistyControllerContext,
    (shell, id): (impl IShellHandle, i32),
) -> KCResult<()> {
    let cx = cx.handle();
    let (active_id, valid) =
        ClipboardHistoryActiveState::map(cx, |state| (state.active_id, state.item.is_some()));
    if valid && active_id == id {
        return paste_from_clipboard_by_active(cx, shell);
    }
    Ok(())
}

pub fn action_next_clipboard_item(cx: MistyControllerContext, _arg: ()) -> KCResult<()> {
    let cx = cx.handle();
    next_clipboard_active(cx)
}

pub fn action_previous_clipboard_item(cx: MistyControllerContext, _arg: ()) -> KCResult<()> {
    let cx = cx.handle();
    previous_clipboard_active(cx)
}

pub fn action_save_clipboard_item(
    cx: MistyControllerContext,
    (shell, id): (impl IShellHandle, i32),
) -> Result<(), Infallible> {
    let cx = cx.handle();
    let item = ClipboardHistoryListState::map(cx, |state| {
        state
            .list()
            .iter()
            .find(|v| v.id() == id)
            .map(|v| v.clone())
    });

    if item.is_none() {
        return Ok(());
    }
    let item = item.unwrap();

    let file_path = match item {
        ClipboardHistoryItem::Text { .. } => shell.blocking_save_file("clipboard.txt"),
        ClipboardHistoryItem::Image { .. } => shell.blocking_save_file("clipboard.png"),
    };
    if file_path.is_none() {
        return Ok(());
    }
    let file_path = file_path.unwrap();
    let dir_path = file_path.parent();
    if dir_path.is_none() {
        return Ok(());
    }
    let dir_path = dir_path.unwrap();

    {
        let meta = fs::metadata(dir_path);
        if let Ok(meta) = meta {
            if !meta.is_dir() {
                return Ok(());
            }
        } else {
            fs::create_dir_all(dir_path).unwrap();
        }
    }

    match item {
        ClipboardHistoryItem::Text(item) => {
            fs::write(file_path.as_path(), item.text()).unwrap();
        }
        ClipboardHistoryItem::Image(item) => {
            let (width, height) = item.dimensions();
            let bytes = raw_bytes_to_png(item.bytes().to_vec(), width as usize, height as usize);
            fs::write(file_path.as_path(), bytes).unwrap();
        }
    }

    Ok(())
}

pub fn action_remove_clipboard_history_item(
    cx: MistyControllerContext,
    active_id: i32,
) -> KCResult<()> {
    let cx = cx.handle();
    let loc = find_active_clipboard_history_loc(cx);

    batch_remove_clipboard_items(cx, vec![active_id])?;
    ClipboardHistoryListState::update(cx, |state| {
        state.list = state
            .list
            .clone()
            .into_iter()
            .filter(|v| v.id() != active_id)
            .collect();
    });

    if let Some(ActiveLoc {
        len: _len,
        id,
        index,
    }) = loc
    {
        if active_id == id {
            let next_id =
                ClipboardHistoryListState::map(cx, |state| state.list.get(index).map(|v| v.id()));
            if let Some(next_id) = next_id {
                active_clipboard_by_id(cx, next_id);
            }
        }
    }

    Ok(())
}

pub fn action_remove_all_clipboard_history_items(
    cx: MistyControllerContext,
    _arg: (),
) -> KCResult<()> {
    let cx = cx.handle();

    remove_all_clipboard_items(cx)?;
    ClipboardHistoryListState::update(cx, |state| {
        state.list.clear();
    });

    Ok(())
}
