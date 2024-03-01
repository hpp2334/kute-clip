use enigo::KeyboardControllable;

use super::{PlatformManager, PlatformManagerImpl};

impl PlatformManagerImpl for PlatformManager {
    fn disable_active_current_app() {
        // noop
    }

    fn enable_active_current_app() {
        // noop
    }

    fn paste_clipboard() {
        let mut enigo = enigo::Enigo::new();
        enigo.key_sequence_parse("{+CTRL}v{-CTRL}");
    }
}
