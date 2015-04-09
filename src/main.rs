use std::io;
use std::io::prelude::*;
use std::iter::repeat;
use std::str::FromStr;
use std::num::Int;
use std::num::ToPrimitive;

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
        io::stdout().flush();
        io::stdin().read_line(&mut linebuf);

        let num = i32::from_str(linebuf.trim());
        match num.ok() {
            Some(n) => {
                let mut good = true;
                println!("{}", match n {
                    1 => hamming(),
                    2...4 => "Sorry, not implemented yet.".to_string(),
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

fn hamming() -> String {
    let mut message = String::new();
    print!("Enter string to encode: ");
    io::stdout().flush();
    io::stdin().read_line(&mut message);

    message = message.trim().to_string();
    let mlen = message.len().to_u32().unwrap();
    let len = 2.pow((2..).find(|&r| 2.pow(r) - r - 1 >= mlen).unwrap()) - 1;

    message
}
