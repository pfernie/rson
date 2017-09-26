extern crate rson_rs as rson;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct UnitStruct;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct NewType(f32);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct TupleStruct(UnitStruct, i8);

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
struct Key(u32);

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum Enum {
    Unit,
    Bool(bool),
    Chars(char, String),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Struct {
    tuple: ((), NewType, TupleStruct),
    vec: Vec<Option<UnitStruct>>,
    map: HashMap<Key, Enum>,
}

#[test]
fn roundtrip() {
    let value = Struct {
        tuple: ((), NewType(0.5), TupleStruct(UnitStruct, -5)),
        vec: vec![None, Some(UnitStruct)],
        map: vec![
            (Key(5), Enum::Unit),
            (Key(6), Enum::Bool(false)),
            (Key(7), Enum::Bool(true)),
            (Key(9), Enum::Chars('x', "".to_string())),
        ].into_iter().collect()
    };

    let serial = rson::ser::to_string(&value).unwrap();

    println!("Serialized: {}", serial);

    let deserial = rson::de::from_str(&serial);

    assert_eq!(Ok(value), deserial);
}
