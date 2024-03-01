use global_hotkey::hotkey::{Code, Modifiers};
use kute_clip_core::actions::preference::action_change_hide_app_losing_focus;
use kute_clip_test::{FakeAppRef};

#[test]
fn trigger_escape() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();

    app.keypress(Code::Escape, None);
    assert_eq!(app.mw_visible(), false);

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn trigger_shortcut() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();

    let keypress_default = || {
        app.keypress(Code::KeyV, Some(Modifiers::CONTROL | Modifiers::SHIFT));
    };

    keypress_default();
    assert_eq!(app.mw_visible(), false);

    keypress_default();
    assert_eq!(app.mw_visible(), true);

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn on_lose_focus_1() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();

    app.change_focused(false);
    assert_eq!(app.mw_visible(), false);

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn on_lose_focus_2() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();

    app.call_controller(action_change_hide_app_losing_focus, false);

    app.change_focused(false);
    assert_eq!(app.mw_visible(), true);

    app.assert_reload_from_app_dir(&app_dir);
}
