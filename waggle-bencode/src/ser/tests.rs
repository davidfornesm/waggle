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
