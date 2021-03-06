// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AppStreamType {
    Stream,
    Seekable,
    RandomAccess,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for AppStreamType {
    type GlibType = ffi::GstAppStreamType;

    fn to_glib(&self) -> ffi::GstAppStreamType {
        match *self {
            AppStreamType::Stream => ffi::GST_APP_STREAM_TYPE_STREAM,
            AppStreamType::Seekable => ffi::GST_APP_STREAM_TYPE_SEEKABLE,
            AppStreamType::RandomAccess => ffi::GST_APP_STREAM_TYPE_RANDOM_ACCESS,
            AppStreamType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAppStreamType> for AppStreamType {
    fn from_glib(value: ffi::GstAppStreamType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => AppStreamType::Stream,
            1 => AppStreamType::Seekable,
            2 => AppStreamType::RandomAccess,
            value => AppStreamType::__Unknown(value),
        }
    }
}

impl StaticType for AppStreamType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_app_stream_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AppStreamType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AppStreamType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for AppStreamType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

