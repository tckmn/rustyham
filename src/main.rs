use std::io;
use std::io::prelude::*;
use std::iter::repeat;
use std::str::FromStr;

fn main() {
    let title = "RUSTYHAM: A HAMMING CODE GENERATOR IN RUST";
    let border: String = repeat('=').take(title.len()).collect::<>();
    println!("{}", border);
    println!("{}", title);
    println!("{}", border);

    loop {
        let mut linebuf: String = String::new();
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
                    2...4 => "Sorry, not implemented yet.",
                    _ => { good = false; "Invalid input." }
                });
                if good { break; }
            }
            None => {
                println!("Invalid input.");
            }
        };
    }
}

fn hamming() -> &'static str {
    "STOP - HAMMINGTIME!"
}