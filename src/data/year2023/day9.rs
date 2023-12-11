use anyhow::Result;
use nom::{
    combinator::all_consuming,
    character::complete::char,
    multi::separated_list1,
    IResult,
};

use crate::data::common::{read_lines, DatasetType, parse_new_line, parse_i32};

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, line) = separated_list1(char(' '), parse_i32)(input)?;
    Ok((input, line))
}

fn parse_data(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    all_consuming(|inp| -> IResult<&str, Vec<Vec<i32>>> {
        let (inp, lines) = separated_list1(char('\n'), parse_line)(inp)?;
        let (inp, _) = parse_new_line(inp)?;
        Ok((inp, lines))
    })(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<Vec<i32>>> {
    let lines = read_lines((2023, 9), dataset_type)?;
    let data = lines.join("\n");
    // TODO: using `?` instead of `unwrap()` here causes the borrow-checker to lose its shit.
    // Figure it out
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
