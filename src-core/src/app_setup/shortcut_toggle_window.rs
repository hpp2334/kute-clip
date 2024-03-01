use misty_vm::client::MistyClientAccessor;

use crate::{actions::preference::get_registered_shortcut, shell::IShellHandle};

pub fn setup_shortcut_toggle_window(shell: impl IShellHandle, accessor: MistyClientAccessor) {
    shell.clone().on_shortcut_detect(move |shortcut| {
        let client = accessor.get();
        if client.is_none() {
            return;
        }
        let client = client.unwrap();
        let registered_shortcut = get_registered_shortcut(client.handle());
        if registered_shortcut.is_none() {
            return;
        }
        let registered_shortcut = registered_shortcut.unwrap();

        if shortcut == &registered_shortcut {
            if shell.mw_visible() {
                shell.hide();
            } else {
                shell.show();
            }
        }
    });
}
