// This file was generated by gir (074a1ca) from gir-files (???)
// DO NOT EDIT

use PermissionRequest;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct UserMediaPermissionRequest(Object<ffi::WebKitUserMediaPermissionRequest>): PermissionRequest;

    match fn {
        get_type => || ffi::webkit_user_media_permission_request_get_type(),
    }
}

impl UserMediaPermissionRequest {}