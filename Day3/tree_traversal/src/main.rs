use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "data.txt";

fn main() {
    let f = File::open(FILENAME)
        .expect("Something went wrong reading the file");
    let reader = BufReader::new(f);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let xy_slopes: Vec<(u32, u32)> = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    let num_columns = lines[0].chars().count();
    let num_lines = lines.len();

    for slope in xy_slopes {
        let mut num_found: i32 = 0;
        let (x_slope, y_slope) = slope;

        let mut x_coord: u32 = 0;
        let mut y_coord: u32 = 0;

        while y_coord < num_lines as u32 {
            let x_val = x_coord % num_columns as u32;
            let char_at_pos = lines[y_coord as usize].chars().nth(x_val as usize).unwrap();

            if char_at_pos.to_string() == "#" {
                num_found += 1
            }
            x_coord += x_slope;
            y_coord += y_slope;

        }
        println!("X: {}, Y: {} has {} matches", x_slope, y_slope, num_found);
    }
}
