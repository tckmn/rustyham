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

#[test]
fn test_encode_binary() {
    assert_eq!(hamming(Hamming::EncodeBinary, "101010".to_string()), "111101001010000");
}

#[test]
fn test_decode_binary() {
    assert_eq!(hamming(Hamming::DecodeBinary, "111101001010000".to_string()), "101010");
}
