use std::io;
use std::io::prelude::*;
use std::iter::repeat;
use std::str::FromStr;

extern crate rustyham;
use rustyham::rustyham::*;

fn main() {
    let title = "RUSTYHAM: A HAMMING CODE GENERATOR IN RUST";
    let border: String = repeat('=').take(title.len()).collect::<>();
    println!("{}", border);
    println!("{}", title);
    println!("{}", border);

    loop {
        let mut linebuf = String::new();
        println!("(1) Encode ASCII");
        println!("(2) Encode binary");
        println!("(3) Decode to ASCII");
        println!("(4) Decode to binary");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut linebuf).unwrap();

        let num = i32::from_str(linebuf.trim());
        match num.ok() {
            Some(n) => {
                let mut good = true;
                println!("{}", match n {
                    1 => hamming(Hamming::Encode, prompt("Enter string to encode: ")),
                    2 => hamming(Hamming::EncodeBinary, prompt("Enter binary string to encode: ")),
                    3 => hamming(Hamming::Decode, prompt("Enter Hamming code to decode: ")),
                    4 => hamming(Hamming::DecodeBinary, prompt("Enter Hamming code to decode to binary: ")),
                    _ => { good = false; "Invalid input.".to_string() }
                });
                if good { break; }
            }
            None => {
                println!("Invalid input.");
            }
        };
    }
}

fn prompt(s: &str) -> String {
    let mut input = String::new();
    print!("{}", s);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
