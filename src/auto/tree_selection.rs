// This file was generated by gir (0d8699e) from gir-files (11e0e6d)
// DO NOT EDIT

use SelectionMode;
use TreeIter;
use TreeModel;
use TreePath;
use TreeView;
use ffi;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct TreeSelection(Object<ffi::GtkTreeSelection>);

    match fn {
        get_type => || ffi::gtk_tree_selection_get_type(),
    }
}

impl TreeSelection {
    pub fn count_selected_rows(&self) -> i32 {
        unsafe {
            ffi::gtk_tree_selection_count_selected_rows(self.to_glib_none().0)
        }
    }

    pub fn get_mode(&self) -> SelectionMode {
        unsafe {
            ffi::gtk_tree_selection_get_mode(self.to_glib_none().0)
        }
    }

    //pub fn get_select_function(&self) -> /*Unknown conversion*//*Unimplemented*/TreeSelectionFunc {
    //    unsafe { TODO: call ffi::gtk_tree_selection_get_select_function() }
    //}

    pub fn get_selected(&self) -> Option<(TreeModel, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_tree_selection_get_selected(self.to_glib_none().0, &mut model, iter.to_glib_none_mut().0));
            if ret { Some((from_glib_none(model), iter)) } else { None }
        }
    }

    pub fn get_selected_rows(&self) -> (Vec<TreePath>, TreeModel) {
        unsafe {
            let mut model = ptr::null_mut();
            let ret = FromGlibPtrContainer::from_glib_full(ffi::gtk_tree_selection_get_selected_rows(self.to_glib_none().0, &mut model));
            (ret, from_glib_none(model))
        }
    }

    pub fn get_tree_view(&self) -> Option<TreeView> {
        unsafe {
            from_glib_none(ffi::gtk_tree_selection_get_tree_view(self.to_glib_none().0))
        }
    }

    //pub fn get_user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::gtk_tree_selection_get_user_data() }
    //}

    pub fn iter_is_selected(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_selection_iter_is_selected(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    pub fn path_is_selected(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_selection_path_is_selected(self.to_glib_none().0, mut_override(path.to_glib_none().0)))
        }
    }

    pub fn select_all(&self) {
        unsafe {
            ffi::gtk_tree_selection_select_all(self.to_glib_none().0);
        }
    }

    pub fn select_iter(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_select_iter(self.to_glib_none().0, mut_override(iter.to_glib_none().0));
        }
    }

    pub fn select_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_select_path(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    pub fn select_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_select_range(self.to_glib_none().0, mut_override(start_path.to_glib_none().0), mut_override(end_path.to_glib_none().0));
        }
    }

    //pub fn selected_foreach(&self, func: /*Unknown conversion*//*Unimplemented*/TreeSelectionForeachFunc, data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_tree_selection_selected_foreach() }
    //}

    pub fn set_mode(&self, type_: SelectionMode) {
        unsafe {
            ffi::gtk_tree_selection_set_mode(self.to_glib_none().0, type_);
        }
    }

    //pub fn set_select_function(&self, func: /*Unknown conversion*//*Unimplemented*/TreeSelectionFunc, data: /*Unimplemented*/Fundamental: Pointer, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_selection_set_select_function() }
    //}

    pub fn unselect_all(&self) {
        unsafe {
            ffi::gtk_tree_selection_unselect_all(self.to_glib_none().0);
        }
    }

    pub fn unselect_iter(&self, iter: &TreeIter) {
        unsafe {
            ffi::gtk_tree_selection_unselect_iter(self.to_glib_none().0, mut_override(iter.to_glib_none().0));
        }
    }

    pub fn unselect_path(&self, path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_unselect_path(self.to_glib_none().0, mut_override(path.to_glib_none().0));
        }
    }

    pub fn unselect_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe {
            ffi::gtk_tree_selection_unselect_range(self.to_glib_none().0, mut_override(start_path.to_glib_none().0), mut_override(end_path.to_glib_none().0));
        }
    }
}
