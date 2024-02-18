//! Functions to serialize an array to a raw string* for PostgreSQL COPY Command.
//!
//! Use external crates(e.g, csv) to serialize the serialized string to a csv string.
//! 
//! \* This crate does NOT care special characters: e.g, ``/[,'"]/``

use core::fmt;
use core::fmt::Display;

use std::io;

use serde::ser::{SerializeMap, SerializeSeq};
use serde::ser::{SerializeStruct, SerializeStructVariant};
use serde::ser::{SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};

use serde::{Serialize, Serializer};

struct Ser<W> {
    wtr: W,
    prev: Option<char>,
}

#[derive(Debug)]
pub enum SerError {
    Msg(String),
    WriteErr(String),
}

impl Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Msg(msg) => write!(f, "{msg}"),
            Self::WriteErr(err) => write!(f, "write error: {err}"),
        }
    }
}

impl std::error::Error for SerError {}

impl serde::ser::Error for SerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Msg(msg.to_string())
    }
}

impl<'a, W> SerializeSeq for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T>(&mut self, val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        let prefix: &str = match self.prev {
            None => "",
            Some('{') => "",
            Some(_) => ",",
        };
        self.prev = match self.prev {
            None => Some('{'),
            Some(_) => Some('.'),
        };
        prefix.serialize(&mut **self)?;
        val.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let suffix: &str = "}";
        write!(self.wtr, "{suffix}").map_err(|e| SerError::WriteErr(format!("{e}")))?;
        Ok(())
    }
}

impl<'a, W> SerializeTuple for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = SerError;
    fn serialize_element<T>(&mut self, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, W> SerializeTupleStruct for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = SerError;
    fn serialize_field<T>(&mut self, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, W> SerializeTupleVariant for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = SerError;
    fn serialize_field<T>(&mut self, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, W> SerializeStruct for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = SerError;
    fn serialize_field<T>(&mut self, _key: &'static str, val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(&mut **self)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        writeln!(self.wtr).map_err(|e| SerError::WriteErr(format!("{e}")))
    }
}

impl<'a, W> SerializeStructVariant for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = SerError;
    fn serialize_field<T>(&mut self, _key: &'static str, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, W> SerializeMap for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = SerError;
    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }
    fn serialize_value<T>(&mut self, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        unimplemented!()
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, W> Serializer for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = SerError;

    type SerializeSeq = Self;
    type SerializeMap = Self;

    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let serialized: &str = match v {
            true => "true",
            false => "false",
        };
        write!(self.wtr, "{serialized}").map_err(|e| SerError::WriteErr(format!("{e}")))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v.into())
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write!(self.wtr, "{v}").map_err(|e| SerError::WriteErr(format!("{e}")))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.into())
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write!(self.wtr, "{v}").map_err(|e| SerError::WriteErr(format!("{e}")))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write!(self.wtr, "{v}").map_err(|e| SerError::WriteErr(format!("{e}")))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        write!(self.wtr, "{v}").map_err(|e| SerError::WriteErr(format!("{e}")))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        write!(self.wtr, "{v}").map_err(|e| SerError::WriteErr(format!("{e}")))
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_some<T>(self, t: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        t.serialize(self)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _vix: u32,
        varname: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(varname)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        val: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _vix: u32,
        _var: &'static str,
        val: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let prefix: &str = "{";
        self.prev = Some('{');
        write!(self.wtr, "{prefix}").map_err(|e| SerError::WriteErr(format!("{e}")))?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _vix: u32,
        _var: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _vix: u32,
        _var: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

/// Writes a val to a writer.
///
/// ## Arguments
/// - wtr: A target [`io::Write`].
/// - val: A value to be serialized([`Serialize`]).
pub fn to_writer<W, T>(wtr: W, val: &T) -> Result<(), SerError>
where
    T: Serialize,
    W: io::Write,
{
    let mut ser = Ser { wtr, prev: None };
    val.serialize(&mut ser)
}

#[cfg(test)]
mod test_arr {

    mod to_writer {

        mod arr {

            #[test]
            fn empty() {
                let v: Vec<i16> = vec![];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{}");
            }

            #[test]
            fn single() {
                let v: Vec<i16> = vec![42];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{42}");
            }

            #[test]
            fn double() {
                let v: Vec<i16> = vec![599, 3776];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{599,3776}");
            }

            #[test]
            fn triple() {
                let v: Vec<i16> = vec![42, 333, 634];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{42,333,634}");
            }
        }

        mod arr2 {

            #[test]
            fn empty1() {
                let v: Vec<Vec<i16>> = vec![vec![]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{}}");
            }

            #[test]
            fn single1() {
                let v: Vec<Vec<i16>> = vec![vec![42]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{42}}");
            }

            #[test]
            fn single2() {
                let v: Vec<Vec<i16>> = vec![vec![333, 634]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{333,634}}");
            }

            #[test]
            fn double1() {
                let v: Vec<Vec<i16>> = vec![vec![333], vec![634]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{333},{634}}");
            }

            #[test]
            fn double2() {
                let v: Vec<Vec<i16>> = vec![vec![333, 634], vec![599, 3776]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{333,634},{599,3776}}");
            }

            #[test]
            fn triple1() {
                let v: Vec<Vec<i16>> = vec![vec![42], vec![333], vec![634]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{42},{333},{634}}");
            }

            #[test]
            fn triple2() {
                let v: Vec<Vec<i16>> = vec![vec![599, 3776], vec![333, 634], vec![0, 1]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{599,3776},{333,634},{0,1}}");
            }
        }

        mod arr3 {

            #[test]
            fn empty() {
                let v: Vec<Vec<Vec<i16>>> = vec![];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{}");
            }

            #[test]
            fn empty1() {
                let v: Vec<Vec<Vec<i16>>> = vec![vec![vec![]]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{{}}}");
            }

            #[test]
            fn single1i() {
                let v: Vec<Vec<Vec<i16>>> = vec![vec![vec![42]]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{{42}}}");
            }

            #[test]
            fn single1ii() {
                let v: Vec<Vec<Vec<i16>>> = vec![vec![vec![333, 634]]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{{333,634}}}");
            }

            #[test]
            fn single2ii() {
                let v: Vec<Vec<Vec<i16>>> = vec![vec![vec![333, 634], vec![599, 3776]]];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{{333,634},{599,3776}}}");
            }

            #[test]
            fn double2ii() {
                let v: Vec<Vec<Vec<i32>>> = vec![
                    vec![vec![333, 634], vec![599, 3776]],
                    vec![vec![42, 42195], vec![314, 299792458]],
                ];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                assert_eq!(s, "{{{333,634},{599,3776}},{{42,42195},{314,299792458}}}");
            }

            #[test]
            fn triple4v() {
                let v: Vec<Vec<Vec<i32>>> = vec![
                    vec![
                        vec![0, 1, 2, 3, 4],
                        vec![2, 3, 5, 7, 11],
                        vec![2, 4, 6, 8, 10],
                        vec![4, 2, 1, 9, 5],
                    ],
                    vec![
                        vec![10, 1, 2, 3, 4],
                        vec![12, 3, 5, 7, 11],
                        vec![12, 4, 6, 8, 10],
                        vec![14, 2, 1, 9, 5],
                    ],
                    vec![
                        vec![20, 1, 2, 3, 4],
                        vec![22, 3, 5, 7, 11],
                        vec![22, 4, 6, 8, 10],
                        vec![24, 2, 1, 9, 5],
                    ],
                ];
                let mut wtr: Vec<u8> = vec![];
                crate::arr::to_writer(&mut wtr, &v).unwrap();
                let s: String = String::from_utf8(wtr).unwrap();
                let expected = r#"{
                    {
                        {0,1,2,3,4},
                        {2,3,5,7,11},
                        {2,4,6,8,10},
                        {4,2,1,9,5}
                    },
                    {
                        {10,1,2,3,4},
                        {12,3,5,7,11},
                        {12,4,6,8,10},
                        {14,2,1,9,5}
                    },
                    {
                        {20,1,2,3,4},
                        {22,3,5,7,11},
                        {22,4,6,8,10},
                        {24,2,1,9,5}
                    }
                }"#;
                assert_eq!(s, expected.replace(['\n', ' '], ""));
            }
        }
    }
}
