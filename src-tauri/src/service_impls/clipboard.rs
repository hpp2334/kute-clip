use std::borrow::Cow;

use kute_clip_core::app_setup::{ClipboardHandler, IClipboard, IClipboardLoop};

pub struct ClipboardImpl;
pub struct ClipboardLoopImpl;

struct ClipboardHandlerWrapper(ClipboardHandler);
impl AsRef<ClipboardHandler> for ClipboardHandlerWrapper {
    fn as_ref(&self) -> &ClipboardHandler {
        &self.0
    }
}

impl clipboard_master::ClipboardHandler for ClipboardHandlerWrapper {
    fn on_clipboard_change(&mut self) -> clipboard_master::CallbackResult {
        if self.as_ref().on_change() {
            clipboard_master::CallbackResult::Next
        } else {
            clipboard_master::CallbackResult::Stop
        }
    }
}

impl IClipboard for ClipboardImpl {
    fn text(&self) -> Option<String> {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        if let Ok(text) = clipboard.get_text() {
            Some(text)
        } else {
            None
        }
    }
    fn image(&self) -> Option<(usize, usize, Vec<u8>)> {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        if let Ok(data) = clipboard.get_image() {
            let width = data.width;
            let height = data.height;
            let bytes = data.bytes.to_vec();
            Some((width, height, bytes))
        } else {
            None
        }
    }
    fn write_text(&self, text: String) {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        clipboard.set_text(text).unwrap();
    }
    fn write_image(&self, width: usize, height: usize, bytes: &[u8]) {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        clipboard
            .set_image(arboard::ImageData {
                width,
                height,
                bytes: Cow::Borrowed(&bytes),
            })
            .unwrap();
    }
}

impl IClipboardLoop for ClipboardLoopImpl {
    fn run(&self, handler: ClipboardHandler) {
        let _ = clipboard_master::Master::new(ClipboardHandlerWrapper(handler))
            .run()
            .unwrap();
    }
}
