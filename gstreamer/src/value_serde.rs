// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::{StaticType, ToValue};

use num_rational::Rational32;

use serde::de;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser;
use serde::ser::{Serialize, Serializer, SerializeTuple};

use std::{fmt, mem};

use DateTime;
use Sample;

use value::*;

pub const ARRAY_TYPE_NAME: &'static str = "Array";
pub const LIST_TYPE_NAME: &'static str = "List";

fn get_other_type_id<T: StaticType>() -> usize {
    match T::static_type() {
        glib::Type::Other(type_id) => type_id,
        type_ => panic!("Expecting `Other` variant, found `{}`", type_),
    }
}

lazy_static! {
    pub(crate) static ref ARRAY_OTHER_TYPE_ID: usize = get_other_type_id::<Array>();
    pub(crate) static ref BITMASK_OTHER_TYPE_ID: usize = get_other_type_id::<Bitmask>();
    pub(crate) static ref DATE_TIME_OTHER_TYPE_ID: usize = get_other_type_id::<DateTime>();
    pub(crate) static ref FRACTION_OTHER_TYPE_ID: usize = get_other_type_id::<Fraction>();
    pub(crate) static ref FRACTION_RANGE_OTHER_TYPE_ID: usize =
        get_other_type_id::<FractionRange>();
    pub(crate) static ref INT_RANGE_I32_OTHER_TYPE_ID: usize =
        get_other_type_id::<IntRange<i32>>();
    pub(crate) static ref INT_RANGE_I64_OTHER_TYPE_ID: usize =
        get_other_type_id::<IntRange<i64>>();
    pub(crate) static ref LIST_OTHER_TYPE_ID: usize = get_other_type_id::<List>();
    pub(crate) static ref SAMPLE_OTHER_TYPE_ID: usize = get_other_type_id::<Sample>();
}

impl<'a> Serialize for Fraction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Fraction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Rational32::deserialize(deserializer)
            .and_then(|rational| Ok(Fraction::new(*rational.numer(), *rational.denom())))
    }
}

macro_rules! ser_value (
    ($value:expr, $t_str:expr, $t:ty, $ser_closure:expr) => (
        {
            let value = $value.get::<$t>().unwrap();
            $ser_closure($t_str, value)
        }
    );
    ($value:expr, $ser_closure:expr) => (
        match $value.type_() {
            glib::Type::I8 => ser_value!($value, "i8", i8, $ser_closure),
            glib::Type::U8 => ser_value!($value, "ui8", u8, $ser_closure),
            glib::Type::Bool => ser_value!($value, "bool", bool, $ser_closure),
            glib::Type::I32 => ser_value!($value, "i32", i32, $ser_closure),
            glib::Type::U32 => ser_value!($value, "u32", u32, $ser_closure),
            glib::Type::ILong => ser_value!($value, "ilong", i32, $ser_closure),
            glib::Type::ULong => ser_value!($value, "ulong", u32, $ser_closure),
            glib::Type::I64 => ser_value!($value, "i64", i64, $ser_closure),
            glib::Type::U64 => ser_value!($value, "u64", u64, $ser_closure),
            glib::Type::F32 => ser_value!($value, "f32", f32, $ser_closure),
            glib::Type::F64 => ser_value!($value, "f64", f64, $ser_closure),
            glib::Type::String => ser_value!($value, "String", String, $ser_closure),
            glib::Type::Other(type_id) => {
                if *ARRAY_OTHER_TYPE_ID == type_id {
                    ser_value!($value, ARRAY_TYPE_NAME, Array, $ser_closure)
                } else if *BITMASK_OTHER_TYPE_ID == type_id {
                    ser_value!($value, "Bitmask", Bitmask, $ser_closure)
                } else if *DATE_TIME_OTHER_TYPE_ID == type_id {
                    ser_value!($value, "DateTime", DateTime, $ser_closure)
                } else if *FRACTION_OTHER_TYPE_ID == type_id {
                    ser_value!($value, "Fraction", Fraction, $ser_closure)
                } else if *FRACTION_RANGE_OTHER_TYPE_ID == type_id {
                    ser_value!($value, "FractionRange", FractionRange, $ser_closure)
                } else if *INT_RANGE_I32_OTHER_TYPE_ID == type_id {
                    ser_value!($value, "IntRange<i32>", IntRange<i32>, $ser_closure)
                } else if *INT_RANGE_I64_OTHER_TYPE_ID == type_id {
                    ser_value!($value, "IntRange<i64>", IntRange<i64>, $ser_closure)
                } else if *LIST_OTHER_TYPE_ID == type_id {
                    ser_value!($value, LIST_TYPE_NAME, List, $ser_closure)
                } else if *SAMPLE_OTHER_TYPE_ID == type_id {
                    ser_value!($value, "Sample", Sample, $ser_closure)
                } else {
                    Err(
                        ser::Error::custom(
                            format!("unimplemented `Value` serialization for type {}",
                                glib::Type::Other(type_id),
                            )
                        )
                    )
                }
            }
            type_ => {
                Err(
                    ser::Error::custom(
                        format!("unimplemented `Value` serialization for type {}", type_)
                    )
                )
            }
        }
    )
);

pub(crate) struct SendValue(glib::SendValue);
impl SendValue {
    pub(crate) fn from(send_value: glib::SendValue) -> Self {
        SendValue(send_value)
    }
}

impl From<SendValue> for glib::SendValue {
    fn from(send_value: SendValue) -> Self {
        send_value.0
    }
}

impl Serialize for SendValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ser_value!(self.0, |type_, value| {
            let mut tup = serializer.serialize_tuple(2)?;
            tup.serialize_element(type_)?;
            tup.serialize_element(&value)?;
            tup.end()
        })
    }
}

macro_rules! impl_ser_send_value_collection (
    ($t:ident) => (
        impl<'a> Serialize for $t<'a> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let send_value_vec = unsafe {
                    mem::transmute::<&[glib::SendValue], &[SendValue]>(
                        self.as_slice()
                    )
                };
                send_value_vec.serialize(serializer)
            }
        }
    );
);

impl_ser_send_value_collection!(Array);
impl_ser_send_value_collection!(List);

macro_rules! de_value(
    ($outer_type:expr, $type_name:expr, $seq:expr, $t:ty) => (
        $seq
            .next_element::<$t>()?
            .ok_or_else(||
                de::Error::custom(format!(
                    "Expected a value for `{}` with type {:?}",
                    $outer_type,
                    $type_name,
                ))
            )?
            .to_value()
    );
);

macro_rules! de_send_value(
    ($type_name:expr, $seq:expr, $t:ty) => (
        SendValue::from(
            de_value!("Value", $type_name, $seq, $t)
                .try_into_send_value::<$t>()
                .map_err(|_|
                    de::Error::custom(format!(
                        "Failed to convert `Value` with type {:?} to `SendValue`",
                        $type_name,
                    ))
                )?
        )
    );
    ($type_name:expr, $seq:expr) => (
        match $type_name.as_str() {
            "i8" => de_send_value!($type_name, $seq, i8),
            "u8" => de_send_value!($type_name, $seq, u8),
            "bool" => de_send_value!($type_name, $seq, bool),
            "i32" => de_send_value!($type_name, $seq, i32),
            "u32" => de_send_value!($type_name, $seq, u32),
            "ilong" => de_send_value!($type_name, $seq, i32),
            "ulong" => de_send_value!($type_name, $seq, u32),
            "i64" => de_send_value!($type_name, $seq, i64),
            "u64" => de_send_value!($type_name, $seq, u64),
            "f32" => de_send_value!($type_name, $seq, f32),
            "f64" => de_send_value!($type_name, $seq, f64),
            "String" => de_send_value!($type_name, $seq, String),
            "Array" => de_send_value!($type_name, $seq, Array),
            "Bitmask" => de_send_value!($type_name, $seq, Bitmask),
            "DateTime" => de_send_value!($type_name, $seq, DateTime),
            "Fraction" => de_send_value!($type_name, $seq, Fraction),
            "FractionRange" => de_send_value!($type_name, $seq, FractionRange),
            "IntRange<i32>" => de_send_value!($type_name, $seq, IntRange<i32>),
            "IntRange<i64>" => de_send_value!($type_name, $seq, IntRange<i64>),
            "Sample" => de_send_value!($type_name, $seq, Sample),
            _ => return Err(
                de::Error::custom(
                    format!(
                        "unimplemented deserialization for `Value` with type `{}`",
                        $type_name,
                    ),
                )
            ),
        }
    );
);

struct SendValueVisitor;
impl<'de> Visitor<'de> for SendValueVisitor {
    type Value = SendValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a tuple: (name, value)")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let type_name = seq.next_element::<String>()?
            .ok_or(de::Error::custom("Expected a value for `Value` type"))?;
        Ok(de_send_value!(type_name, seq))
    }
}

impl<'de> Deserialize<'de> for SendValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple(2, SendValueVisitor{})
    }
}

macro_rules! impl_de_send_value_collection (
    ($t:ident) => {
        impl<'a, 'de> Deserialize<'de> for $t<'a> {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let send_value_vec = Vec::<SendValue>::deserialize(deserializer)?;
                Ok($t::from_owned(unsafe{
                    mem::transmute::<Vec<SendValue>, Vec<glib::SendValue>>(send_value_vec)
                }))
            }
        }
    }
);

impl_de_send_value_collection!(Array);
impl_de_send_value_collection!(List);

#[cfg(test)]
mod tests {
    extern crate ron;
    extern crate serde_json;

    #[test]
    fn test_serialize_simple() {
        use Fraction;
        use FractionRange;
        use IntRange;
        use Bitmask;

        ::init().unwrap();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        // Fraction
        let fraction = Fraction::new(1, 3);

        let res = ron::ser::to_string_pretty(&fraction, pretty_config.clone());
        assert_eq!(Ok("(1, 3)".to_owned()), res);

        let res = serde_json::to_string(&fraction).unwrap();
        assert_eq!("[1,3]".to_owned(), res);

        // FractionRange
        let fraction_range = FractionRange::new(Fraction::new(1, 3), Fraction::new(1, 2));

        let res = ron::ser::to_string_pretty(&fraction_range, pretty_config.clone());
        assert_eq!(
            Ok(
                concat!(
                    "(",
                    "    min: (1, 3),",
                    "    max: (1, 2),",
                    ")"
                )
                    .to_owned()
            ),
            res,
        );

        let res = serde_json::to_string(&fraction_range).unwrap();
        assert_eq!("{\"min\":[1,3],\"max\":[1,2]}".to_owned(), res);

        // IntRange
        let int_range = IntRange::<i32>::new_with_step(0, 42, 21);
        let res = ron::ser::to_string_pretty(&int_range, pretty_config.clone());
        assert_eq!(
            Ok(
                concat!(
                    "(",
                    "    min: 0,",
                    "    max: 42,",
                    "    step: 21,",
                    ")"
                )
                    .to_owned()
            ),
            res,
        );

        let res = serde_json::to_string(&int_range).unwrap();
        assert_eq!("{\"min\":0,\"max\":42,\"step\":21}".to_owned(), res);

        // Bitmask
        let bitmask = Bitmask::new(1024 + 128 + 32);

        let res = ron::ser::to_string_pretty(&bitmask, pretty_config.clone());
        assert_eq!(Ok("(1184)".to_owned()), res);

        let res = serde_json::to_string(&bitmask).unwrap();
        assert_eq!("1184".to_owned(), res);
    }

    #[test]
    fn test_serialize_collections() {
        use glib::value::ToValue;

        use Array;
        use Fraction;
        use List;

        ::init().unwrap();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        // Array
        let value_13 = Fraction::new(1, 3).to_value();
        let send_value_13 = value_13.try_into_send_value::<Fraction>().unwrap();

        let value_12 = Fraction::new(1, 2).to_value();
        let send_value_12 = value_12.try_into_send_value::<Fraction>().unwrap();

        let value_str = "test str".to_value();
        let send_value_str = value_str.try_into_send_value::<String>().unwrap();

        let array = Array::new(&[&send_value_13, &send_value_12, &send_value_str]);

        let res = ron::ser::to_string_pretty(&array, pretty_config.clone());
        assert_eq!(
            Ok(
                concat!(
                    "[",
                    "    (\"Fraction\", (1, 3)),",
                    "    (\"Fraction\", (1, 2)),",
                    "    (\"String\", \"test str\"),",
                    "]"
                )
                    .to_owned()
            ),
            res,
        );

        let res = serde_json::to_string(&array).unwrap();
        assert_eq!(
            "[[\"Fraction\",[1,3]],[\"Fraction\",[1,2]],[\"String\",\"test str\"]]"
                .to_owned(),
            res
        );

        // List
        let value_12 = Fraction::new(1, 2).to_value();
        let send_value_12 = value_12.try_into_send_value::<Fraction>().unwrap();

        let value_str = "test str".to_value();
        let send_value_str = value_str.try_into_send_value::<String>().unwrap();

        let list = List::new(&[&send_value_12, &send_value_str]);

        let res = ron::ser::to_string_pretty(&list, pretty_config.clone());
        assert_eq!(
            Ok(
                concat!(
                    "[",
                    "    (\"Fraction\", (1, 2)),",
                    "    (\"String\", \"test str\"),",
                    "]"
                )
                    .to_owned()
            ),
            res,
        );
    }

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_deserialize_simple() {
        extern crate ron;
        extern crate serde_json;

        use Fraction;
        use FractionRange;
        use IntRange;
        use Bitmask;

        ::init().unwrap();

        // Fraction
        let fraction_ron = "(1, 3)";
        let fraction: Fraction = ron::de::from_str(fraction_ron).unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        let fraction_json = "[1,3]";
        let fraction: Fraction = serde_json::from_str(fraction_json).unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        // FractionRange
        let fraction_range_ron = "(min: (1, 3), max: (1, 2))";
        let fraction_range: FractionRange = ron::de::from_str(fraction_range_ron).unwrap();
        assert_eq!(fraction_range.min().0.denom(), &3);
        assert_eq!(fraction_range.max().0.denom(), &2);

        let fraction_range_json = "{\"min\":[1,3],\"max\":[1,2]}";
        let fraction_range: FractionRange = serde_json::from_str(fraction_range_json).unwrap();
        assert_eq!(fraction_range.min().0.denom(), &3);
        assert_eq!(fraction_range.max().0.denom(), &2);

        // IntRange
        let int_range_ron = "(min: 0, max: 42, step: 21)";
        let int_range: IntRange<i32> = ron::de::from_str(int_range_ron).unwrap();
        assert_eq!(int_range.min(), 0);
        assert_eq!(int_range.max(), 42);
        assert_eq!(int_range.step(), 21);

        let int_range_json = "{\"min\":0,\"max\":42,\"step\":21}";
        let int_range: IntRange<i32> = serde_json::from_str(int_range_json).unwrap();
        assert_eq!(int_range.min(), 0);
        assert_eq!(int_range.max(), 42);
        assert_eq!(int_range.step(), 21);

        // Bitmask
        let bitmask_ref = Bitmask::new(1024 + 128 + 32);

        let bitmask_ron = "(1184)";
        let bitmask: Bitmask = ron::de::from_str(bitmask_ron).unwrap();
        assert_eq!(bitmask_ref, bitmask);

        let bitmask_json = "1184";
        let bitmask: Bitmask = serde_json::from_str(bitmask_json).unwrap();
        assert_eq!(bitmask_ref, bitmask);
    }

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_deserialize_collections() {
        extern crate ron;
        extern crate serde_json;

        use Array;
        use Fraction;
        use List;

        ::init().unwrap();

        // Array
        let array_ron =
            r#"[
                ("Fraction", (1, 3)),
                ("Fraction", (1, 2)),
                ("String", "test str"),
            ]"#;
        let array: Array = ron::de::from_str(array_ron).unwrap();
        let slice = array.as_slice();
        assert_eq!(3, slice.len());

        let fraction = slice[0].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        let fraction = slice[1].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &2);

        assert_eq!("test str".to_owned(), slice[2].get::<String>().unwrap());

        let array_json =
            r#"[["Fraction",[1,3]],["Fraction",[1,2]],["String","test str"]]"#;
        let array: Array = serde_json::from_str(array_json).unwrap();
        let slice = array.as_slice();
        assert_eq!(3, slice.len());

        let fraction = slice[0].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &3);

        let fraction = slice[1].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &2);

        assert_eq!("test str".to_owned(), slice[2].get::<String>().unwrap());

        // List
        let list_ron =
            r#"[
                ("Fraction", (1, 2)),
                ("String", "test str"),
            ]"#;
        let list: List = ron::de::from_str(list_ron).unwrap();
        let slice = list.as_slice();
        assert_eq!(2, slice.len());

        let fraction = slice[0].get::<Fraction>().unwrap();
        assert_eq!(fraction.0.numer(), &1);
        assert_eq!(fraction.0.denom(), &2);

        assert_eq!("test str".to_owned(), slice[1].get::<String>().unwrap());
    }
}