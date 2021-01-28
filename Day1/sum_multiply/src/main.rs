use std::fs;

extern crate argparse;

use itertools::Itertools;

const FILENAME: &str = "data.txt";
const NUM_VALUES: i64 = 2;

enum ResultTuple {
    TwoTuple(Option<(i64, i64)>),
    ThreeTuple(Option<(i64, i64, i64)>),
}

fn main() -> anyhow::Result<()> {
    let contents = fs::read_to_string(FILENAME)?;

    let numbers: Vec<i64> = contents
        .split_whitespace() // splits string contents on whitespce, returns split
        .map(str::parse::<i64>) // maps str parse for i32 to all values
        .collect::<Result<Vec<_>, _>>()?; // collects split into in memory vector, return Err if occurs thanks to ?

    match find_x_values_that_sum_to_target(NUM_VALUES, numbers, 2020) { //TODO: Remove duplication here
        ResultTuple::TwoTuple(value) => {
            let numbers = value.unwrap();
            let answer = numbers.0 * numbers.1;
            println!("Numbers are {} and {}, and the answer is {}", numbers.0, numbers.1, answer);
        }
        ResultTuple::ThreeTuple(value) => {
            let numbers = value.unwrap();
            let answer =  numbers.0 * numbers.1 * numbers.2;
            println!("Numbers are {}, {}, and {}, and the answer is {}", numbers.0, numbers.1, numbers.2, answer);
        }
    }

    // let multiplied_value = num1 * num2;
    // println!("{}", multiplied_value);
    Ok(())
}

fn find_x_values_that_sum_to_target(num_values: i64, values: Vec<i64>, target: i64) -> ResultTuple{
    if num_values < 2 || num_values > 3 {
        panic!("Only 2/3 values supported for processing")
    }

    if num_values == 2 {
        return ResultTuple::TwoTuple(values.into_iter().tuple_combinations().find(|(a, b)| a + b == target));
    } else {
        return ResultTuple::ThreeTuple(values.into_iter().tuple_combinations().find(|(a, b, c)| a + b + c == 2020));
    }

}