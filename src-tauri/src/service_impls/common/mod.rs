mod defs;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(not(target_os = "macos"))]
mod other;

use defs::{PlatformManager, PlatformManagerImpl};
use kute_clip_core::libs::common::{IAppActivator, IClipboardPaster};

pub struct ClipboardPasterImpl;

impl IClipboardPaster for ClipboardPasterImpl {
    fn paste(&self) {
        PlatformManager::paste_clipboard();
    }
}

pub struct AppActivatorImpl;

impl IAppActivator for AppActivatorImpl {
    fn disable_active(&self) {
        PlatformManager::disable_active_current_app();
    }

    fn enable_active(&self) {
        PlatformManager::enable_active_current_app();
    }
}
