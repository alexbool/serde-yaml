// Copyright 2016 Serde YAML Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! YAML Serialization
//!
//! This module provides YAML serialization with the type `Serializer`.

use std::{fmt, io, mem};

use yaml_rust::{Yaml, YamlEmitter};
use yaml_rust::yaml;

use serde::ser;

use super::error::{Error, Result};

/// A structure for serializing a Rust value into a YAML value.
pub struct Serializer {
    /// The YAML value to hold the result.
    doc: Yaml,
}

impl Serializer {
    pub fn new() -> Self {
        Serializer {
            doc: Yaml::Null,
        }
    }

    pub fn take(self) -> Yaml {
        self.doc
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Serializer::new()
    }
}

impl ser::Serializer for Serializer {
    type Error = Error;
    type SeqState = yaml::Array;
    type TupleState = yaml::Array;
    type TupleStructState = yaml::Array;
    type TupleVariantState = (&'static str, yaml::Array);
    type MapState = (Option<yaml::Yaml>, yaml::Hash);
    type StructState = (Option<yaml::Yaml>, yaml::Hash);
    type StructVariantState = (&'static str, (Option<yaml::Yaml>, yaml::Hash));

    fn serialize_bool(&mut self, v: bool) -> Result<()> {
        self.doc = Yaml::Boolean(v);
        Ok(())
    }

    fn serialize_isize(&mut self, v: isize) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i8(&mut self, v: i8) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(&mut self, v: i16) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(&mut self, v: i32) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(&mut self, v: i64) -> Result<()> {
        self.doc = Yaml::Integer(v);
        Ok(())
    }

    fn serialize_usize(&mut self, v: usize) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u8(&mut self, v: u8) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u16(&mut self, v: u16) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u32(&mut self, v: u32) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u64(&mut self, v: u64) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(&mut self, v: f32) -> Result<()> {
        self.doc = Yaml::Real(v.to_string());
        Ok(())
    }

    fn serialize_f64(&mut self, v: f64) -> Result<()> {
        self.doc = Yaml::Real(v.to_string());
        Ok(())
    }

    fn serialize_char(&mut self, value: char) -> Result<()> {
        self.doc = Yaml::String(value.to_string());
        Ok(())
    }

    fn serialize_str(&mut self, value: &str) -> Result<()> {
        self.doc = Yaml::String(String::from(value));
        Ok(())
    }

    fn serialize_bytes(&mut self, value: &[u8]) -> Result<()> {
        let mut state = try!(self.serialize_seq(Some(value.len())));
        for c in value {
            try!(self.serialize_seq_elt(&mut state, c));
        }
        self.serialize_seq_end(state)
    }

    fn serialize_unit(&mut self) -> Result<()> {
        self.doc = Yaml::Null;
        Ok(())
    }

    fn serialize_unit_struct(&mut self, _name: &'static str) -> Result<()> {
        self.doc = Yaml::Null;
        Ok(())
    }

    fn serialize_unit_variant(
        &mut self,
        _name: &str,
        _variant_index: usize,
        variant: &str
    ) -> Result<()> {
        self.doc = Yaml::String(String::from(variant));
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        &mut self,
        _name: &'static str,
        value: T
    ) -> Result<()>
        where T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        &mut self,
        _name: &str,
        _variant_index: usize,
        variant: &str,
        value: T
    ) -> Result<()>
        where T: ser::Serialize,
    {
        self.doc = singleton_hash(try!(to_yaml(variant)), try!(to_yaml(value)));
        Ok(())
    }

    fn serialize_none(&mut self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<V>(&mut self, value: V) -> Result<()>
        where V: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(&mut self, len: Option<usize>) -> Result<yaml::Array> {
        Ok(match len {
            None => yaml::Array::new(),
            Some(len) => yaml::Array::with_capacity(len),
        })
    }

    fn serialize_seq_elt<T>(
        &mut self,
        state: &mut yaml::Array,
        elem: T
    ) -> Result<()>
        where T: ser::Serialize,
    {
        state.push(try!(to_yaml(elem)));
        Ok(())
    }

    fn serialize_seq_end(&mut self, state: yaml::Array) -> Result<()> {
        self.doc = Yaml::Array(state);
        Ok(())
    }

    fn serialize_seq_fixed_size(&mut self, len: usize) -> Result<yaml::Array> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple(&mut self, len: usize) -> Result<yaml::Array> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_elt<T>(
        &mut self,
        state: &mut yaml::Array,
        elem: T
    ) -> Result<()>
        where T: ser::Serialize,
    {
        self.serialize_seq_elt(state, elem)
    }

    fn serialize_tuple_end(&mut self, state: yaml::Array) -> Result<()> {
        self.serialize_seq_end(state)
    }

    fn serialize_tuple_struct(
        &mut self,
        _name: &'static str,
        len: usize
    ) -> Result<yaml::Array> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct_elt<V>(
        &mut self,
        state: &mut yaml::Array,
        value: V
    ) -> Result<()>
        where V: ser::Serialize,
    {
        self.serialize_seq_elt(state, value)
    }

    fn serialize_tuple_struct_end(&mut self, state: yaml::Array) -> Result<()> {
        self.serialize_seq_end(state)
    }

    fn serialize_tuple_variant(
        &mut self,
        _enum: &'static str,
        _idx: usize,
        variant: &'static str,
        len: usize
    ) -> Result<(&'static str, yaml::Array)> {
        let state = try!(self.serialize_seq(Some(len)));
        Ok((variant, state))
    }

    fn serialize_tuple_variant_elt<V>(
        &mut self,
        state: &mut (&'static str, yaml::Array),
        v: V
    ) -> Result<()>
        where V: ser::Serialize,
    {
        self.serialize_seq_elt(&mut state.1, v)
    }

    fn serialize_tuple_variant_end(
        &mut self,
        state: (&'static str, yaml::Array)
    ) -> Result<()> {
        try!(self.serialize_seq_end(state.1));
        self.doc = singleton_hash(try!(to_yaml(state.0)),
                                  mem::replace(&mut self.doc, Yaml::Null));
        Ok(())
    }

    fn serialize_map(&mut self, _len: Option<usize>) -> Result<(Option<yaml::Yaml>, yaml::Hash)> {
        Ok((None, yaml::Hash::new()))
    }

    fn serialize_map_key<T>(
        &mut self,
        state: &mut (Option<yaml::Yaml>, yaml::Hash),
        key: T
    ) -> Result<()>
        where T: ser::Serialize
    {
        state.0 = Some(try!(to_yaml(key)));
        Ok(())
    }

    fn serialize_map_value<T>(
        &mut self,
        state: &mut (Option<yaml::Yaml>, yaml::Hash),
        value: T
    ) -> Result<()>
        where T: ser::Serialize
    {
        match state.0.take() {
            Some(key) => state.1.insert(key, try!(to_yaml(value))),
            None => {
                return Err(Error::Custom("serialize_map_value called without matching \
                                          serialize_map_key call".to_owned()));
            }
        };
        Ok(())
    }

    fn serialize_map_end(&mut self, state: (Option<yaml::Yaml>, yaml::Hash)) -> Result<()> {
        self.doc = Yaml::Hash(state.1);
        Ok(())
    }

    fn serialize_struct(
        &mut self,
        _name: &'static str,
        len: usize
    ) -> Result<(Option<yaml::Yaml>, yaml::Hash)> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_elt<V>(
        &mut self,
        state: &mut (Option<yaml::Yaml>, yaml::Hash),
        key: &'static str,
        value: V
    ) -> Result<()>
        where V: ser::Serialize,
    {
        try!(self.serialize_map_key(state, key));
        self.serialize_map_value(state, value)
    }

    fn serialize_struct_end(&mut self, state: (Option<yaml::Yaml>, yaml::Hash)) -> Result<()> {
        self.serialize_map_end(state)
    }

    fn serialize_struct_variant(
        &mut self,
        _enum: &'static str,
        _idx: usize,
        variant: &'static str,
        len: usize
    ) -> Result<(&'static str, (Option<yaml::Yaml>, yaml::Hash))> {
        let state = try!(self.serialize_map(Some(len)));
        Ok((variant, state))
    }

    fn serialize_struct_variant_elt<V>(
        &mut self,
        state: &mut (&'static str, (Option<yaml::Yaml>, yaml::Hash)),
        field: &'static str,
        v: V
    ) -> Result<()>
        where V: ser::Serialize,
    {
        try!(self.serialize_map_key(&mut state.1, field));
        self.serialize_map_value(&mut state.1, v)
    }

    fn serialize_struct_variant_end(
        &mut self,
        state: (&'static str, (Option<yaml::Yaml>, yaml::Hash))
    ) -> Result<()> {
        try!(self.serialize_map_end(state.1));
        self.doc = singleton_hash(try!(to_yaml(state.0)),
                                  mem::replace(&mut self.doc, Yaml::Null));
        Ok(())
    }
}

pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
    where W: io::Write,
          T: ser::Serialize,
{
    let doc = try!(to_yaml(value));
    let mut writer_adapter = FmtToIoWriter {
        writer: writer,
    };
    try!(YamlEmitter::new(&mut writer_adapter).dump(&doc));
    Ok(())
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
    where T: ser::Serialize,
{
    let mut vec = Vec::with_capacity(128);
    try!(to_writer(&mut vec, value));
    Ok(vec)
}

pub fn to_string<T>(value: &T) -> Result<String>
    where T: ser::Serialize,
{
    Ok(try!(String::from_utf8(try!(to_vec(value)))))
}

/// The yaml-rust library uses `fmt.Write` intead of `io.Write` so this is a
/// simple adapter.
struct FmtToIoWriter<'a, W>
    where W: io::Write + 'a,
{
    writer: &'a mut W,
}

impl<'a, W> fmt::Write for FmtToIoWriter<'a, W>
    where W: io::Write + 'a,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.writer.write(s.as_bytes()).is_err() {
            return Err(fmt::Error);
        }
        Ok(())
    }
}

fn to_yaml<T>(elem: T) -> Result<Yaml>
    where T: ser::Serialize,
{
    let mut ser = Serializer::new();
    try!(elem.serialize(&mut ser));
    Ok(ser.take())
}

fn singleton_hash(k: Yaml, v: Yaml) -> Yaml {
    let mut hash = yaml::Hash::new();
    hash.insert(k, v);
    Yaml::Hash(hash)
}
