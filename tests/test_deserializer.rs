use std::fs;

use hessian_rs::Error;
use hessian_rs::{de::Deserializer, Value};

fn load_value_from_file(file_name: &str) -> Result<Value, Error> {
    let rdr = fs::read(file_name)?;
    let mut de = Deserializer::new(rdr);
    de.read_value()
}

#[test]
fn test_decode_long_binary() {
    let value = load_value_from_file("tests/fixtures/bytes/65535.bin").unwrap();
    match value {
        Value::Bytes(bytes) => assert_eq!(bytes, vec![0x41; 65535]),
        _ => panic!("expect bytes"),
    }
}

#[test]
fn test_decode_date() {
    assert_eq!(
        load_value_from_file("tests/fixtures/date/894621060000.bin").unwrap(),
        Value::Date(894621060000)
    );
    assert_eq!(
        load_value_from_file("tests/fixtures/date/894621091000.bin").unwrap(),
        Value::Date(894621091000)
    );
    assert_eq!(
        load_value_from_file("tests/fixtures/date/128849018880000.bin").unwrap(),
        Value::Date(128849018880000)
    );
    assert_eq!(
        load_value_from_file("tests/fixtures/date/-128849018940000.bin").unwrap(),
        Value::Date(-128849018940000)
    );
}

#[test]
fn test_decode_string() {
    assert_eq!(
        load_value_from_file("tests/fixtures/string/empty.bin").unwrap(),
        Value::String("".to_string())
    );
    assert_eq!(
        load_value_from_file("tests/fixtures/string/foo.bin").unwrap(),
        Value::String("foo".to_string())
    );
    assert_eq!(
        load_value_from_file("tests/fixtures/string/chinese.bin").unwrap(),
        Value::String("中文 Chinese".to_string())
    );
}