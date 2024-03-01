use misty_vm::misty_service;

pub trait IClipboardPaster: Send + Sync + 'static {
    fn paste(&self);
}

pub trait IAppActivator: Send + Sync + 'static {
    fn enable_active(&self);
    fn disable_active(&self);
}

misty_service!(ClipboardPaster, IClipboardPaster);
misty_service!(AppActivator, IAppActivator);

pub struct AppActivatorImpl;
