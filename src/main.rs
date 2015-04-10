use std::io;
use std::io::prelude::*;
use std::iter::repeat;
use std::str::FromStr;
use std::num::Int;
use std::collections::BitVec;

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
    // prompt for ASCII input
    let mut message = String::new();
    print!("Enter string to encode: ");
    io::stdout().flush();
    io::stdin().read_line(&mut message);

    // compute block and message length
    message = message.trim().to_string();
    let mlen = message.len() as u32 * 7;
    let lenpow = (2..).find(|&r| 2.pow(r) - r - 1 >= mlen).unwrap();
    let len = 2.pow(lenpow) - 1;

    // the thing we're storing the hamming code in
    let mut bv = BitVec::from_elem(len, false);

    // convert ASCII string to binary
    // IMPORTANT NOTE: the following line takes ownership of the `message'
    // variable. We no longer have access to the original string from this
    // point onwards.
    let bytes = message.into_bytes();
    let bytes_str = bytes.iter()
        .map(|&c| format!("{:0>1$b}", c, 7))
        .collect::<Vec<String>>()
        .concat();
    let mut bytes_iter = bytes_str.chars();

    // set data bits
    for i in 1..len {
        if (i & (i - 1)) != 0 {
            bv.set(i-1, bytes_iter.next().unwrap_or('0') == '1');
        }
    }

    // set parity bits
    for i in 0..lenpow {
        let bi = 2.pow(i)-1;
        let mut parity = false;
        let mut ignore = false;
        let mut counter = 0;
        for j in bi..len {
            if !ignore {
                if bv.get(j).unwrap() {
                    parity = !parity;
                }
            }
            counter += 1;
            if counter >= 2.pow(i) {
                ignore = !ignore;
                counter = 0;
            }
        }
        bv.set(bi, parity);
    }

    format!("{:?}", bv)
}
