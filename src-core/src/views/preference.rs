use serde::Serialize;

use crate::{actions::preference::PreferenceState, models::preference::PreferenceData};

use super::RootView;

#[derive(Debug, Serialize, Clone)]
pub struct VPreference {
    pub data: PreferenceData,
    pub shortcut_success: bool,
}

pub fn preference_view(state: &PreferenceState, root: &mut RootView) {
    let data = state.data().clone();
    let shortcut_success: bool = *state.shortcut_success();
    root.preference = Some(VPreference {
        data,
        shortcut_success,
    });
}
