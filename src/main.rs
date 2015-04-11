use std::io;
use std::io::prelude::*;
use std::iter::repeat;
use std::str::FromStr;

enum Hamming { Encode, Decode, EncodeBinary, DecodeBinary }

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

fn hamming(action: Hamming, s: String) -> String {
    match action {
        Hamming::Encode | Hamming::EncodeBinary => {
            // get an iterator over the individual bits
            let mut bytes;
            let bytes_str = match action {
                Hamming::EncodeBinary => s,
                _ => {
                    // convert ASCII string to binary
                    bytes = s.into_bytes();  // takes ownership of s
                    bytes.iter().map(|&c| format!("{:0>1$b}", c, 7))
                        .collect::<Vec<String>>().concat()
                }
            };
            let mut bytes_iter = bytes_str.chars();
            // we should assume that ownership of s has already been
            // transferred away by this point

            // compute block and message length
            let mlen = bytes_str.len() as u32;
            let lenpow = (2..).find(|&r| 2u32.pow(r) - r - 1 >= mlen).unwrap();
            let len = 2us.pow(lenpow) - 1;

            // the thing we're storing the hamming code in
            let mut code: Vec<bool> = repeat(false).take(len).collect::<>();

            // set data bits
            for i in 1..len {
                if (i & (i - 1)) != 0 {  // if i is not a power of 2
                    code[i-1] = bytes_iter.next().unwrap_or('0') == '1';
                }
            }

            // set parity bits
            for i in 0..lenpow {
                code[2us.pow(i) - 1] = calc_parity(&code, i);
            }

            code.into_iter().map(|x| if x {"1"} else {"0"}).collect::<Vec<_>>().concat()
        },
        Hamming::Decode | Hamming::DecodeBinary => {
            // verify parity bits, fix 1-bit-flipped errors if any
            let len = s.len();
            let lenpow = ((len + 1) as f32).sqrt().round() as u32;
            let mut chars = s.chars().map(|x| x == '1').collect::<Vec<bool>>();
            let mut flipped_bit = -1i32;
            while (0..lenpow).any(|i| calc_parity(&chars, i)) {
                if flipped_bit != -1 {
                    chars[flipped_bit as usize] = !chars[flipped_bit as usize];
                }
                flipped_bit += 1;
                chars[flipped_bit as usize] = !chars[flipped_bit as usize];
            }

            // collect all bits at non-powers-of-2
            let data = chars.iter().enumerate()
                .filter(|x| ((x.0 + 1) & x.0) != 0).map(|x| if *x.1 {'1'} else {'0'});

            // return
            match action {
                Hamming::DecodeBinary => {
                    data.collect::<String>()
                },
                _ => {
                    let chars = (&data.collect::<Vec<char>>()[..]).chunks(7)
                        .map(|x| u8::from_str_radix(&x.iter().cloned().collect::<String>()[..], 2).unwrap())
                        .collect::<Vec<u8>>();
                    String::from_utf8(chars).unwrap()
                }
            }
        }
    }
}

fn calc_parity(code: &Vec<bool>, i: u32) -> bool {
    let bi = 2us.pow(i) - 1;
    let (mut parity, mut ignore, mut counter) = (false, false, 0);
    for j in bi..code.len() {
        if !ignore && code[j] { parity = !parity }
        counter += 1;
        if counter >= 2u32.pow(i) {
            ignore = !ignore;
            counter = 0;
        }
    }
    parity
}

fn prompt(s: &str) -> String {
    let mut input = String::new();
    print!("{}", s);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
