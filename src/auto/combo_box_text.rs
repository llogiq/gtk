// This file was generated by gir (0d8699e) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use CellEditable;
use CellLayout;
use ComboBox;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ComboBoxText(Object<ffi::GtkComboBoxText>): ComboBox, Bin, Container, Widget, Buildable, CellEditable, CellLayout;

    match fn {
        get_type => || ffi::gtk_combo_box_text_get_type(),
    }
}

impl ComboBoxText {
    pub fn new() -> ComboBoxText {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_text_new()).downcast_unchecked()
        }
    }

    pub fn new_with_entry() -> ComboBoxText {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_text_new_with_entry()).downcast_unchecked()
        }
    }

    pub fn append(&self, id: Option<&str>, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append(self.to_glib_none().0, id.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn append_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn get_active_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_combo_box_text_get_active_text(self.to_glib_none().0))
        }
    }

    pub fn insert(&self, position: i32, id: Option<&str>, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert(self.to_glib_none().0, position, id.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn insert_text(&self, position: i32, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert_text(self.to_glib_none().0, position, text.to_glib_none().0);
        }
    }

    pub fn prepend(&self, id: Option<&str>, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend(self.to_glib_none().0, id.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn prepend_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn remove(&self, position: i32) {
        unsafe {
            ffi::gtk_combo_box_text_remove(self.to_glib_none().0, position);
        }
    }

    pub fn remove_all(&self) {
        unsafe {
            ffi::gtk_combo_box_text_remove_all(self.to_glib_none().0);
        }
    }
}
