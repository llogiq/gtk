// This file was generated by gir (0d8699e) from gir-files (11e0e6d)
// DO NOT EDIT

use Error;
use ffi;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct CssProvider(Object<ffi::GtkCssProvider>);

    match fn {
        get_type => || ffi::gtk_css_provider_get_type(),
    }
}

impl CssProvider {
    pub fn new() -> CssProvider {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_css_provider_new())
        }
    }

    //pub fn load_from_data(&self, data: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 3 }, length: /*Unimplemented*/Fundamental: SSize) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_css_provider_load_from_data() }
    //}

    //pub fn load_from_file<T: IsA</*Ignored*/glib::File>>(&self, file: &T) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_css_provider_load_from_file() }
    //}

    pub fn load_from_path(&self, path: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_css_provider_load_from_path(self.to_glib_none().0, path.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn load_from_resource(&self, resource_path: &str) {
        unsafe {
            ffi::gtk_css_provider_load_from_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::gtk_css_provider_to_string(self.to_glib_none().0))
        }
    }

    pub fn get_default() -> Option<CssProvider> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_css_provider_get_default())
        }
    }

    pub fn get_named(name: &str, variant: Option<&str>) -> Option<CssProvider> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_css_provider_get_named(name.to_glib_none().0, variant.to_glib_none().0))
        }
    }
}

impl fmt::Display for CssProvider {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
