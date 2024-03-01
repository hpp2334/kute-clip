use std::path::PathBuf;

use global_hotkey::hotkey::HotKey;

pub trait IShellHandle: Send + Sync + Clone + 'static {
    fn on_mw_focus_change(&self, on: impl Fn(bool) + Send + Sync + 'static);
    fn on_shortcut_detect(&self, on: impl Fn(&HotKey) + Send + Sync + 'static);
    fn register_shortcut(&self, shortcut: HotKey) -> bool;
    fn unregister_shortcut(&self, shortcut: HotKey);
    fn show(&self);
    fn hide(&self);
    fn mw_visible(&self) -> bool;
    fn blocking_save_file(&self, file_name: &str) -> Option<PathBuf>;
}
