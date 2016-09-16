// This file was generated by gir (074a1ca) from gir-files (???)
// DO NOT EDIT

use BackForwardListItem;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct BackForwardList(Object<ffi::WebKitBackForwardList>);

    match fn {
        get_type => || ffi::webkit_back_forward_list_get_type(),
    }
}

impl BackForwardList {
    pub fn get_back_item(&self) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_back_item(self.to_glib_none().0))
        }
    }

    pub fn get_back_list(&self) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::webkit_back_forward_list_get_back_list(self.to_glib_none().0))
        }
    }

    pub fn get_back_list_with_limit(&self, limit: u32) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::webkit_back_forward_list_get_back_list_with_limit(self.to_glib_none().0, limit))
        }
    }

    pub fn get_current_item(&self) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_current_item(self.to_glib_none().0))
        }
    }

    pub fn get_forward_item(&self) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_forward_item(self.to_glib_none().0))
        }
    }

    pub fn get_forward_list(&self) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::webkit_back_forward_list_get_forward_list(self.to_glib_none().0))
        }
    }

    pub fn get_forward_list_with_limit(&self, limit: u32) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::webkit_back_forward_list_get_forward_list_with_limit(self.to_glib_none().0, limit))
        }
    }

    pub fn get_length(&self) -> u32 {
        unsafe {
            ffi::webkit_back_forward_list_get_length(self.to_glib_none().0)
        }
    }

    pub fn get_nth_item(&self, index: i32) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_nth_item(self.to_glib_none().0, index))
        }
    }

    //pub fn connect_changed<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Unimplemented items_removed: *.Pointer
    //}
}