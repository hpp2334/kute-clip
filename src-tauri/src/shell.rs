use crate::misty::CLIENT;
use global_hotkey::hotkey::HotKey;
use kute_clip_core::{libs::common::AppActivator, shell::IShellHandle};
use misty_vm::services::MistyServiceTrait;
use std::path::PathBuf;
use tauri::{Manager, WebviewWindow, WindowEvent};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

#[derive(Clone)]
pub struct ShellHandle {
    tauri_handle: tauri::AppHandle,
}

impl ShellHandle {
    pub fn new(handle: tauri::AppHandle) -> Self {
        Self {
            tauri_handle: handle,
        }
    }
}

impl IShellHandle for ShellHandle {
    fn on_mw_focus_change(&self, on: impl Fn(bool) -> () + Send + Sync + 'static) {
        let win = self.tauri_handle.get_webview_window("main").unwrap();
        win.on_window_event(move |e| match e {
            WindowEvent::Focused(focused) => on(*focused),
            _ => {}
        });
    }

    fn on_shortcut_detect(&self, on: impl Fn(&HotKey) + Send + Sync + 'static) {
        self.tauri_handle
            .plugin(
                tauri_plugin_global_shortcut::Builder::with_handler(move |_app, shortcut| {
                    on(shortcut)
                })
                .build(),
            )
            .unwrap();
    }

    fn register_shortcut(&self, shortcut: HotKey) -> bool {
        let res = self.tauri_handle.global_shortcut().register(shortcut);
        res.is_ok()
    }

    fn unregister_shortcut(&self, shortcut: HotKey) {
        self.tauri_handle
            .global_shortcut()
            .unregister(shortcut)
            .unwrap();
    }

    fn hide(&self) {
        let binding = CLIENT.accessor().get().unwrap();
        let handle = binding.handle();
        AppActivator::of(handle).disable_active();
        self.tauri_handle.hide_win();
    }

    fn show(&self) {
        let binding = CLIENT.accessor().get().unwrap();
        let handle = binding.handle();
        AppActivator::of(handle).enable_active();
        self.tauri_handle.show_win();
    }

    fn mw_visible(&self) -> bool {
        let main_win = self.tauri_handle.get_webview_window("main").unwrap();
        main_win.is_visible().unwrap()
    }

    fn blocking_save_file(&self, file_name: &str) -> Option<PathBuf> {
        let dialog = self
            .tauri_handle
            .dialog()
            .file()
            .add_filter("Any", &["*.*"])
            .set_file_name(file_name);
        let file_path = dialog.blocking_save_file();
        file_path
    }
}

trait WindowVisibilityTrait {
    fn hide_win(&self);
    fn show_win(&self);
}

#[cfg(not(target_os = "macos"))]
impl WindowVisibilityTrait for tauri::AppHandle {
    fn hide_win(&self) {
        let main_win = self.get_webview_window("main").unwrap();
        main_win.hide_win();
    }
    fn show_win(&self) {
        let main_win = self.get_webview_window("main").unwrap();
        main_win.show_win();
    }
}

#[cfg(target_os = "macos")]
impl WindowVisibilityTrait for tauri::AppHandle {
    fn hide_win(&self) {
        self.hide().unwrap();
    }
    fn show_win(&self) {
        self.show().unwrap();

        let main_win = self.get_webview_window("main").unwrap();
        main_win.set_focus().unwrap();
    }
}

impl WindowVisibilityTrait for WebviewWindow {
    fn hide_win(&self) {
        self.hide().unwrap();
    }
    fn show_win(&self) {
        self.show().unwrap();
        self.set_focus().unwrap();
    }
}
