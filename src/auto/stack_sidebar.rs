// This file was generated by gir (0d8699e) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
#[cfg(feature = "v3_16")]
use Stack;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct StackSidebar(Object<ffi::GtkStackSidebar>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_stack_sidebar_get_type(),
    }
}

impl StackSidebar {
    #[cfg(feature = "v3_16")]
    pub fn new() -> StackSidebar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_sidebar_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn get_stack(&self) -> Option<Stack> {
        unsafe {
            from_glib_full(ffi::gtk_stack_sidebar_get_stack(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn set_stack(&self, stack: &Stack) {
        unsafe {
            ffi::gtk_stack_sidebar_set_stack(self.to_glib_none().0, stack.to_glib_none().0);
        }
    }
}
