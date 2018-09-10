// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
use Error;
use WebView;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct URISchemeRequest(Object<ffi::WebKitURISchemeRequest, ffi::WebKitURISchemeRequestClass>);

    match fn {
        get_type => || ffi::webkit_uri_scheme_request_get_type(),
    }
}

pub trait URISchemeRequestExt {
    //fn finish<'a, P: IsA</*Ignored*/gio::InputStream>, Q: Into<Option<&'a str>>>(&self, stream: &P, stream_length: i64, mime_type: Q);

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn finish_error(&self, error: &mut Error);

    fn get_path(&self) -> Option<String>;

    fn get_scheme(&self) -> Option<String>;

    fn get_uri(&self) -> Option<String>;

    fn get_web_view(&self) -> Option<WebView>;
}

impl<O: IsA<URISchemeRequest>> URISchemeRequestExt for O {
    //fn finish<'a, P: IsA</*Ignored*/gio::InputStream>, Q: Into<Option<&'a str>>>(&self, stream: &P, stream_length: i64, mime_type: Q) {
    //    unsafe { TODO: call ffi::webkit_uri_scheme_request_finish() }
    //}

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn finish_error(&self, error: &mut Error) {
        unsafe {
            ffi::webkit_uri_scheme_request_finish_error(self.to_glib_none().0, error.to_glib_none_mut().0);
        }
    }

    fn get_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_scheme_request_get_path(self.to_glib_none().0))
        }
    }

    fn get_scheme(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_scheme_request_get_scheme(self.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_scheme_request_get_uri(self.to_glib_none().0))
        }
    }

    fn get_web_view(&self) -> Option<WebView> {
        unsafe {
            from_glib_none(ffi::webkit_uri_scheme_request_get_web_view(self.to_glib_none().0))
        }
    }
}
