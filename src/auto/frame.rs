// This file was generated by gir (0d8699e) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use ShadowType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct Frame(Object<ffi::GtkFrame>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_frame_get_type(),
    }
}

impl Frame {
    pub fn new(label: Option<&str>) -> Frame {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_frame_new(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait FrameExt {
    fn get_label(&self) -> Option<String>;

    fn get_label_align(&self) -> (f32, f32);

    fn get_label_widget(&self) -> Option<Widget>;

    fn get_shadow_type(&self) -> ShadowType;

    fn set_label(&self, label: Option<&str>);

    fn set_label_align(&self, xalign: f32, yalign: f32);

    fn set_label_widget<T: IsA<Widget>>(&self, label_widget: Option<&T>);

    fn set_shadow_type(&self, type_: ShadowType);
}

impl<O: IsA<Frame>> FrameExt for O {
    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_frame_get_label(self.to_glib_none().0))
        }
    }

    fn get_label_align(&self) -> (f32, f32) {
        unsafe {
            let mut xalign = mem::uninitialized();
            let mut yalign = mem::uninitialized();
            ffi::gtk_frame_get_label_align(self.to_glib_none().0, &mut xalign, &mut yalign);
            (xalign, yalign)
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_frame_get_label_widget(self.to_glib_none().0))
        }
    }

    fn get_shadow_type(&self) -> ShadowType {
        unsafe {
            ffi::gtk_frame_get_shadow_type(self.to_glib_none().0)
        }
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::gtk_frame_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_align(&self, xalign: f32, yalign: f32) {
        unsafe {
            ffi::gtk_frame_set_label_align(self.to_glib_none().0, xalign, yalign);
        }
    }

    fn set_label_widget<T: IsA<Widget>>(&self, label_widget: Option<&T>) {
        unsafe {
            ffi::gtk_frame_set_label_widget(self.to_glib_none().0, label_widget.to_glib_none().0);
        }
    }

    fn set_shadow_type(&self, type_: ShadowType) {
        unsafe {
            ffi::gtk_frame_set_shadow_type(self.to_glib_none().0, type_);
        }
    }
}
