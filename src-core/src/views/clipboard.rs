use serde::Serialize;

use crate::{
    actions::clipboard_changed::{ClipboardHistoryActiveState, ClipboardHistoryListState},
    models::clipboard::{ClipboardHistoryItem, ClipboardHistoryItemType},
};

use super::RootView;

#[derive(Debug, Serialize, Clone)]
pub struct VClipboardHistoryItem {
    pub id: i32,
    pub typ: ClipboardHistoryItemType,
    pub text: String,
    pub width: i32,
    pub height: i32,
    pub resource: u64,
    pub shortcut: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct VClipboardHistoryDetailItem {
    pub id: i32,
    pub typ: ClipboardHistoryItemType,
    pub text: String,
    pub width: i32,
    pub height: i32,
    pub resource: u64,
    pub chars_len: usize,
    pub resource_byte_size: String,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct VClipboardHistory {
    pub items: Vec<VClipboardHistoryItem>,
}

#[derive(Debug, Serialize, Clone)]
pub struct VClipboardActiveHistoryItem {
    pub id: i32,
    pub item: Option<VClipboardHistoryDetailItem>,
}

fn index_to_shortcut(index: usize) -> String {
    match index {
        0 => "1".to_string(),
        1 => "2".to_string(),
        2 => "3".to_string(),
        3 => "4".to_string(),
        4 => "5".to_string(),
        5 => "6".to_string(),
        _ => "-".to_string(),
    }
}

fn fmt_resource_byte_size(byte_size: usize) -> String {
    if byte_size < (1 << 10) {
        return format!("{}B", byte_size);
    }
    if byte_size < (1 << 20) {
        return format!("{:.2}KB", byte_size as f64 / (1 << 10) as f64);
    }
    if byte_size < (1 << 30) {
        return format!("{:.2}MB", byte_size as f64 / (1 << 20) as f64);
    }
    return format!("{:.2}GB", byte_size as f64 / (1 << 30) as f64);
}

fn build_v_clipboard_history_item(
    item: ClipboardHistoryItem,
    index: usize,
) -> VClipboardHistoryItem {
    let id = item.id();
    match item {
        ClipboardHistoryItem::Text(item) => {
            let text = item.text();
            VClipboardHistoryItem {
                id,
                typ: ClipboardHistoryItemType::Text,
                // For list item, limit string length to 300
                text: if text.len() > 300 {
                    text[0..300].to_string() + "..."
                } else {
                    text.to_string()
                },
                width: 0,
                height: 0,
                resource: 0,
                shortcut: index_to_shortcut(index),
            }
        }
        ClipboardHistoryItem::Image(item) => {
            let (width, height) = item.dimensions();
            VClipboardHistoryItem {
                id,
                typ: ClipboardHistoryItemType::Image,
                text: Default::default(),
                width,
                height,
                resource: *item.base64_png_resource().id(),
                shortcut: index_to_shortcut(index),
            }
        }
    }
}

fn build_v_clipboard_history_detail_item(
    item: ClipboardHistoryItem,
) -> VClipboardHistoryDetailItem {
    let id = item.id();
    match item {
        ClipboardHistoryItem::Text(item) => {
            let text = item.text();
            let chars_len = text.len();
            VClipboardHistoryDetailItem {
                id,
                typ: ClipboardHistoryItemType::Text,
                // For detail item, limit string length to 3000 due to performance
                text: if chars_len > 3000 {
                    text[0..3000].to_string() + "..."
                } else {
                    text.to_string()
                },
                width: 0,
                height: 0,
                resource: 0,
                chars_len,
                resource_byte_size: "".to_string(),
            }
        }
        ClipboardHistoryItem::Image(item) => {
            let (width, height) = item.dimensions();
            VClipboardHistoryDetailItem {
                id,
                typ: ClipboardHistoryItemType::Image,
                text: Default::default(),
                width,
                height,
                resource: *item.base64_png_resource().id(),
                chars_len: 0,
                resource_byte_size: fmt_resource_byte_size(item.bytes().len()),
            }
        }
    }
}

pub fn clipboard_history_view(state: &ClipboardHistoryListState, root: &mut RootView) {
    let view_items = state
        .list()
        .iter()
        .enumerate()
        .map(|(index, item)| build_v_clipboard_history_item(item.clone(), index))
        .collect();
    let view = VClipboardHistory { items: view_items };

    root.clipboard_history = Some(view);
}

pub fn clipboard_active_item_view(state: &ClipboardHistoryActiveState, root: &mut RootView) {
    let active_id = *state.active_id();
    let item = match state.item() {
        Some(item) => {
            let item = build_v_clipboard_history_detail_item(item.clone());

            Some(item)
        }
        None => None,
    };
    let item = VClipboardActiveHistoryItem {
        id: active_id,
        item,
    };

    root.active_clipboard_item = Some(item);
}
