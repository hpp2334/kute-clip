use std::convert::Infallible;

use crate::{
    models::preference::{load_preference, PreferenceData, ShortcutInfo},
    result::KCResult,
    shell::IShellHandle,
};
use getset::Getters;
use global_hotkey::hotkey::HotKey;
use misty_vm::{
    client::{AsReadonlyMistyClientHandle, MistyClientHandle, MistyReadonlyClientHandle},
    controllers::MistyControllerContext,
    states::MistyStateTrait,
    MistyState,
};

use super::clipboard_changed::retain_ncount_clipboard_histories;

#[derive(Debug, Default, MistyState, Getters)]
pub struct PreferenceState {
    #[getset(get = "pub")]
    data: PreferenceData,
    #[getset(get = "pub")]
    shortcut_success: bool,
}

pub fn init_preference(cx: MistyClientHandle, shell: impl IShellHandle) -> KCResult<()> {
    let preference = load_preference(cx.readonly_handle())?;

    PreferenceState::update(cx, |state| {
        state.data = preference;
        state.shortcut_success = false;
    });
    register_current_shortcut(cx, shell).unwrap();

    Ok(())
}

pub fn get_registered_shortcut(cx: MistyReadonlyClientHandle) -> Option<HotKey> {
    let (success, shortcut) = PreferenceState::map(cx, |state| {
        (state.shortcut_success, state.data.shortcut.shortcut())
    });

    if success {
        Some(shortcut)
    } else {
        None
    }
}

pub fn get_hide_app_when_losing_focus(cx: MistyReadonlyClientHandle) -> bool {
    PreferenceState::map(cx, |state| state.data.hide_app_when_losing_focus)
}

pub fn get_limit_count(cx: MistyReadonlyClientHandle) -> usize {
    PreferenceState::map(cx, |state| state.data.limit)
}

fn unregister_current_shortcut(cx: MistyClientHandle, shell: impl IShellHandle) {
    let shortcut = PreferenceState::map(cx, |state| state.data.shortcut.shortcut());
    shell.unregister_shortcut(shortcut);
}

pub fn register_current_shortcut(
    cx: MistyClientHandle,
    shell: impl IShellHandle,
) -> Result<(), Infallible> {
    let shortcut = PreferenceState::map(cx, |state| state.data.shortcut.shortcut());

    if shell.register_shortcut(shortcut) {
        PreferenceState::update(cx, |state| {
            state.shortcut_success = true;
        });
    } else {
        PreferenceState::update(cx, |state| {
            state.shortcut_success = false;
        });
    }
    Ok(())
}

pub fn action_change_shortcut(
    cx: MistyControllerContext,
    (shell, arg): (impl IShellHandle, ShortcutInfo),
) -> Result<(), Infallible> {
    let cx = cx.handle();
    unregister_current_shortcut(cx, shell.clone());
    PreferenceState::update(cx, |state| {
        state.data.shortcut = arg;
        state.shortcut_success = false;
    });
    register_current_shortcut(cx, shell)
}

pub fn action_change_limit(cx: MistyControllerContext, limit: usize) -> KCResult<()> {
    let cx = cx.handle();
    let limit = limit.clamp(1, 999);
    PreferenceState::update(cx, |state: &mut PreferenceState| {
        state.data.limit = limit;
    });
    retain_ncount_clipboard_histories(cx, limit)?;
    Ok(())
}

pub fn action_change_hide_app_losing_focus(
    cx: MistyControllerContext,
    value: bool,
) -> Result<(), Infallible> {
    let cx = cx.handle();

    PreferenceState::update(cx, |state: &mut PreferenceState| {
        state.data.hide_app_when_losing_focus = value;
    });
    Ok(())
}
