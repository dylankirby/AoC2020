use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "data.txt";

fn main() {
    let f = File::open(FILENAME)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(f);

    let mut num_valid: u32 = 0;
    for line in reader.lines() {
        let line_data = match line {
            Ok(res) => res,
            Err(_) => continue
        };

        let parsed: Vec<String> = line_data
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();

        let range: Vec<&str> = parsed[0].split("-")
            .collect();

        let mut lower: u8 = range[0]
            .parse()
            .expect("Error parsing lower bound to integer");
        lower -= 1;

        let mut upper: u8 = range[1]
            .parse()
            .expect("Error parsing upper bound to integer");
        upper -= 1;

        let character: char = parsed[1].chars().nth(0).unwrap();
        let password: &String = &parsed[2];
        let char_at_lower = password.chars().nth(lower as usize).unwrap();
        let char_at_upper = password.chars().nth(upper as usize).unwrap();

        if ((char_at_upper == character) | (char_at_lower == character)) && !(char_at_upper == character && char_at_lower == character) {
            println!("Valid");
            num_valid += 1;
        }
        
    }

    println!("Found {} valid passwords", num_valid);
}

// fn main1() {
//     let f = File::open(FILENAME)
//         .expect("Something went wrong reading the file");
//     let reader = BufReader::new(f);

//     let mut num_valid: u32 = 0;
//     for line in reader.lines() {
//         let line_data = match line {
//             Ok(res) => res,
//             Err(_) => continue
//         };

//         let parsed: Vec<String> = line_data
//             .split_whitespace()
//             .map(|s| s.parse().expect("parse error"))
//             .collect();

//         let range: Vec<&str> = parsed[0].split("-")
//             .collect();

//         let character: char = parsed[1].chars().nth(0).unwrap();

//         let lower: u8 = range[0]
//             .parse()
//             .expect("Error parsing lower bound to integer");

//         let upper: u8 = range[1]
//             .parse()
//             .expect("Error parsing upper bound to integer");

//         let password: &String = &parsed[2];
//         let my_chars: Vec<_> = password.chars().collect();
//         let mut num_my_char: u8 = 0;
//         for c in my_chars {
//             if c == character {
//                 num_my_char += 1;
//             }
//         };
//         println!("{}", num_my_char);

//         if (lower <= num_my_char && upper >= num_my_char) {
//             num_valid += 1;
//         }
        
//     }

//     println!("Found {} valid passwords", num_valid);
// }