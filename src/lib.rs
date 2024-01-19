use serde::Serialize;
use crate::ser::{Error, Serializer};
use crate::value::Value;

pub mod ser;
pub mod value;

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