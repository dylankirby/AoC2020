use std::fs;
use std::io::ErrorKind;

const FILENAME: &str = "data.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let (num1, num2, num3) = find_2020_sum(numbers);

    let multiplied_value = num1 * num2 * num3;
    println!("{}", multiplied_value);
}

fn find_2020_sum(values: Vec<i32>) -> (i32, i32, i32){
    for value in &values {
        let pair_value: i32 = 2020 - value;
        for inner_value in &values {
            let second_pair_value: i32 = pair_value - inner_value;
            if values.contains(&second_pair_value) {
                return (*value, *inner_value, second_pair_value)
            }
        }
    }
    (0, 0, 0)
}
