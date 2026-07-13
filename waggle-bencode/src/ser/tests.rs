use crate::to_bytes;
use serde::Serialize;

#[test]
fn serialize_positive_number() {
    let number = 123;
    let expected = b"i123e";
    let serialized = to_bytes(&number).unwrap();
    assert_eq!(serialized, expected)
}

#[test]
fn serialize_negative_number() {
    let number = -123;
    let expected = b"i-123e";
    let serialized = to_bytes(&number).unwrap();
    assert_eq!(serialized, expected)
}

#[test]
fn serialize_zero() {
    let number = 0;
    let expected = b"i0e";
    let serialized = to_bytes(&number).unwrap();
    assert_eq!(serialized, expected)
}

#[test]
fn serialize_bytes() {
    let bytes = "hello";
    let expected = b"5:hello";
    let serialized = to_bytes(&bytes).unwrap();
    assert_eq!(serialized, expected)
}

#[test]
fn serialize_empty_bytes() {
    let bytes = "";
    let expected = b"0:";
    let serialized = to_bytes(&bytes).unwrap();
    assert_eq!(serialized, expected)
}

#[test]
fn serialize_list_of_integers() {
    let list = vec![1, 2, 3];
    let expected = b"li1ei2ei3ee";
    let serialized = to_bytes(&list).unwrap();
    assert_eq!(serialized, expected)
}

#[test]
fn serialize_list_of_bytes() {
    let list = vec!["hi", "hey", "hello"];
    let expected = b"l2:hi3:hey5:helloe";
    let serialized = to_bytes(&list).unwrap();
    assert_eq!(serialized, expected)
}

#[test]
fn serialize_empty_list() {
    let list: Vec<i32> = Vec::new();
    let expected = b"le";
    let serialized = to_bytes(&list).unwrap();
    assert_eq!(serialized, expected)
}

#[test]
fn serialize_tuple_of_integer_and_bytes() {
    let tuple = (42, "hello");
    let expected = b"li42e5:helloe";
    let serialized = to_bytes(&tuple).unwrap();
    assert_eq!(serialized, expected)
}

#[derive(Serialize)]
struct TupleStruct(i32, i32);

#[test]
fn serialize_tuple_struct() {
    let value = TupleStruct(3, 12);
    let expected = b"li3ei12ee";
    let serialized = to_bytes(&value).unwrap();
    assert_eq!(serialized, expected)
}

#[derive(Serialize)]
struct EmptyTupleStruct();
#[test]
fn serialize_empty_tuple_struct() {
    let value = EmptyTupleStruct();
    let expected = b"le";
    let serialized = to_bytes(&value).unwrap();
    assert_eq!(serialized, expected)
}
