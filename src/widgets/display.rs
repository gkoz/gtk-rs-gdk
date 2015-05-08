// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GdkDisplay — Controls a set of GdkScreens and their associated input devices

use ffi;
use libc::{c_uint};
use glib::translate::{FromGlibPtr, ToGlibPtr};
use glib::to_bool;

#[repr(C)]
pub struct Display {
    pointer: *mut ffi::C_GdkDisplay
}

impl Display {
    pub fn open(display_name: &str) -> Option<Display> {
        let tmp = unsafe {
            ffi::gdk_display_open(display_name.lend_to_glib().0)
        };

        if tmp.is_null() {
            None
        } else {
            Some(Display {
                pointer: tmp
            })
        }
    }

    pub fn get_default() -> Option<Display> {
        let tmp = unsafe { ffi::gdk_display_get_default() };

        if tmp.is_null() {
            None
        } else {
            Some(Display {
                pointer: tmp
            })
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            FromGlibPtr::borrow_from_glib(
                ffi::gdk_display_get_name(self.pointer))
        }
    }

    /*pub fn get_screen(&self, screen_num: i32) -> Option<::Screen> {
        let tmp = unsafe { ffi::gdk_display_get_screen(self.pointer, screen_num as c_int) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::Screen::wrap(tmp)) }
        }
    }

    pub fn get_default_screen(&self, screen_num: i32) -> Option<::Screen> {
        let tmp = unsafe { ffi::gdk_display_get_default_screen(self.pointer, screen_num as c_int) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::Screen::wrap(tmp)) }
        }
    }

    pub fn get_device_manager(&self, screen_num: i32) -> Option<::DeviceManager> {
        let tmp = unsafe { ffi::gdk_display_get_device_manager(self.pointer, screen_num as c_int) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::DeviceManager::wrap(tmp)) }
        }
    }*/

    pub fn device_is_grabbed(&self, device: &::Device) -> bool {
        unsafe { to_bool(ffi::gdk_display_device_is_grabbed(self.pointer, device.unwrap_pointer())) }
    }

    pub fn beep(&self) {
        unsafe { ffi::gdk_display_beep(self.pointer) }
    }

    pub fn sync(&self) {
        unsafe { ffi::gdk_display_sync(self.pointer) }
    }

    pub fn flush(&self) {
        unsafe { ffi::gdk_display_flush(self.pointer) }
    }

    pub fn close(&self) {
        unsafe { ffi::gdk_display_close(self.pointer) }
    }

    pub fn is_closed(&self) -> bool {
        unsafe { to_bool(ffi::gdk_display_is_closed(self.pointer)) }
    }

    /*pub fn get_event(&self) -> Option<::Event> {
        let tmp = unsafe { ffi::gdk_display_get_event(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::Event::wrap(tmp)) }
        }
    }

    pub fn peek_event(&self) -> Option<::Event> {
        let tmp = unsafe { ffi::gdk_display_peek_event(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::Event::wrap(tmp)) }
        }
    }

    pub fn put_event(&self, event: &::Event) {
        unsafe { ffi::gdk_display_put_event(self.pointer, event.unwrap_pointer() as *const ffi::C_GdkEvent) }
    }*/

    pub fn has_pending(&self) -> bool {
        unsafe { to_bool(ffi::gdk_display_has_pending(self.pointer)) }
    }

    pub fn set_double_click_time(&self, msec: u32) {
        unsafe { ffi::gdk_display_set_double_click_time(self.pointer, msec as c_uint) }
    }

    pub fn set_double_click_distance(&self, msec: u32) {
        unsafe { ffi::gdk_display_set_double_click_distance(self.pointer, msec as c_uint) }
    }

    pub fn supports_cursor_color(&self) -> bool {
        unsafe { to_bool(ffi::gdk_display_supports_cursor_color(self.pointer)) }
    }

    pub fn supports_cursor_alpha(&self) -> bool {
        unsafe { to_bool(ffi::gdk_display_supports_cursor_alpha(self.pointer)) }
    }

    pub fn get_default_cursor_size(&self) -> u32 {
        unsafe { ffi::gdk_display_get_default_cursor_size(self.pointer) }
    }

    pub fn get_maximal_cursor_size(&self, width: &mut u32, height: &mut u32) {
        unsafe { ffi::gdk_display_get_maximal_cursor_size(self.pointer, width as *mut c_uint, height as *mut c_uint) }
    }

    /*pub fn get_default_group(&self) -> Option<::Window> {
        let tmp = unsafe { ffi::gdk_display_get_default_group(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::Window::wrap(tmp)) }
        }
    }*/

    pub fn supports_selection_notification(&self) -> bool {
        unsafe { to_bool(ffi::gdk_display_supports_selection_notification(self.pointer)) }
    }

    /*pub fn request_selection_notification(&self, selection: &::Atom) -> bool {
        unsafe { to_bool(ffi::gdk_display_request_selection_notification(self.pointer, selection.unwrap_pointer())) }
    }*/

    pub fn supports_clipboard_persistence(&self) -> bool {
        unsafe { to_bool(ffi::gdk_display_supports_clipboard_persistence(self.pointer)) }
    }

    /*pub fn store_clipboard(&self, clipboard_window: &::Window, time_: u32, targets: Vec<::Atom>) {
        unsafe { ffi::gdk_display_store_clipboard(self.pointer, clipboard_window.unwrap_pointer(), time_, targets.as_mut_pointer(),
            targets.len() as c_int) }
    }*/

    pub fn supports_shapes(&self) -> bool {
        unsafe { to_bool(ffi::gdk_display_supports_shapes(self.pointer)) }
    }

    pub fn supports_input_shapes(&self) -> bool {
        unsafe { to_bool(ffi::gdk_display_supports_input_shapes(self.pointer)) }
    }

    pub fn supports_composite(&self) -> bool {
        unsafe { to_bool(ffi::gdk_display_supports_composite(self.pointer)) }
    }

    /*pub fn get_app_launch_context(&self) -> Option<::AppLaunchContext> {
        let tmp = unsafe { ffi::gdk_display_get_app_launch_context(self.pointer) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::AppLaunchContext::wrap(tmp)) }
        }
    }*/

    pub fn notify_startup_complete(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_display_notify_startup_complete(self.pointer, startup_id.lend_to_glib().0)
        }
    }
}

impl_GObjectFunctions!(Display, C_GdkDisplay);
