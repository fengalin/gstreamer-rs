// This file was generated by gir (12a28ac) from gir-files (???)
// DO NOT EDIT

use Caps;
use Object;
use StreamFlags;
use StreamType;
use TagList;
use ffi;
use glib::Value;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Stream(Object<ffi::GstStream>): Object;

    match fn {
        get_type => || ffi::gst_stream_get_type(),
    }
}

impl Stream {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b Caps>>>(stream_id: P, caps: Q, type_: StreamType, flags: StreamFlags) -> Stream {
        assert_initialized_main_thread!();
        let stream_id = stream_id.into();
        let stream_id = stream_id.to_glib_none();
        let caps = caps.into();
        let caps = caps.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_stream_new(stream_id.0, caps.0, type_.to_glib(), flags.to_glib()))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_stream_get_caps(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_flags(&self) -> StreamFlags {
        unsafe {
            from_glib(ffi::gst_stream_get_stream_flags(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_stream_get_stream_id(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_type(&self) -> StreamType {
        unsafe {
            from_glib(ffi::gst_stream_get_stream_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_tags(&self) -> Option<TagList> {
        unsafe {
            from_glib_full(ffi::gst_stream_get_tags(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_caps<'a, P: Into<Option<&'a Caps>>>(&self, caps: P) {
        let caps = caps.into();
        let caps = caps.to_glib_none();
        unsafe {
            ffi::gst_stream_set_caps(self.to_glib_none().0, caps.0);
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_stream_flags(&self, flags: StreamFlags) {
        unsafe {
            ffi::gst_stream_set_stream_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_stream_type(&self, stream_type: StreamType) {
        unsafe {
            ffi::gst_stream_set_stream_type(self.to_glib_none().0, stream_type.to_glib());
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_tags<'a, P: Into<Option<&'a TagList>>>(&self, tags: P) {
        let tags = tags.into();
        let tags = tags.to_glib_none();
        unsafe {
            ffi::gst_stream_set_tags(self.to_glib_none().0, tags.0);
        }
    }

    pub fn get_property_caps(&self) -> Option<Caps> {
        let mut value = Value::from(None::<&Caps>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "caps".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_caps(&self, caps: Option<&Caps>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "caps".to_glib_none().0, Value::from(caps).to_glib_none().0);
        }
    }

    pub fn get_property_stream_flags(&self) -> StreamFlags {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stream-flags".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<u32>().unwrap()))
        }
    }

    pub fn set_property_stream_flags(&self, stream_flags: StreamFlags) {
        let stream_flags = stream_flags.to_glib().bits() as u32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stream-flags".to_glib_none().0, Value::from(&stream_flags).to_glib_none().0);
        }
    }

    pub fn get_property_stream_id(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stream-id".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn get_property_stream_type(&self) -> StreamType {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stream-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<u32>().unwrap()))
        }
    }

    pub fn set_property_stream_type(&self, stream_type: StreamType) {
        let stream_type = stream_type.to_glib().bits() as u32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stream-type".to_glib_none().0, Value::from(&stream_type).to_glib_none().0);
        }
    }

    pub fn get_property_tags(&self) -> Option<TagList> {
        let mut value = Value::from(None::<&TagList>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "tags".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_tags(&self, tags: Option<&TagList>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "tags".to_glib_none().0, Value::from(tags).to_glib_none().0);
        }
    }

    pub fn connect_property_caps_notify<F: Fn(&Stream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Stream) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::caps",
                transmute(notify_caps_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_stream_flags_notify<F: Fn(&Stream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Stream) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stream-flags",
                transmute(notify_stream_flags_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_stream_id_notify<F: Fn(&Stream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Stream) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stream-id",
                transmute(notify_stream_id_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_stream_type_notify<F: Fn(&Stream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Stream) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stream-type",
                transmute(notify_stream_type_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_tags_notify<F: Fn(&Stream) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Stream) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tags",
                transmute(notify_tags_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

unsafe extern "C" fn notify_caps_trampoline(this: *mut ffi::GstStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Stream) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_stream_flags_trampoline(this: *mut ffi::GstStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Stream) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_stream_id_trampoline(this: *mut ffi::GstStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Stream) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_stream_type_trampoline(this: *mut ffi::GstStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Stream) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_tags_trampoline(this: *mut ffi::GstStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &&(Fn(&Stream) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}
