use misty_vm::client::MistyClientAccessor;


use crate::{actions::{preference::get_hide_app_when_losing_focus}, shell::IShellHandle};

pub fn setup_on_window_events(shell: impl IShellHandle, accessor: MistyClientAccessor) {
    shell.clone().on_mw_focus_change(move |focused| {
        let client = accessor.get();
        if client.is_none() {
            return;
        }
        let client = client.unwrap();
        
        let hide_app_when_losing_focus = get_hide_app_when_losing_focus(client.handle());
        if !focused && hide_app_when_losing_focus {
            shell.hide();
        }
    });
}
