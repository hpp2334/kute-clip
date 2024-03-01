// The following code is from enigo (https://github.com/enigo-rs/enigo)

use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

use core_graphics::display::CFIndex;
use core_graphics::event::CGKeyCode;

// required for NSEvent
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

type CFDataRef = *const c_void;

#[repr(C)]
#[derive(Clone, Copy)]
struct NSPoint {
    x: f64,
    y: f64,
}

#[repr(C)]
struct __TISInputSource;
type TISInputSourceRef = *const __TISInputSource;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct __CFString([u8; 0]);
type CFStringRef = *const __CFString;
type Boolean = c_uchar;
type UInt8 = c_uchar;
type SInt32 = c_int;
type UInt16 = c_ushort;
type UInt32 = c_uint;
type UniChar = UInt16;
type UniCharCount = c_ulong;

type OptionBits = UInt32;
type OSStatus = SInt32;

type CFStringEncoding = UInt32;

const TRUE: c_uint = 1;

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct UCKeyboardTypeHeader {
    keyboardTypeFirst: UInt32,
    keyboardTypeLast: UInt32,
    keyModifiersToTableNumOffset: UInt32,
    keyToCharTableIndexOffset: UInt32,
    keyStateRecordsIndexOffset: UInt32,
    keyStateTerminatorsOffset: UInt32,
    keySequenceDataIndexOffset: UInt32,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct UCKeyboardLayout {
    keyLayoutHeaderFormat: UInt16,
    keyLayoutDataVersion: UInt16,
    keyLayoutFeatureInfoOffset: UInt32,
    keyboardTypeCount: UInt32,
    keyboardTypeList: [UCKeyboardTypeHeader; 1usize],
}

#[allow(non_upper_case_globals)]
const kUCKeyTranslateNoDeadKeysBit: _bindgen_ty_703 = _bindgen_ty_703::kUCKeyTranslateNoDeadKeysBit;

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum _bindgen_ty_703 {
    kUCKeyTranslateNoDeadKeysBit = 0,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct __CFAllocator([u8; 0]);
type CFAllocatorRef = *const __CFAllocator;

#[allow(non_upper_case_globals)]
const kCFStringEncodingUTF8: u32 = 134_217_984;

#[allow(improper_ctypes)]
#[link(name = "Carbon", kind = "framework")]
extern "C" {
    fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
    fn TISCopyCurrentKeyboardLayoutInputSource() -> TISInputSourceRef;

    #[allow(non_upper_case_globals)]
    #[link_name = "kTISPropertyUnicodeKeyLayoutData"]
    static kTISPropertyUnicodeKeyLayoutData: CFStringRef;

    #[allow(non_snake_case)]
    fn TISGetInputSourceProperty(
        inputSource: TISInputSourceRef,
        propertyKey: CFStringRef,
    ) -> *mut c_void;

    #[allow(non_snake_case)]
    fn CFDataGetBytePtr(theData: CFDataRef) -> *const UInt8;

    #[allow(non_snake_case)]
    fn UCKeyTranslate(
        keyLayoutPtr: *const UInt8, //*const UCKeyboardLayout,
        virtualKeyCode: UInt16,
        keyAction: UInt16,
        modifierKeyState: UInt32,
        keyboardType: UInt32,
        keyTranslateOptions: OptionBits,
        deadKeyState: *mut UInt32,
        maxStringLength: UniCharCount,
        actualStringLength: *mut UniCharCount,
        unicodeString: *mut UniChar,
    ) -> OSStatus;

    fn LMGetKbdType() -> UInt8;

    #[allow(non_snake_case)]
    fn CFStringCreateWithCharacters(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        numChars: CFIndex,
    ) -> CFStringRef;

    #[allow(non_upper_case_globals)]
    #[link_name = "kCFAllocatorDefault"]
    static kCFAllocatorDefault: CFAllocatorRef;

    #[allow(non_snake_case)]
    fn CFStringGetLength(theString: CFStringRef) -> CFIndex;

    #[allow(non_snake_case)]
    fn CFStringGetCString(
        theString: CFStringRef,
        buffer: *mut c_char,
        bufferSize: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
}

pub fn get_layoutdependent_keycode(string: &str) -> CGKeyCode {
    let mut pressed_keycode = 0;

    // loop through every keycode (0 - 127)
    for keycode in 0..128 {
        // no modifier
        if let Some(key_string) = keycode_to_string(keycode, 0x100) {
            if string == key_string {
                pressed_keycode = keycode;
            }
        }

        // shift modifier
        if let Some(key_string) = keycode_to_string(keycode, 0x20102) {
            if string == key_string {
                pressed_keycode = keycode;
            }
        }
    }

    pressed_keycode
}

fn keycode_to_string(keycode: u16, modifier: u32) -> Option<String> {
    let cf_string = create_string_for_key(keycode, modifier);
    let buffer_size = unsafe { CFStringGetLength(cf_string) + 1 };
    let mut buffer: i8 = std::i8::MAX;
    let success =
        unsafe { CFStringGetCString(cf_string, &mut buffer, buffer_size, kCFStringEncodingUTF8) };
    if success == TRUE as u8 {
        let rust_string = String::from_utf8(vec![buffer as u8]).unwrap();
        return Some(rust_string);
    }

    None
}

#[allow(clippy::unused_self)]
fn create_string_for_key(keycode: u16, modifier: u32) -> CFStringRef {
    let mut current_keyboard = unsafe { TISCopyCurrentKeyboardInputSource() };
    let mut layout_data =
        unsafe { TISGetInputSourceProperty(current_keyboard, kTISPropertyUnicodeKeyLayoutData) };
    if layout_data.is_null() {
        // TISGetInputSourceProperty returns null with some keyboard layout.
        // Using TISCopyCurrentKeyboardLayoutInputSource to fix NULL return.
        // See also: https://github.com/microsoft/node-native-keymap/blob/089d802efd387df4dce1f0e31898c66e28b3f67f/src/keyboard_mac.mm#L90
        current_keyboard = unsafe { TISCopyCurrentKeyboardLayoutInputSource() };
        layout_data = unsafe {
            TISGetInputSourceProperty(current_keyboard, kTISPropertyUnicodeKeyLayoutData)
        };
        debug_assert!(!layout_data.is_null());
    }
    let keyboard_layout = unsafe { CFDataGetBytePtr(layout_data) };

    let mut keys_down: UInt32 = 0;
    // let mut chars: *mut c_void;//[UniChar; 4];
    let mut chars: u16 = 0;
    let mut real_length: UniCharCount = 0;
    unsafe {
        UCKeyTranslate(
            keyboard_layout,
            keycode,
            3, // kUCKeyActionDisplay = 3
            modifier,
            LMGetKbdType() as u32,
            kUCKeyTranslateNoDeadKeysBit as u32,
            &mut keys_down,
            8, // sizeof(chars) / sizeof(chars[0]),
            &mut real_length,
            &mut chars,
        );
    }

    unsafe { CFStringCreateWithCharacters(kCFAllocatorDefault, &chars, 1) }
}
