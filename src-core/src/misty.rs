use misty_vm::{misty_states, states::MistyStateManager};

use crate::actions::{
    clipboard_changed::{ClipboardHistoryActiveState, ClipboardHistoryListState},
    preference::PreferenceState,
};

pub fn build_state_manager() -> MistyStateManager {
    MistyStateManager::new(misty_states!(
        ClipboardHistoryListState,
        ClipboardHistoryActiveState,
        PreferenceState,
    ))
}
