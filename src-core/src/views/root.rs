use misty_vm::views::MistyViewModelManager;
use serde::Serialize;

use crate::views::clipboard::VClipboardHistory;

use super::{clipboard::{
    clipboard_active_item_view, clipboard_history_view, VClipboardActiveHistoryItem,
}, preference::{preference_view, VPreference}};

#[derive(Default, Serialize, Clone)]
pub struct RootView {
    pub clipboard_history: Option<VClipboardHistory>,
    pub active_clipboard_item: Option<VClipboardActiveHistoryItem>,
    pub preference: Option<VPreference>
}

pub fn build_view_manager() -> MistyViewModelManager<RootView> {
    MistyViewModelManager::builder()
        .register(clipboard_history_view)
        .register(clipboard_active_item_view)
        .register(preference_view)
        .build()
}
