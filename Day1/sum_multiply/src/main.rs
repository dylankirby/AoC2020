use std::fs;

extern crate argparse;

use argparse::{ArgumentParser, Store};
use itertools::Itertools;

const FILENAME: &str = "data.txt";
const NUM_VALUES: i64 = 2;

enum TwoOrThreeTuple<'a> {
    TwoTuple(&'a (i64, i64)),
    ThreeTuple(&'a (i64, i64, i64)),
}

fn main() -> anyhow::Result<()> {
    let contents = fs::read_to_string(FILENAME)?;

    let numbers: Vec<i64> = contents
        .split_whitespace() // splits string contents on whitespce, returns split
        .map(str::parse::<i64>) // maps str parse for i32 to all values
        .collect::<Result<Vec<_>, _>>()?; // collects split into in memory vector, return Err if occurs thanks to ?

    let found_values = find_x_values_that_sum_to_target(NUM_VALUES, numbers, 2020);
    dbg!(found_values);

    // let multiplied_value = num1 * num2;
    // println!("{}", multiplied_value);
    Ok(())
}

fn find_x_values_that_sum_to_target(num_values: i64, values: Vec<i64>, target: i64) -> Option<TwoOrThreeTuple<'static>> {
    if num_values < 2 || num_values > 3 {
        panic!("Only 2/3 values supported for processing")
    }
    let as_tuples = values.into_iter().tuple_combinations();

    if num_values == 2 {
        TwoOrThreeTuple::TwoTuple(as_tuples.find(|(a, b)| a + b == target))
    } else {
        TwoOrThreeTuple::ThreeTuple(as_tuples.find(|(a, b, c)| a + b + c == 2020))
    }
}