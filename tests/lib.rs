extern crate rustyham;
use rustyham::rustyham::*;

#[test]
fn testfoobar() {
    assert!(hamming(Hamming::Encode, "a".to_string()) == "101110010010000".to_string());
}
