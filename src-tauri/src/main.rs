// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::PathBuf, sync::atomic::AtomicBool};

use directories::UserDirs;
use kute_clip_core::{
    actions::{
        application::action_init_application,
        clipboard_changed::{
            action_change_clipboard_active, action_next_clipboard_item,
            action_paste_clipboard_if_active, action_previous_clipboard_item,
            action_remove_all_clipboard_history_items, action_remove_clipboard_history_item,
            action_save_clipboard_item,
        },
        keyboard::action_keydown,
        preference::{
            action_change_hide_app_losing_focus, action_change_limit, action_change_shortcut,
        },
    },
    app_setup::app_setup,
    models::preference::ShortcutInfo,
};
use misty::{apply_ret, setup_misty_client, CommandRet, CLIENT};
use shell::ShellHandle;
use tauri::Manager;
use tauri_plugin_global_shortcut::Code;
use tray::setup_tray;

mod misty;
mod service_impls;
mod shell;
mod tray;

fn get_app_dir() -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    let document_dir = user_dirs.document_dir().unwrap();
    let app_document_dir = document_dir.join("kute-clip");

    let meta = std::fs::metadata(&app_document_dir);

    if meta.is_err() || !meta.unwrap().is_dir() {
        std::fs::create_dir(&app_document_dir).unwrap();
    }

    app_document_dir
}

fn setup_subscriber() {
    static SETUP_SUBSCRIBER_ONCE: AtomicBool = AtomicBool::new(false);
    let has_setup = SETUP_SUBSCRIBER_ONCE.swap(true, std::sync::atomic::Ordering::SeqCst);
    if has_setup {
        return;
    }

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tauri::command]
fn flush_schedule() -> CommandRet {
    apply_ret(CLIENT.flush_scheduled_tasks())
}

#[tauri::command]
fn init_application(app: tauri::AppHandle) -> CommandRet {
    apply_ret(CLIENT.call_controller(
        action_init_application,
        (ShellHandle::new(app), get_app_dir()),
    ))
}

#[tauri::command]
fn change_clipboard_active(id: i32) -> CommandRet {
    apply_ret(CLIENT.call_controller(action_change_clipboard_active, id))
}

#[tauri::command]
fn handle_keydown(app: tauri::AppHandle, key: Code) -> CommandRet {
    apply_ret(CLIENT.call_controller(action_keydown, (ShellHandle::new(app), key)))
}

#[tauri::command]
fn paste_clipboard_if_active(app: tauri::AppHandle, id: i32) -> CommandRet {
    apply_ret(CLIENT.call_controller(
        action_paste_clipboard_if_active,
        (ShellHandle::new(app), id),
    ))
}

#[tauri::command]
fn next_clipboard_item() -> CommandRet {
    apply_ret(CLIENT.call_controller(action_next_clipboard_item, ()))
}

#[tauri::command]
fn previous_clipboard_item() -> CommandRet {
    apply_ret(CLIENT.call_controller(action_previous_clipboard_item, ()))
}

#[tauri::command]
fn save_clipboard_item(app: tauri::AppHandle, id: i32) -> CommandRet {
    apply_ret(CLIENT.call_controller(action_save_clipboard_item, (ShellHandle::new(app), id)))
}

#[tauri::command]
fn change_shortcut(app: tauri::AppHandle, info: ShortcutInfo) -> CommandRet {
    apply_ret(CLIENT.call_controller(action_change_shortcut, (ShellHandle::new(app), info)))
}

#[tauri::command]
fn change_limit(limit: usize) -> CommandRet {
    apply_ret(CLIENT.call_controller(action_change_limit, limit))
}

#[tauri::command]
fn change_hide_app_losing_focus(value: bool) -> CommandRet {
    apply_ret(CLIENT.call_controller(action_change_hide_app_losing_focus, value))
}

#[tauri::command]
fn remove_clipboard_history_item(id: i32) -> CommandRet {
    apply_ret(CLIENT.call_controller(action_remove_clipboard_history_item, id))
}

#[tauri::command]
fn remove_all_clipboard_history_items() -> CommandRet {
    apply_ret(CLIENT.call_controller(action_remove_all_clipboard_history_items, ()))
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    setup_subscriber();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}))
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            let shell = ShellHandle::new(app.handle().clone());
            setup_tray(app, shell.clone());
            setup_misty_client(app.app_handle().clone());
            app_setup(shell.clone(), CLIENT.accessor())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init_application,
            change_clipboard_active,
            flush_schedule,
            handle_keydown,
            paste_clipboard_if_active,
            next_clipboard_item,
            previous_clipboard_item,
            save_clipboard_item,
            change_shortcut,
            change_limit,
            change_hide_app_losing_focus,
            remove_clipboard_history_item,
            remove_all_clipboard_history_items,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
