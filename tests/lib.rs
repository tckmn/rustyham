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

#[test]
fn test_error_correction() {
    let s = hamming(Hamming::Encode, "test".to_string()).into_bytes();
    for i in 0..s.len() {
        let mut s2 = s.clone();
        s2[i] = if s2[i] == 48 { 49 } else { 48 };
        assert_eq!(hamming(Hamming::Decode, String::from_utf8(s2).unwrap()), "test".to_string());
    }
}
