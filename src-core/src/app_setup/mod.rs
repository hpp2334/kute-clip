use misty_vm::client::MistyClientAccessor;

use crate::{result::KCResult, shell::IShellHandle};

pub use self::clipboard_listener_loop::start_clipboard_loop;
use self::{
    handle_window_events::setup_on_window_events,
    shortcut_toggle_window::setup_shortcut_toggle_window,
};

mod clipboard_listener_loop;
mod handle_window_events;
mod shortcut_toggle_window;

pub use clipboard_listener_loop::{
    Clipboard, ClipboardHandler, ClipboardLoop, IClipboard, IClipboardLoop,
};

pub fn app_setup(shell: impl IShellHandle, accessor: MistyClientAccessor) -> KCResult<()> {
    setup_on_window_events(shell.clone(), accessor.clone());
    setup_shortcut_toggle_window(shell.clone(), accessor.clone());
    start_clipboard_loop(accessor);
    Ok(())
}
