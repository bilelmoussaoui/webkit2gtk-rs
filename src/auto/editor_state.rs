// This file was generated by gir (074a1ca) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct EditorState(Object<ffi::WebKitEditorState>);

    match fn {
        get_type => || ffi::webkit_editor_state_get_type(),
    }
}

impl EditorState {
    #[cfg(feature = "v2_10")]
    pub fn get_typing_attributes(&self) -> u32 {
        unsafe {
            ffi::webkit_editor_state_get_typing_attributes(self.to_glib_none().0)
        }
    }
}