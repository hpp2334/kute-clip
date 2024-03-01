use std::ffi::c_void;

use super::defs::{PlatformManager, PlatformManagerImpl};
use cocoa::appkit::{NSApp, NSApplication};
use core_foundation::base::CFRelease;
use core_graphics::{
    event::{CGEventFlags, CGEventTapLocation, CGKeyCode},
    event_source::CGEventSourceStateID,
    sys::CGEventRef,
};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};

mod keycode;

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventSourceCreate(state_id: CGEventSourceStateID) -> CGEventRef;
    fn CGEventCreateKeyboardEvent(
        source: CGEventRef,
        virtualKey: CGKeyCode,
        keyDown: bool,
    ) -> CGEventRef;
    fn CGEventPost(tap: CGEventTapLocation, event: CGEventRef);
    fn CGEventSetFlags(event: CGEventRef, flags: u64);
}

trait NSNumber {
    unsafe fn long_value(self) -> i64;
}

impl NSNumber for *mut Object {
    unsafe fn long_value(self) -> i64 {
        msg_send![self, longValue]
    }
}

fn get_current_app() -> i64 {
    let workspace: *mut Object = unsafe { msg_send![class!(NSWorkspace), sharedWorkspace] };
    let app: *mut Object = unsafe { msg_send![workspace, frontmostApplication] };
    let pid: i64 = unsafe { msg_send![app, processIdentifier] };
    pid
}

fn restore_app(pid: i64) {
    let current_app: *mut Object = unsafe {
        msg_send![
            class!(NSRunningApplication),
            runningApplicationWithProcessIdentifier: pid
        ]
    };
    if current_app.is_null() {
        return;
    }
    const NSAPPLICATION_ACTIVATE_ALL_WINDOWS: i32 = 1 << 0;
    unsafe {
        msg_send![
            current_app,
            activateWithOptions: NSAPPLICATION_ACTIVATE_ALL_WINDOWS
        ]
    }
}

fn disable_active_current_app() {
    unsafe {
        let ns_app: *mut Object = NSApp();
        let _res = ns_app.setActivationPolicy_(
            cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyProhibited,
        );
    }
}

fn enable_active_current_app() {
    unsafe {
        let ns_app: *mut Object = NSApp();
        let _res = ns_app.setActivationPolicy_(
            cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory,
        );
    }
}


fn paste_clipboard_impl() {
    unsafe {
        let virtual_key_v = keycode::get_layoutdependent_keycode("v");

        let source_ref = CGEventSourceCreate(CGEventSourceStateID::CombinedSessionState);

        let key_v_up = CGEventCreateKeyboardEvent(source_ref, virtual_key_v, false);
        let key_v_down = CGEventCreateKeyboardEvent(source_ref, virtual_key_v, true);

        CGEventSetFlags(key_v_up, CGEventFlags::CGEventFlagCommand.bits() | 0x000008);
        CGEventSetFlags(
            key_v_down,
            CGEventFlags::CGEventFlagCommand.bits() | 0x000008,
        );

        CGEventPost(CGEventTapLocation::HID, key_v_down);
        CGEventPost(CGEventTapLocation::HID, key_v_up);

        CFRelease(key_v_up as *const c_void);
        CFRelease(key_v_down as *const c_void);
        CFRelease(source_ref as *const c_void);
    }
}

fn paste_clipboard() {
    let pid = get_current_app();
    paste_clipboard_impl();
    restore_app(pid);
}

impl PlatformManagerImpl for PlatformManager {
    fn disable_active_current_app() {
        disable_active_current_app()
    }

    fn enable_active_current_app() {
        enable_active_current_app()
    }

    fn paste_clipboard() {
        paste_clipboard()
    }
}
