use misty_vm::{client::MistyClientAccessor, misty_service, services::MistyServiceTrait};

use crate::actions::clipboard_changed::{handle_clipboard_changed, OnClipboardActionArg};

pub trait IClipboard: Send + Sync + 'static {
    fn text(&self) -> Option<String>;
    fn image(&self) -> Option<(usize, usize, Vec<u8>)>;
    fn write_text(&self, text: String);
    fn write_image(&self, width: usize, height: usize, bytes: &[u8]);
}

pub struct ClipboardHandler {
    accessor: MistyClientAccessor,
}

pub trait IClipboardLoop: Send + Sync + 'static {
    fn run(&self, handler: ClipboardHandler);
}

misty_service!(Clipboard, IClipboard);
misty_service!(ClipboardLoop, IClipboardLoop);

impl ClipboardHandler {
    pub fn on_change(&self) -> bool {
        let handle = self.accessor.get();
        if handle.is_none() {
            return false;
        }
        let handle = handle.unwrap();
        let handle = handle.handle();

        if let Some(text) = Clipboard::of(handle).text() {
            handle.schedule(|handle| {
                handle_clipboard_changed(handle, OnClipboardActionArg::Text(text))
            });
        }
        if let Some((width, height, bytes)) = Clipboard::of(handle).image() {
            handle.schedule(move |handle| {
                handle_clipboard_changed(
                    handle,
                    OnClipboardActionArg::Image {
                        width,
                        height,
                        bytes,
                    },
                )
            });
        }
        return true;
    }
}

pub fn start_clipboard_loop(accessor: MistyClientAccessor) {
    std::thread::spawn(move || {
        let handle = accessor.get().unwrap();
        let handle = handle.handle();

        let handler = ClipboardHandler { accessor };
        ClipboardLoop::of(handle).run(handler);
    });
}
