// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_6", feature = "dox"))]
use glib::translate::*;
use webkit2_sys;
#[cfg(any(feature = "v2_6", feature = "dox"))]
use UserContentInjectedFrames;
#[cfg(any(feature = "v2_6", feature = "dox"))]
use UserScriptInjectionTime;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UserScript(Shared<webkit2_sys::WebKitUserScript>);

    match fn {
        ref => |ptr| webkit2_sys::webkit_user_script_ref(ptr),
        unref => |ptr| webkit2_sys::webkit_user_script_unref(ptr),
        get_type => || webkit2_sys::webkit_user_script_get_type(),
    }
}

impl UserScript {
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub fn new(
        source: &str,
        injected_frames: UserContentInjectedFrames,
        injection_time: UserScriptInjectionTime,
        whitelist: &[&str],
        blacklist: &[&str],
    ) -> UserScript {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_user_script_new(
                source.to_glib_none().0,
                injected_frames.to_glib(),
                injection_time.to_glib(),
                whitelist.to_glib_none().0,
                blacklist.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_22", feature = "dox"))]
    pub fn new_for_world(
        source: &str,
        injected_frames: UserContentInjectedFrames,
        injection_time: UserScriptInjectionTime,
        world_name: &str,
        whitelist: &[&str],
        blacklist: &[&str],
    ) -> UserScript {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(webkit2_sys::webkit_user_script_new_for_world(
                source.to_glib_none().0,
                injected_frames.to_glib(),
                injection_time.to_glib(),
                world_name.to_glib_none().0,
                whitelist.to_glib_none().0,
                blacklist.to_glib_none().0,
            ))
        }
    }
}
