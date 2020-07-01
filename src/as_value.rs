use std::collections::HashMap;
use std::error;
use std::fmt;

use serde::{
    ser::{self, Error as SerdeError},
    Serialize,
};

use super::value::{self, ToHessian, Value};

// AsHessian Serializer
#[derive(Clone, Default)]
struct Serializer {}

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub message: String,
}

impl SerdeError for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.to_string())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

struct SeqSerializer<'a> {
    name: Option<&'a str>,
    items: Vec<Value>,
}

struct MapSerializer {
    keys: Vec<Value>,
    values: Vec<Value>,
}

struct StructSerializer<'a> {
    name: &'a str,
    fields: Vec<String>,
    values: Vec<Value>,
}

impl<'a> ser::SerializeSeq for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.items
            .push(value.serialize(&mut Serializer::default())?);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Error> {
        match self.name {
            Some(name) => Ok(Value::List(value::List::from((name, self.items)))),
            None => Ok(Value::List(value::List::from(self.items))),
        }
    }
}

impl<'a> ser::SerializeTuple for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleStruct for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        ser::SerializeTuple::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleVariant for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        ser::SerializeTuple::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeMap for MapSerializer {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> {
        self.keys.push(key.serialize(&mut Serializer::default())?);
        Ok(())
    }

    #[inline]
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.values
            .push(value.serialize(&mut Serializer::default())?);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut map = HashMap::new();
        for (k, v) in self.keys.iter().zip(self.values.iter()) {
            map.insert(k.clone(), v.clone());
        }
        Ok(Value::Map(value::Map::from(map)))
    }
}

// TODO: Add struct type for Value
impl<'a> ser::SerializeStruct for StructSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.fields.push(key.into());
        self.values
            .push(value.serialize(&mut Serializer::default())?);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Self::Error> {
        let mut map = HashMap::new();
        for (k, v) in self.fields.iter().zip(self.values.iter()) {
            map.insert(k.to_hessian(), v.clone());
        }
        Ok(Value::Map(value::Map::from((self.name, map))))
    }
}

impl<'a> ser::SerializeStructVariant for StructSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        ser::SerializeStruct::serialize_field(self, key, value)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value, Self::Error> {
        ser::SerializeStruct::end(self)
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = Self::SerializeSeq;
    type SerializeTupleStruct = Self::SerializeTuple;
    type SerializeTupleVariant = Self::SerializeTuple;
    type SerializeMap = MapSerializer;
    type SerializeStruct = StructSerializer<'a>;
    type SerializeStructVariant = Self::SerializeStruct;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_hessian())
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        Ok((value as i32).to_hessian())
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        Ok((value as i32).to_hessian())
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_hessian())
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_hessian())
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        Ok((value as i32).to_hessian())
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        Ok((value as i32).to_hessian())
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        if value < i32::max_value() as u32 {
            Ok((value as i32).to_hessian())
        } else {
            Ok((value as i64).to_hessian())
        }
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        Ok((value as i64).to_hessian())
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        Ok((value as f64).to_hessian())
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_hessian())
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        Ok(value.encode_utf8(&mut buf).to_hessian())
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_hessian())
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_hessian())
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(().to_hessian())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(len) => Ok(SeqSerializer {
                name: None,
                items: Vec::with_capacity(len),
            }),
            _ => Ok(SeqSerializer {
                name: None,
                items: Vec::new(),
            }),
        }
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SeqSerializer {
            name: None,
            items: Vec::with_capacity(len),
        })
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SeqSerializer {
            name: Some(name),
            items: Vec::with_capacity(len),
        })
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SeqSerializer {
            name: Some(variant),
            items: Vec::with_capacity(len),
        })
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match len {
            Some(len) => Ok(MapSerializer {
                keys: Vec::with_capacity(len),
                values: Vec::with_capacity(len),
            }),
            None => Ok(MapSerializer {
                keys: Vec::new(),
                values: Vec::new(),
            }),
        }
    }

    #[inline]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer {
            name,
            fields: Vec::with_capacity(len),
            values: Vec::with_capacity(len),
        })
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructSerializer {
            name: variant,
            fields: Vec::with_capacity(len),
            values: Vec::with_capacity(len),
        })
    }
}

impl serde::Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match *self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(b),
            Value::Int(i) => serializer.serialize_i32(i),
            Value::Long(l) => serializer.serialize_i64(l),
            Value::Double(d) => serializer.serialize_f64(d),
            Value::Date(d) => serializer.serialize_i64(d),
            Value::Bytes(ref bytes) => serializer.serialize_bytes(bytes),
            Value::String(ref s) => serializer.serialize_str(s),
            Value::Ref(i) => serializer.serialize_i32(i as i32),
            Value::List(ref l) => {
                match *l {
                    value::List::Typed(name, v) => {
                    let ser = serializer.serialize_seq(Some(v.len()))?;
                    for e in v {
                        ser.serialize_element(e)?;
                    }
                    seq.end()
                    }
                    value::List::Untyped(v) => {
                    let ser = serializer.serialize_seq(Some(v.len()))?;
                    for e in v {
                        ser.serialize_element(e)?;
                    }
                    seq.end()
                    }
                }
            }
            Value::Map(ref m) => {
                Error("test".into())
            }
        }
    }
}

pub fn to_value<S: Serialize>(value: S) -> Result<Value, Error> {
    let mut serializer = Serializer::default();
    value.serialize(&mut serializer)
}
