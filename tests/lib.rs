extern crate rustyham;
use rustyham::rustyham::*;

#[test]
fn test_encode() {
    assert_eq!(hamming(Hamming::Encode, "test".to_string()), "111011011001100110111100111110100000000000000000000000000000000".to_string());
}

#[test]
fn test_decode() {
    assert_eq!(hamming(Hamming::Decode, "111011011001100110111100111110100000000000000000000000000000000".to_string()), "test".to_string());
}
