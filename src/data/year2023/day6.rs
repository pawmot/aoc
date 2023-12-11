use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::all_consuming,
    multi::{many0, separated_list1, many1},
    IResult,
};

use crate::data::common::{read_lines, DatasetType, parse_u16};

pub struct Race {
    pub time: u16,
    pub record: u16,
}

fn parse_times(input: &str) -> IResult<&str, Vec<u16>> {
    all_consuming(|inp| -> IResult<&str, Vec<u16>> {
        let (inp, _) = tag("Time:")(inp)?;
        let (inp, _) = many0(char(' '))(inp)?;
        let (inp, times) = separated_list1(many1(char(' ')), parse_u16)(inp)?;
        Ok((inp, times))
    })(input)
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u16>> {
    all_consuming(|inp| -> IResult<&str, Vec<u16>> {
        let (inp, _) = tag("Distance:")(inp)?;
        let (inp, _) = many0(char(' '))(inp)?;
        let (inp, times) = separated_list1(many1(char(' ')), parse_u16)(inp)?;
        Ok((inp, times))
    })(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<Race>> {
    let lines = read_lines((2023, 6), dataset_type)?;
    // TODO: using `?` instead of `unwrap()` here causes the borrow-checker to lose its shit.
    // Figure it out
    let times = parse_times(&lines[0]).unwrap().1;
    let distances = parse_distances(&lines[1]).unwrap().1;
    let result = times.into_iter()
        .zip(distances.into_iter())
        .map(|(time, record)| Race { time, record })
        .collect();
    Ok(result)
}

