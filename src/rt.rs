// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::Cell;
use std::ptr;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use libc::c_uint;
use ffi;
use glib::translate::*;
use gdk;

thread_local! {
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false)
}

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => (
        if !::rt::is_initialized_main_thread() {
            if ::rt::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => ()
}

/// Asserts that `gtk::init` has not been called.
macro_rules! assert_not_initialized {
    () => (
        if ::rt::is_initialized() {
            panic!("This function has to be called before `gtk::init`.");
        }
    )
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
        if cfg!(debug_assertions) {
            assert_initialized_main_thread!();
        }
    )
}

/// Returns `true` if GTK has been initialized.
#[inline]
pub fn is_initialized() -> bool {
    skip_assert_initialized!();
    INITIALIZED.load(Ordering::Acquire)
}

/// Returns `true` if GTK has been initialized and this is the main thread.
#[inline]
pub fn is_initialized_main_thread() -> bool {
    skip_assert_initialized!();
    IS_MAIN_THREAD.with(|c| c.get())
}

/// Informs this crate that GTK has been initialized and the current thread is the main one.
pub unsafe fn set_initialized() {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return;
    }
    else if is_initialized() {
        panic!("Attempted to initialize GTK from two different threads.");
    }
    gdk::set_initialized();
    INITIALIZED.store(true, Ordering::Release);
    IS_MAIN_THREAD.with(|c| c.set(true));
}

/// Tries to initialize GTK+.
///
/// Call either this function or [`Application::new`][new] before using any
/// other GTK+ functions.
///
/// [new]: struct.Application.html#method.new
///
/// Note that this function calls `gtk_init_check()` rather than `gtk_init()`,
/// so will not cause the program to terminate if GTK could not be initialized.
/// Instead, an Ok is returned if the windowing system was successfully
/// initialized otherwise an Err is returned.
pub fn init() -> Result<(), ()> {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return Ok(());
    }
    else if is_initialized() {
        panic!("Attempted to initialize GTK from two different threads.");
    }
    unsafe {
        if from_glib(ffi::gtk_init_check(ptr::null_mut(), ptr::null_mut())) {
            set_initialized();
            Ok(())
        }
        else {
            Err(())
        }
    }
}

pub fn main() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_main();
    }
}

pub fn main_quit() {
    assert_initialized_main_thread!();
    unsafe {
        if ffi::gtk_main_level() > 0 {
            ffi::gtk_main_quit();
        }
        else if cfg!(debug_assertions) {
            panic!("Attempted to quit a GTK main loop when none is running.");
        }
    }
}

pub fn main_level() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gtk_main_level() as u32
    }
}

pub fn main_iteration() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_main_iteration()) }
}

pub fn main_iteration_do(blocking: bool) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gtk_main_iteration_do(blocking.to_glib())) }
}

pub fn events_pending() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_events_pending())
    }
}

pub fn get_major_version() -> u32 {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_get_major_version() as u32
    }
}

pub fn get_minor_version() -> u32 {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_get_minor_version() as u32
    }
}

pub fn get_micro_version() -> u32 {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_get_micro_version() as u32
    }
}

pub fn get_binary_age() -> u32 {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_get_binary_age() as u32
    }
}

pub fn get_interface_age() -> u32 {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_get_interface_age() as u32
    }
}

pub fn check_version(required_major: u32, required_minor: u32, required_micro: u32) -> Option<String> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(
            ffi::gtk_check_version(required_major as c_uint, required_minor as c_uint, required_micro as c_uint))
    }
 }
