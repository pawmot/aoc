use anyhow::Result;
use nom::{
    combinator::all_consuming,
    multi::many0,
    IResult, bytes::complete::{tag, take_until},
};

use crate::data::common::{read_lines, DatasetType, parse_new_line, parse_word};

fn parse_mapping(input: &str) -> IResult<&str, (String, String, String)> {
    let (input, from) = take_until(" ")(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = take_until(",")(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = take_until(")")(input)?;
    let (input, _) = tag(")\n")(input)?;
    Ok((input, (from.to_owned(), left.to_owned(), right.to_owned())))
}

fn parse_data(input: &str) -> IResult<&str, (String, Vec<(String, String, String)>)> {
    all_consuming(|inp| -> IResult<&str, (String, Vec<(String, String, String)>)> {
        let (inp, instructions) = parse_word(inp)?;
        let (inp, _) = parse_new_line(inp)?;
        let (inp, _) = parse_new_line(inp)?;
        let (inp, mappings) = many0(parse_mapping)(inp)?;
        Ok((inp, (instructions, mappings)))
    })(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<(String, Vec<(String, String, String)>)> {
    let lines = read_lines((2023, 8), dataset_type)?;
    let data = lines.join("\n");
    // TODO: using `?` instead of `unwrap()` here causes the borrow-checker to lose its shit.
    // Figure it out
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
