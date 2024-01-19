use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use serde::{ser, Serialize};
use crate::value::{NamedStruct, NamedVariant, TupleStruct, TupleVariant, UnitStruct, UnitVariant, Value};

/// The default serializer for intermediate values
pub struct Serializer;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl ser::StdError for Error {}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self where T: Display {
        Error {
            message: format!("{msg}"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = SerializeCompound<'a>;
    type SerializeTuple = SerializeCompound<'a>;
    type SerializeTupleStruct = SerializeTupleStruct<'a>;
    type SerializeTupleVariant = SerializeTupleVariant<'a>;
    type SerializeMap = SerializeMap<'a>;
    type SerializeStruct = SerializeStruct<'a>;
    type SerializeStructVariant = SerializeStructVariant<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(v.into())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UnitStruct(UnitStruct {
            name: "None",
        }))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        let inner = value.serialize(&mut *self)?;
        Ok(Value::TupleStruct(TupleStruct {
            name: "Some",
            values: vec![inner],
        }))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UnitStruct(UnitStruct {
            name,
        }))
    }

    fn serialize_unit_variant(self, name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UnitVariant(UnitVariant {
            name,
            variant,
        }))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        Ok(Value::TupleStruct(TupleStruct {
            name,
            values: vec![value.serialize(&mut *self)?],
        }))
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: Serialize {
        Ok(Value::TupleVariant(TupleVariant {
            name,
            variant,
            values: vec![value.serialize(&mut *self)?],
        }))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeCompound {
            serializer: self,
            inner: len.map(Vec::with_capacity).unwrap_or_default(),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeCompound {
            serializer: self,
            inner: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct {
            serializer: self,
            name,
            values: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_variant(self, name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant {
            serializer: self,
            name,
            variant,
            values: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            serializer: self,
            key: None,
            values: BTreeMap::new(),
        })
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct {
            serializer: self,
            name,
            fields: BTreeMap::new(),
        })
    }

    fn serialize_struct_variant(self, name: &'static str, _variant_index: u32, variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant {
            serializer: self,
            name,
            variant,
            fields: BTreeMap::new(),
        })
    }
}

pub struct SerializeCompound<'a> {
    inner: Vec<Value>,
    serializer: &'a mut Serializer,
}

impl ser::SerializeSeq for SerializeCompound<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.push(value.serialize(&mut *self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Seq(self.inner))
    }
}

impl ser::SerializeTuple for SerializeCompound<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.inner.push(value.serialize(&mut *self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Tuple(self.inner))
    }
}

pub struct SerializeTupleStruct<'a> {
    serializer: &'a mut Serializer,
    name: &'static str,
    values: Vec<Value>,
}

impl ser::SerializeTupleStruct for SerializeTupleStruct<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.values.push(value.serialize(&mut *self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleStruct(TupleStruct {
            name: self.name,
            values: self.values,
        }))
    }
}

pub struct SerializeTupleVariant<'a> {
    serializer: &'a mut Serializer,
    name: &'static str,
    variant: &'static str,
    values: Vec<Value>,
}

impl ser::SerializeTupleVariant for SerializeTupleVariant<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        self.values.push(value.serialize(&mut *self.serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleVariant(TupleVariant {
            name: self.name,
            variant: self.variant,
            values: self.values,
        }))
    }
}

pub struct SerializeStruct<'a> {
    serializer: &'a mut Serializer,
    name: &'static str,
    fields: BTreeMap<&'static str, Value>,
}

impl ser::SerializeStruct for SerializeStruct<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        let value = value.serialize(&mut *self.serializer)?;
        self.fields.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::NamedStruct(NamedStruct {
            name: self.name,
            fields: self.fields,
        }))
    }
}

pub struct SerializeStructVariant<'a> {
    serializer: &'a mut Serializer,
    name: &'static str,
    variant: &'static str,
    fields: BTreeMap<&'static str, Value>,
}

impl ser::SerializeStructVariant for SerializeStructVariant<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        let value = value.serialize(&mut *self.serializer)?;
        self.fields.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::NamedVariant(NamedVariant {
            name: self.name,
            variant: self.variant,
            fields: self.fields,
        }))
    }
}

pub struct SerializeMap<'a> {
    serializer: &'a mut Serializer,
    key: Option<Value>,
    values: BTreeMap<Value, Value>,
}

impl ser::SerializeMap for SerializeMap<'_> {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where T: Serialize {
        let key = key.serialize(&mut *self.serializer)?;
        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        let key = self.key.take().expect("stupid ordering!!!");
        let value = value.serialize(&mut *self.serializer)?;
        self.values.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.values))
    }
}