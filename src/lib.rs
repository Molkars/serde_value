//! # Serde Value
//! serde_value is a simple intermediate-value library built on serde serialization
//!
//! # Example
//! ```
//! use serde_derive::{Serialize};
//!
//! #[derive(Serialize, Debug)]
//! pub struct MyData {
//!     a: MyEnum,
//!     b: MyEnum,
//!     c: MyEnum,
//! }
//!
//! #[derive(Serialize, Debug)]
//! pub enum MyEnum {
//!     A(u32),
//!     B { val: String },
//!     C,
//! }
//!
//! let value = MyData {
//!     a: MyEnum::A(1),
//!     b: MyEnum::B { val: "hi".to_string() },
//!     c: MyEnum::C
//! };
//!
//! let value = serde_value::to_value(&value).unwrap();
//! println!("{:?}", value);
//! ```

use serde::Serialize;
use crate::ser::{Error, Serializer};

pub mod ser;
mod value;

pub use value::*;

/// Convert `value` to an intermediate `Value`
pub fn to_value<T: Serialize>(value: &T) -> Result<Value, Error> {
    value.serialize(&mut Serializer)
}

#[test]
fn test() {
    use serde_derive::Serialize;

    let value = 1u32;
    let value = to_value(&value).unwrap();
    assert_eq!(Some(1), value.as_u32());

    #[derive(Serialize)]
    struct A {
        value: u32,
    }

    #[derive(Serialize)]
    enum B {
        A { val: i32 },
    }

    #[derive(Serialize)]
    struct C(u32);

    let value = to_value(&A { value: 0 }).unwrap();
    println!("{value:?}");

    let value = to_value(&C(3)).unwrap();
    println!("{value:?}");

    let value = to_value(&(1, 2, "hi")).unwrap();
    println!("{:?}", value);

    let value = to_value(&B::A { val: 1 }).unwrap();
    println!("{:?}", value);
}