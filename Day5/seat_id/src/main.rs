use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "data.txt";
const ROW_UPPER: i32 = 127;
const ROW_TOP_HALF_CHAR: char = 'B';
const ROW_BOTTOM_HALF_CHAR: char = 'F';

const COLUMN_UPPER: i32 = 7;
const COLUMN_TOP_HALF_CHAR: char = 'R';
const COLUMN_BOTTOM_HALF_CHAR: char = 'L';

const FIND_MINE: bool = true;

fn main() {
    let f = File::open(FILENAME)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(f);

   	let mut highest: i32 = 0;
   	let mut all_found: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line_data = match line {
            Ok(res) => res,
            Err(_) => panic!("Couldn't process line")
        };
        
        let seat_id = process_line(line_data);
        if seat_id > highest {
        	highest = seat_id;
        }

        if FIND_MINE {
        	all_found.push(seat_id);
        }
    }
    println!("Highest found seat ID was {}", highest);

    if FIND_MINE {
    	let mine: i32 = search_for_my_seat(&mut all_found);
    	println!("My seat is: {}", mine);
    }
}


fn process_line(line_data: String) -> i32{
	let row_data = &line_data[..7];
	let column_data = &line_data[7..];

	println!("Got row info: {} and seat info {}", row_data, column_data);

	let row_id = divide_range_from_chars(row_data, ROW_TOP_HALF_CHAR, ROW_BOTTOM_HALF_CHAR, ROW_UPPER);
	let column_id = divide_range_from_chars(column_data, COLUMN_TOP_HALF_CHAR, COLUMN_BOTTOM_HALF_CHAR, COLUMN_UPPER);

	let seat_id = row_id * 8 + column_id;
	seat_id
}

fn divide_range_from_chars(chars_as_str: &str, top_half_char: char, bottom_half_char: char, range_max: i32) -> i32 {
	let chars: Vec<char> = chars_as_str.to_string().chars().collect();
	let mut lower: i32 = 0;
	let mut upper: i32 = range_max.clone();

	for (i, c) in chars.iter().enumerate() {
		let index = i as u32 + 1;
		let change_val: i32 = (range_max + 1)/(2_i32.pow(index));
		println!("Changing by {} for i: {}", change_val, i);
		if c == &bottom_half_char {
			upper -= change_val;
		} else if c == &top_half_char {
			lower += change_val;
		} else {
			panic!("Got unknown seat position")
		}
	}
	if lower == upper {
		return lower
	} else {
		panic!("Could not agree on upper lower number, L {}, U {}", lower, upper)
	}
}

fn search_for_my_seat(all_found_seat_ids: &mut Vec<i32>) -> i32 {
	all_found_seat_ids.sort();
	let num_ids = all_found_seat_ids.len();
	for (i, sid) in all_found_seat_ids.iter().enumerate() {
		if i == 0 || i >= num_ids {
			continue;
		}
		let after = all_found_seat_ids[i+1];
		let expected_after = sid + 1;
		if after != expected_after {
			return expected_after
		}

	}
}