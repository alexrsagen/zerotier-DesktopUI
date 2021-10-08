// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Component;
use crate::Object;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtkSocket")]
    pub struct Socket(Object<ffi::AtkSocket, ffi::AtkSocketClass>) @extends Object, @implements Component;

    match fn {
        type_ => || ffi::atk_socket_get_type(),
    }
}

impl Socket {
    #[doc(alias = "atk_socket_new")]
    pub fn new() -> Socket {
        assert_initialized_main_thread!();
        unsafe { Object::from_glib_full(ffi::atk_socket_new()).unsafe_cast() }
    }
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SOCKET: Option<&Socket> = None;

pub trait AtkSocketExt: 'static {
    #[doc(alias = "atk_socket_embed")]
    fn embed(&self, plug_id: &str);

    #[doc(alias = "atk_socket_is_occupied")]
    fn is_occupied(&self) -> bool;
}

impl<O: IsA<Socket>> AtkSocketExt for O {
    fn embed(&self, plug_id: &str) {
        unsafe {
            ffi::atk_socket_embed(self.as_ref().to_glib_none().0, plug_id.to_glib_none().0);
        }
    }

    fn is_occupied(&self) -> bool {
        unsafe { from_glib(ffi::atk_socket_is_occupied(self.as_ref().to_glib_none().0)) }
    }
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Socket")
    }
}
