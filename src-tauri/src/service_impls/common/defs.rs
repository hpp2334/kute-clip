pub struct PlatformManager;

pub trait PlatformManagerImpl {
    fn disable_active_current_app();
    fn enable_active_current_app();
    fn paste_clipboard();
}
