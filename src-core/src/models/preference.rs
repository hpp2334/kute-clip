use crate::result::KCResult;
use global_hotkey::hotkey::HotKey;
use keyboard_types::{Code, Modifiers};
use misty_vm::client::MistyReadonlyClientHandle;
use serde::{Deserialize, Serialize};

use super::application::load_application_meta;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenceData {
    pub shortcut: ShortcutInfo,
    pub limit: usize,
    pub hide_app_when_losing_focus: bool,
    pub send_notification_when_copy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutInfo {
    pub ctrl: bool,
    pub meta: bool,
    pub alt: bool,
    pub shift: bool,
    pub code: Code,
}

#[cfg(not(target_os = "macos"))]
fn default_shortcut_info() -> ShortcutInfo {
    ShortcutInfo {
        ctrl: true,
        meta: false,
        alt: false,
        shift: true,
        code: Code::KeyV,
    }
}

#[cfg(target_os = "macos")]
fn default_shortcut_info() -> ShortcutInfo {
    ShortcutInfo {
        ctrl: false,
        meta: true,
        alt: false,
        shift: true,
        code: Code::KeyV,
    }
}

impl ShortcutInfo {
    pub fn shortcut(&self) -> HotKey {
        let modifier = {
            let mut m: Option<Modifiers> = None;
            if self.ctrl {
                m = if let Some(m) = m {
                    Some(m | Modifiers::CONTROL)
                } else {
                    Some(Modifiers::CONTROL)
                };
            }
            if self.meta {
                m = if let Some(m) = m {
                    Some(m | Modifiers::META)
                } else {
                    Some(Modifiers::META)
                };
            }
            if self.alt {
                m = if let Some(m) = m {
                    Some(m | Modifiers::ALT)
                } else {
                    Some(Modifiers::ALT)
                };
            }
            if self.shift {
                m = if let Some(m) = m {
                    Some(m | Modifiers::SHIFT)
                } else {
                    Some(Modifiers::SHIFT)
                };
            }

            m
        };
        HotKey::new(modifier, self.code)
    }
}

impl Default for PreferenceData {
    fn default() -> Self {
        Self {
            shortcut: default_shortcut_info(),
            limit: 100,
            hide_app_when_losing_focus: true,
            send_notification_when_copy: false,
        }
    }
}

pub fn load_preference(cx: MistyReadonlyClientHandle) -> KCResult<PreferenceData> {
    let meta = load_application_meta(cx)?;

    let mut preference =
        serde_json::from_str::<PreferenceData>(&meta.preference).unwrap_or_default();

    preference.limit = preference.limit.clamp(10, 999);

    Ok(preference)
}
