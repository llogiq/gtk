// This file was generated by gir (7dd2bcd) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use TextTag;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TextTagTable(Object<ffi::GtkTextTagTable>): Buildable;

    match fn {
        get_type => || ffi::gtk_text_tag_table_get_type(),
    }
}

impl TextTagTable {
    pub fn new() -> TextTagTable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_tag_table_new())
        }
    }

    pub fn add(&self, tag: &TextTag) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_tag_table_add(self.to_glib_none().0, tag.to_glib_none().0))
        }
    }

    //pub fn foreach(&self, func: /*Unknown conversion*//*Unimplemented*/TextTagTableForeach, data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_text_tag_table_foreach() }
    //}

    pub fn get_size(&self) -> i32 {
        unsafe {
            ffi::gtk_text_tag_table_get_size(self.to_glib_none().0)
        }
    }

    pub fn lookup(&self, name: &str) -> Option<TextTag> {
        unsafe {
            from_glib_none(ffi::gtk_text_tag_table_lookup(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn remove(&self, tag: &TextTag) {
        unsafe {
            ffi::gtk_text_tag_table_remove(self.to_glib_none().0, tag.to_glib_none().0);
        }
    }
}