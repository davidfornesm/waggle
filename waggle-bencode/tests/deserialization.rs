use waggle_bencode::from_bytes;

#[test]
fn deserialize_bool() {
    let source = b"i0e";
    let result = from_bytes::<bool>(source);
    assert_eq!(result.unwrap(), false);
}
