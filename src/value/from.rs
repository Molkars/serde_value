use std::borrow::Cow;
use crate::value::{Number, TupleStruct, Value};

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Unit
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

impl<T: Into<Number>> From<T> for Value {
    fn from(value: T) -> Self {
        Value::Number(value.into())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Cow<'_, str>> for Value {
    fn from(value: Cow<str>) -> Self {
        Self::String(value.to_string())
    }
}

impl<const N: usize> From<&'static [u8; N]> for Value {
    fn from(value: &'static [u8; N]) -> Self {
        Self::Seq(value.iter()
            .copied()
            .map(Number::from)
            .map(Into::into)
            .collect())
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        value
            .map(|v| {
                Value::TupleStruct(TupleStruct {
                    name: "Some",
                    values: vec![v.into()],
                })
            })
            .unwrap_or_else(|| {
                Value::TupleStruct(TupleStruct {
                    name: "None",
                    values: vec![]
                })
            })
    }
}

impl<T: Into<Value>> FromIterator<T> for Value {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        Self::Seq(iter.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Value {
    fn from(value: &'a [T]) -> Self {
        value.iter().map(Clone::clone).map(Into::into).collect()
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(value: Vec<T>) -> Self {
        value.into_iter().collect()
    }
}
