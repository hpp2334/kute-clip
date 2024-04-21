use global_hotkey::hotkey::Code;
use kute_clip_core::{
    actions::{
        clipboard_changed::{
            action_change_clipboard_active, action_paste_clipboard_if_active,
            action_remove_all_clipboard_history_items, action_remove_clipboard_history_item,
        },
        preference::action_change_limit,
    },
    models::clipboard::ClipboardHistoryItemType,
};
use kute_clip_test::{trigger_copy_text_1_to_10, ClipboardItem, FakeAppRef};

fn assert_item_text(app: &FakeAppRef, idx: usize, text: &str) {
    let item = app.state().clipboard_history.unwrap().items[idx].clone();
    assert_eq!(item.typ, ClipboardHistoryItemType::Text);
    assert_eq!(item.text, text.to_string());
}

fn assert_item_image(app: &FakeAppRef, idx: usize, width: i32, height: i32) {
    let item = app.state().clipboard_history.unwrap().items[idx].clone();
    assert_eq!(item.typ, ClipboardHistoryItemType::Image);
    assert_eq!(item.width, width);
    assert_eq!(item.height, height);
}

fn get_active_text(app: &FakeAppRef) -> String {
    app.state()
        .active_clipboard_item
        .unwrap()
        .item
        .unwrap()
        .text
}

#[test]
fn trigger_copy_multi() {
    let app = FakeAppRef::new();
    let _app_dir = app.setup();

    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 0);

    app.trigger_copy(ClipboardItem::Text("1".to_string()));
    app.trigger_copy(ClipboardItem::Text("2".to_string()));
    app.trigger_copy(ClipboardItem::Text("3".to_string()));
    app.trigger_copy(ClipboardItem::Text("4".to_string()));
    app.trigger_copy(ClipboardItem::Text("5".to_string()));

    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 5);
    assert_eq!(
        state.active_clipboard_item.unwrap().item.unwrap().text,
        "5".to_string()
    );

    assert_item_text(&app, 0, "5");
    assert_item_text(&app, 1, "4");
    assert_item_text(&app, 2, "3");
    assert_item_text(&app, 3, "2");
    assert_item_text(&app, 4, "1");

    app.keypress(Code::Enter, None);
    app.wait();

    assert_eq!(app.pasted(), ClipboardItem::Text("5".to_string()));
    assert_eq!(app.mw_visible(), false);

    app.assert_reload_from_app_dir(&_app_dir);
}

#[test]
fn trigger_copy_text_image() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();

    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 0);

    app.trigger_copy(ClipboardItem::Text("1".to_string()));
    app.trigger_copy(ClipboardItem::Image((1, 1, vec![1, 2, 3, 4])));

    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 2);

    assert_item_image(&app, 0, 1, 1);
    assert_item_text(&app, 1, "1");

    app.keypress(Code::Enter, None);
    app.wait();

    assert_eq!(app.pasted(), ClipboardItem::Image((1, 1, vec![1, 2, 3, 4])));

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn apply_history_item_1() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();
    trigger_copy_text_1_to_10(&app);

    let items = app.state().clipboard_history.unwrap().items;

    for item in items {
        app.call_controller(action_change_clipboard_active, item.id);
        app.call_controller(action_paste_clipboard_if_active, (app.shell(), item.id));

        app.wait();
        assert_eq!(app.pasted(), ClipboardItem::Text(item.text));
    }

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn remove_history_item_1() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();
    trigger_copy_text_1_to_10(&app);

    let items = app.state().clipboard_history.unwrap().items;
    // 10 9 8 [7] 6 5 ...
    app.call_controller(action_remove_clipboard_history_item, items[3].id);

    let items = app.state().clipboard_history.unwrap().items;
    assert_eq!(items.len(), 9);
    assert_eq!(items[2].text, "8".to_string());
    assert_eq!(items[3].text, "6".to_string());

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn remove_history_item_2() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();
    trigger_copy_text_1_to_10(&app);

    let items = app.state().clipboard_history.unwrap().items;
    // 10 9 8 [7] 6 5 ...
    app.call_controller(action_change_clipboard_active, items[3].id);
    app.call_controller(action_remove_clipboard_history_item, items[3].id);

    let items = app.state().clipboard_history.unwrap().items;
    assert_eq!(items.len(), 9);
    assert_eq!(items[2].text, "8".to_string());
    assert_eq!(items[3].text, "6".to_string());

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn remove_all_history_items() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();
    trigger_copy_text_1_to_10(&app);

    app.call_controller(action_remove_all_clipboard_history_items, ());

    let items = app.state().clipboard_history.unwrap().items;
    assert_eq!(items.len(), 0);

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn keydown_up_down() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();
    trigger_copy_text_1_to_10(&app);

    app.keypress(Code::ArrowUp, None);
    assert_eq!(get_active_text(&app), "10".to_string());

    app.keypress(Code::ArrowDown, None);
    assert_eq!(get_active_text(&app), "9".to_string());

    app.keypress(Code::Enter, None);
    app.wait();

    assert_eq!(app.pasted(), ClipboardItem::Text("9".to_string()));

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn keydown_digits() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();
    trigger_copy_text_1_to_10(&app);

    let digit_actives = [
        (Code::Digit1, "10".to_string()),
        (Code::Digit2, "9".to_string()),
        (Code::Digit3, "8".to_string()),
        (Code::Digit4, "7".to_string()),
        (Code::Digit5, "6".to_string()),
        (Code::Digit6, "5".to_string()),
    ];

    for (code, text) in digit_actives {
        app.show();
        // active
        app.keypress(code, None);

        assert_eq!(get_active_text(&app), text.clone());

        // paste
        app.keypress(code, None);
        app.wait();

        assert_eq!(app.pasted(), ClipboardItem::Text(text));
    }

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn change_limit_and_copy() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();

    for i in 0..20 {
        app.trigger_copy(ClipboardItem::Text(i.to_string()));
    }
    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 20);

    app.call_controller(action_change_limit, 10);
    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 10);

    app.assert_reload_from_app_dir(&app_dir);
}

#[test]
fn slice_into_invalid_utf8_text() {
    let app = FakeAppRef::new();
    let app_dir = app.setup();

    // '𒀀' is 4 bytes
    let to_copy = "a".to_string() + "𒀀".repeat(400).as_str();
    let view_expected = "a".to_string() + "𒀀".repeat(299).as_str() + "...";
    app.trigger_copy(ClipboardItem::Text(to_copy.clone()));

    let state = app.state();
    assert_eq!(state.clipboard_history.unwrap().items.len(), 1);
    let text = app.state().clipboard_history.unwrap().items[0].clone().text;
    assert_eq!(text, view_expected);

    // paste
    app.keypress(Code::Enter, None);
    app.wait();

    assert_eq!(app.pasted(), ClipboardItem::Text(to_copy));

    app.assert_reload_from_app_dir(&app_dir);
}
