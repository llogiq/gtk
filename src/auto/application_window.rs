// This file was generated by gir (0d8699e) from gir-files (11e0e6d)
// DO NOT EDIT

use Application;
use Bin;
use Buildable;
use Container;
use Widget;
use Window;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ApplicationWindow(Object<ffi::GtkApplicationWindow>): Window, Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_application_window_get_type(),
    }
}

impl ApplicationWindow {
    pub fn new(application: &Application) -> ApplicationWindow {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_application_window_new(application.to_glib_none().0)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_id(&self) -> u32 {
        unsafe {
            ffi::gtk_application_window_get_id(self.to_glib_none().0)
        }
    }

    pub fn get_show_menubar(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_window_get_show_menubar(self.to_glib_none().0))
        }
    }

    pub fn set_show_menubar(&self, show_menubar: bool) {
        unsafe {
            ffi::gtk_application_window_set_show_menubar(self.to_glib_none().0, show_menubar.to_glib());
        }
    }
}
