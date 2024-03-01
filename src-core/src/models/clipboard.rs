use misty_vm::{
    client::{AsReadonlyMistyClientHandle, MistyReadonlyClientHandle},
    resources::MistyResourceHandle,
};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{repositories::get_conn, result::KCResult, utils::raw_bytes_to_base64_png};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ClipboardHistoryItemType {
    #[default]
    Text,
    Image,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardHistoryItemModel {
    pub id: i32,
    pub typ: ClipboardHistoryItemType,
    pub text: String,
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ClipboardHistoryItemText {
    model: ClipboardHistoryItemModel,
}

#[derive(Debug, Clone)]
pub struct ClipboardHistoryItemImage {
    model: ClipboardHistoryItemModel,
    png_resource: MistyResourceHandle,
}

#[derive(Debug, Clone)]
pub enum ClipboardHistoryItem {
    Text(ClipboardHistoryItemText),
    Image(ClipboardHistoryItemImage),
}

impl ClipboardHistoryItem {
    pub fn id(&self) -> i32 {
        match &self {
            &ClipboardHistoryItem::Text(v) => v.model.id,
            &ClipboardHistoryItem::Image(v) => v.model.id,
        }
    }
}

impl ClipboardHistoryItemText {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            model: ClipboardHistoryItemModel {
                id,
                typ: ClipboardHistoryItemType::Text,
                text,
                width: 0,
                height: 0,
                bytes: Default::default(),
            },
        }
    }

    pub fn text(&self) -> &str {
        self.model.text.as_str()
    }
}

impl ClipboardHistoryItemImage {
    pub fn new<'a>(
        id: i32,
        width: i32,
        height: i32,
        bytes: Vec<u8>,
        png_resource: MistyResourceHandle,
    ) -> Self {
        Self {
            model: ClipboardHistoryItemModel {
                id,
                typ: ClipboardHistoryItemType::Image,
                text: Default::default(),
                width,
                height,
                bytes,
            },
            png_resource,
        }
    }

    pub fn dimensions(&self) -> (i32, i32) {
        (self.model.width, self.model.height)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.model.bytes
    }

    pub fn base64_png_resource(&self) -> &MistyResourceHandle {
        &self.png_resource
    }
}

fn save_clipboard_item(
    cx: MistyReadonlyClientHandle,
    item: ClipboardHistoryItemModel,
) -> KCResult<()> {
    let conn = get_conn(cx)?;
    conn.execute(
        "INSERT INTO clipboard_history (id, typ, text, width, height, bytes) VALUES (?,?,?,?,?,?)",
        params![
            item.id,
            item.typ as i32,
            item.text,
            item.width,
            item.height,
            item.bytes
        ],
    )?;

    Ok(())
}

pub fn load_clipboard_histories(
    cx: MistyReadonlyClientHandle,
) -> KCResult<Vec<ClipboardHistoryItem>> {
    let conn = get_conn(cx)?;
    let mut list =
        conn.query::<ClipboardHistoryItemModel>("SELECT * FROM clipboard_history", params![])?;

    list.sort_by(|lhs, rhs| rhs.id.cmp(&lhs.id));

    let list = list
        .into_iter()
        .map(|item| {
            let typ = item.typ;
            match typ {
                ClipboardHistoryItemType::Text => {
                    ClipboardHistoryItem::Text(ClipboardHistoryItemText { model: item })
                }
                ClipboardHistoryItemType::Image => {
                    let resource = cx.resource_manager().insert(raw_bytes_to_base64_png(
                        item.bytes.clone(),
                        item.width as usize,
                        item.height as usize,
                    ));
                    ClipboardHistoryItem::Image(ClipboardHistoryItemImage {
                        model: item,
                        png_resource: resource,
                    })
                }
            }
        })
        .collect();

    Ok(list)
}

pub fn batch_remove_clipboard_items<'a>(
    cx: impl AsReadonlyMistyClientHandle<'a>,
    ids: Vec<i32>,
) -> KCResult<()> {
    // TODO: WHERE IN
    let conn = get_conn(cx)?;

    for id in ids {
        conn.execute("DELETE FROM clipboard_history where id = ?", params![id,])?;
    }
    Ok(())
}

pub fn remove_all_clipboard_items<'a>(cx: impl AsReadonlyMistyClientHandle<'a>) -> KCResult<()> {
    let conn = get_conn(cx)?;
    conn.execute("DELETE FROM clipboard_history", [])?;
    Ok(())
}

pub fn save_clipboard_item_text(
    cx: MistyReadonlyClientHandle,
    item: ClipboardHistoryItemText,
) -> KCResult<()> {
    save_clipboard_item(cx, item.model)
}

pub fn save_clipboard_item_image(
    cx: MistyReadonlyClientHandle,
    item: ClipboardHistoryItemImage,
) -> KCResult<()> {
    save_clipboard_item(cx, item.model)
}
