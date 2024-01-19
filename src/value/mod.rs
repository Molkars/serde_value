mod from;

mod number;

use std::collections::BTreeMap;
pub use number::Number;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Value {
    Unit,
    Bool(bool),
    Char(char),
    Number(Number),
    String(String),
    Seq(Vec<Value>),
    Map(BTreeMap<Value, Value>),
    Tuple(Vec<Value>),
    UnitStruct(UnitStruct),
    TupleStruct(TupleStruct),
    NamedStruct(NamedStruct),
    UnitVariant(UnitVariant),
    TupleVariant(TupleVariant),
    NamedVariant(NamedVariant),
}

impl Value {
    #[inline]
    pub fn is_unit(&self) -> bool {
        matches!(self, Value::Unit)
    }

    #[inline]
    pub fn as_unit(&self) -> Option<()> {
        self.is_unit().then_some(())
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(val) => Some(*val),
            _ => None
        }
    }

    #[inline]
    pub fn is_char(&self) -> bool {
        matches!(self, Value::Char(_))
    }

    #[inline]
    pub fn as_char(&self) -> Option<char> {
        match self {
            Value::Char(val) => Some(*val),
            _ => None
        }
    }

    #[inline]
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    #[inline]
    pub fn as_number(&self) -> Option<Number> {
        match self {
            Self::Number(num) => Some(*num),
            _ => None
        }
    }

    #[inline]
    pub fn is_u8(&self) -> bool {
        matches!(self, Value::Number(Number::U8(_)))
    }

    #[inline]
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Self::Number(Number::U8(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_u16(&self) -> bool {
        matches!(self, Value::Number(Number::U16(_)))
    }

    #[inline]
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Self::Number(Number::U16(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_u32(&self) -> bool {
        matches!(self, Value::Number(Number::U32(_)))
    }

    #[inline]
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Self::Number(Number::U32(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_u64(&self) -> bool {
        matches!(self, Value::Number(Number::U64(_)))
    }

    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(Number::U64(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_u128(&self) -> bool {
        matches!(self, Value::Number(Number::U128(_)))
    }

    #[inline]
    pub fn as_u128(&self) -> Option<u128> {
        match self {
            Self::Number(Number::U128(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i8(&self) -> bool {
        matches!(self, Value::Number(Number::I8(_)))
    }

    #[inline]
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Self::Number(Number::I8(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i16(&self) -> bool {
        matches!(self, Value::Number(Number::I16(_)))
    }

    #[inline]
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Self::Number(Number::I16(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i32(&self) -> bool {
        matches!(self, Value::Number(Number::I32(_)))
    }

    #[inline]
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Self::Number(Number::I32(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i64(&self) -> bool {
        matches!(self, Value::Number(Number::I64(_)))
    }

    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Number(Number::I64(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_i128(&self) -> bool {
        matches!(self, Value::Number(Number::I128(_)))
    }

    #[inline]
    pub fn as_i128(&self) -> Option<i128> {
        match self {
            Self::Number(Number::I128(val)) => Some(*val),
            _ => None,
        }
    }

    #[inline]
    pub fn is_f32(&self) -> bool {
        matches!(self, Value::Number(Number::F32(_)))
    }

    #[inline]
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Self::Number(Number::F32(val)) => Some(val.0),
            _ => None,
        }
    }

    #[inline]
    pub fn is_f64(&self) -> bool {
        matches!(self, Value::Number(Number::F64(_)))
    }

    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(Number::F64(val)) => Some(val.0),
            _ => None,
        }
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    #[inline]
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None
        }
    }

    #[inline]
    pub fn is_seq(&self) -> bool {
        matches!(self, Self::Seq(_))
    }

    #[inline]
    pub fn as_seq(&self) -> Option<&[Value]> {
        match self {
            Self::Seq(vals) => Some(vals.as_slice()),
            _ => None,
        }
    }

    #[inline]
    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(_))
    }

    pub fn as_map(&self) -> Option<&BTreeMap<Value, Value>> {
        match self {
            Self::Map(map) => Some(map),
            _ => None,
        }
    }

    #[inline]
    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple(_))
    }

    pub fn as_tuple(&self) -> Option<&[Value]> {
        match self {
            Self::Tuple(values) => Some(values.as_slice()),
            _ => None,
        }
    }

    #[inline]
    pub fn is_unit_struct(&self) -> bool {
        matches!(self, Self::UnitStruct(_))
    }

    #[inline]
    pub fn is_tuple_struct(&self) -> bool {
        matches!(self, Self::TupleStruct(_))
    }

    #[inline]
    pub fn is_named_struct(&self) -> bool {
        matches!(self, Self::NamedStruct(_))
    }

    #[inline]
    pub fn is_unit_variant(&self) -> bool {
        matches!(self, Self::UnitVariant(_))
    }

    #[inline]
    pub fn is_tuple_variant(&self) -> bool {
        matches!(self, Self::TupleVariant(_))
    }

    #[inline]
    pub fn is_named_variant(&self) -> bool {
        matches!(self, Self::NamedVariant(_))
    }

    pub fn as_unit_struct(&self) -> Option<&UnitStruct> {
        match self {
            Self::UnitStruct(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_tuple_struct(&self) -> Option<&TupleStruct> {
        match self {
            Self::TupleStruct(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_named_struct(&self) -> Option<&NamedStruct> {
        match self {
            Self::NamedStruct(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_unit_variant(&self) -> Option<&UnitVariant> {
        match self {
            Self::UnitVariant(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_tuple_variant(&self) -> Option<&TupleVariant> {
        match self {
            Self::TupleVariant(val) => Some(val),
            _ => None,
        }
    }
    pub fn as_named_variant(&self) -> Option<&NamedVariant> {
        match self {
            Self::NamedVariant(val) => Some(val),
            _ => None,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnitStruct {
    pub name: &'static str,
}

#[non_exhaustive]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TupleStruct {
    pub name: &'static str,
    pub values: Vec<Value>,
}

#[non_exhaustive]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamedStruct {
    pub name: &'static str,
    pub fields: BTreeMap<&'static str, Value>,
}

#[non_exhaustive]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnitVariant {
    pub name: &'static str,
    pub variant: &'static str,
}

#[non_exhaustive]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TupleVariant {
    pub name: &'static str,
    pub variant: &'static str,
    pub values: Vec<Value>,
}

#[non_exhaustive]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamedVariant {
    pub name: &'static str,
    pub variant: &'static str,
    pub fields: BTreeMap<&'static str, Value>,
}

mod debug {
    use std::fmt::{Debug, Formatter};
    use crate::value::{NamedStruct, NamedVariant, TupleStruct, TupleVariant, UnitStruct, UnitVariant, Value};

    impl Debug for Value {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Value::Unit => write!(f, "{:?}", ()),
                Value::Bool(val) => write!(f, "{val:?}"),
                Value::Char(val) => write!(f, "{val:?}"),
                Value::Number(val) => write!(f, "{val:?}"),
                Value::String(val) => write!(f, "{val:?}"),
                Value::Seq(items) => {
                    let mut builder = f.debug_list();
                    for item in items {
                        builder.entry(item);
                    }
                    builder.finish()
                }
                Value::Map(map) => {
                    let mut builder = f.debug_map();
                    for (k, v) in map {
                        builder.entry(k, v);
                    }
                    builder.finish()
                }
                Value::Tuple(items) => {
                    let mut builder = f.debug_tuple("");
                    for item in items {
                        builder.field(item);
                    }
                    builder.finish()
                }
                Value::UnitStruct(val) => write!(f, "{val:?}"),
                Value::TupleStruct(val) => write!(f, "{val:?}"),
                Value::NamedStruct(val) => write!(f, "{val:?}"),
                Value::UnitVariant(val) => write!(f, "{val:?}"),
                Value::TupleVariant(val) => write!(f, "{val:?}"),
                Value::NamedVariant(val) => write!(f, "{val:?}"),
            }
        }
    }

    impl Debug for UnitStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.name)
        }
    }

    impl Debug for TupleStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut builder = f.debug_tuple(self.name);

            for value in &self.values {
                builder.field(value);
            }

            builder.finish()
        }
    }

    impl Debug for NamedStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut builder = f.debug_struct(self.name);

            for (name, value) in self.fields.iter() {
                builder.field(name, value);
            }

            builder.finish()
        }
    }


    impl Debug for UnitVariant {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}::{}", self.name, self.variant)
        }
    }

    impl Debug for TupleVariant {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut builder = f.debug_tuple(&format!("{}::{}", self.name, self.variant));

            for value in &self.values {
                builder.field(value);
            }

            builder.finish()
        }
    }

    impl Debug for NamedVariant {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut builder = f.debug_struct(&format!("{}::{}", self.name, self.variant));

            for (name, value) in self.fields.iter() {
                builder.field(name, value);
            }

            builder.finish()
        }
    }
}