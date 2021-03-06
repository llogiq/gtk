// This file was generated by gir (0d8699e) from gir-files (11e0e6d)
// DO NOT EDIT

use ApplicationInhibitFlags;
use Window;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Application(Object<ffi::GtkApplication>);

    match fn {
        get_type => || ffi::gtk_application_get_type(),
    }
}

impl Application {
    //pub fn add_accelerator(&self, accelerator: &str, action_name: &str, parameter: /*Ignored*/Option<&glib::Variant>) {
    //    unsafe { TODO: call ffi::gtk_application_add_accelerator() }
    //}

    pub fn add_window<T: IsA<Window>>(&self, window: &T) {
        unsafe {
            ffi::gtk_application_add_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_accels_for_action(&self, detailed_action_name: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_accels_for_action(self.to_glib_none().0, detailed_action_name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_actions_for_accel(&self, accel: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_actions_for_accel(self.to_glib_none().0, accel.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_active_window(self.to_glib_none().0))
        }
    }

    //pub fn get_app_menu(&self) -> /*Ignored*/Option<glib::MenuModel> {
    //    unsafe { TODO: call ffi::gtk_application_get_app_menu() }
    //}

    //#[cfg(feature = "v3_14")]
    //pub fn get_menu_by_id(&self, id: &str) -> /*Ignored*/Option<glib::Menu> {
    //    unsafe { TODO: call ffi::gtk_application_get_menu_by_id() }
    //}

    //pub fn get_menubar(&self) -> /*Ignored*/Option<glib::MenuModel> {
    //    unsafe { TODO: call ffi::gtk_application_get_menubar() }
    //}

    #[cfg(feature = "v3_6")]
    pub fn get_window_by_id(&self, id: u32) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_window_by_id(self.to_glib_none().0, id))
        }
    }

    pub fn get_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_application_get_windows(self.to_glib_none().0))
        }
    }

    pub fn inhibit<T: IsA<Window>>(&self, window: Option<&T>, flags: ApplicationInhibitFlags, reason: Option<&str>) -> u32 {
        unsafe {
            ffi::gtk_application_inhibit(self.to_glib_none().0, window.to_glib_none().0, flags, reason.to_glib_none().0)
        }
    }

    pub fn is_inhibited(&self, flags: ApplicationInhibitFlags) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_is_inhibited(self.to_glib_none().0, flags))
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn list_action_descriptions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_list_action_descriptions(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn prefers_app_menu(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_prefers_app_menu(self.to_glib_none().0))
        }
    }

    //pub fn remove_accelerator(&self, action_name: &str, parameter: /*Ignored*/Option<&glib::Variant>) {
    //    unsafe { TODO: call ffi::gtk_application_remove_accelerator() }
    //}

    pub fn remove_window<T: IsA<Window>>(&self, window: &T) {
        unsafe {
            ffi::gtk_application_remove_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]) {
        unsafe {
            ffi::gtk_application_set_accels_for_action(self.to_glib_none().0, detailed_action_name.to_glib_none().0, accels.to_glib_none().0);
        }
    }

    //pub fn set_app_menu<T: IsA</*Ignored*/glib::MenuModel>>(&self, app_menu: Option<&T>) {
    //    unsafe { TODO: call ffi::gtk_application_set_app_menu() }
    //}

    //pub fn set_menubar<T: IsA</*Ignored*/glib::MenuModel>>(&self, menubar: Option<&T>) {
    //    unsafe { TODO: call ffi::gtk_application_set_menubar() }
    //}

    pub fn uninhibit(&self, cookie: u32) {
        unsafe {
            ffi::gtk_application_uninhibit(self.to_glib_none().0, cookie);
        }
    }
}
