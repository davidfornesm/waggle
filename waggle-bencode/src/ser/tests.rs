use crate::to_bytes;

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
