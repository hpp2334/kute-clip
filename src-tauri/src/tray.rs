use kute_clip_core::shell::IShellHandle;
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};

pub fn setup_tray(app: &mut tauri::App, shell: impl IShellHandle) {
    let item_vis = MenuItemBuilder::new("Show/Hide")
        .id("VIS")
        .enabled(true)
        .build(app)
        .unwrap();
    let item_quit = MenuItemBuilder::new("Exit")
        .id("EXIT")
        .enabled(true)
        .build(app)
        .unwrap();
    let menu = MenuBuilder::new(app)
        .item(&item_vis)
        .item(&item_quit)
        .build()
        .unwrap();

    let icon_data = include_bytes!("../icons/icon.ico");
    let icon = Image::from_bytes(icon_data).unwrap();

    let _tray_icon = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .tooltip("KuteClip")
        .on_menu_event(move |app, ev| {
            let main_win = app.get_webview_window("main").unwrap();
            let id = ev.id();
            match id.0.as_str() {
                "VIS" => {
                    if main_win.is_visible().unwrap() {
                        shell.hide();
                    } else {
                        shell.show();
                    }
                }
                "EXIT" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .build(app)
        .unwrap();
}
