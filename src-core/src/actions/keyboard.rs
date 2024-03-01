use keyboard_types::Code;
use misty_vm::controllers::MistyControllerContext;

use crate::{result::KCResult, shell::IShellHandle};

use super::clipboard_changed::{
    active_or_paste_from_clipboard_by_index, next_clipboard_active, paste_from_clipboard_by_active,
    previous_clipboard_active,
};

pub fn action_keydown(
    cx: MistyControllerContext,
    (shell, key): (impl IShellHandle, Code),
) -> KCResult<()> {
    let cx = cx.handle();
    match key {
        Code::ArrowDown => next_clipboard_active(cx),
        Code::ArrowUp => previous_clipboard_active(cx),
        Code::Enter => paste_from_clipboard_by_active(cx, shell),
        Code::Escape => {
            shell.hide();
            Ok(())
        }
        Code::Digit1 | Code::Numpad1 => active_or_paste_from_clipboard_by_index(cx, shell, 0),
        Code::Digit2 | Code::Numpad2 => active_or_paste_from_clipboard_by_index(cx, shell, 1),
        Code::Digit3 | Code::Numpad3 => active_or_paste_from_clipboard_by_index(cx, shell, 2),
        Code::Digit4 | Code::Numpad4 => active_or_paste_from_clipboard_by_index(cx, shell, 3),
        Code::Digit5 | Code::Numpad5 => active_or_paste_from_clipboard_by_index(cx, shell, 4),
        Code::Digit6 | Code::Numpad6 => active_or_paste_from_clipboard_by_index(cx, shell, 5),
        _ => Ok(()),
    }
}
