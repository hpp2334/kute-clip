use kute_clip_core::models::{clipboard::ClipboardHistoryItemType, preference::ShortcutInfo};
use kute_clip_test::{ClipboardItem, FakeAppRef};

#[test]
fn init_application() {
    let app = FakeAppRef::new();
    let _app_dir = app.setup();

    assert_eq!(app.mw_visible(), true);

    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 0);

    let preference = state.preference.unwrap();
    assert_eq!(preference.shortcut_success, true);
    assert_eq!(preference.data.hide_app_when_losing_focus, true);
    assert_eq!(preference.data.send_notification_when_copy, false);
    assert_eq!(preference.data.limit, 100);
    assert_eq!(
        preference.data.shortcut,
        ShortcutInfo {
            ctrl: true,
            meta: false,
            alt: false,
            shift: true,
            code: global_hotkey::hotkey::Code::KeyV
        }
    );
}

#[test]
fn trigger_copy() {
    let app = FakeAppRef::new();
    let _app_dir = app.setup();

    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 0);

    app.trigger_copy(ClipboardItem::Text("ABC".to_string()));

    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 1);
    {
        let item = app.state().clipboard_history.unwrap().items[0].clone();
        assert_eq!(item.typ, ClipboardHistoryItemType::Text);
        assert_eq!(item.text, "ABC".to_string());
    }

    app.assert_reload_from_app_dir(&_app_dir);
}
