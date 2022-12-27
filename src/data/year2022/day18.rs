use anyhow::Result;

use nom::{bytes::complete::tag, combinator::all_consuming, multi::many0, IResult};

use crate::data::common::{parse_new_line, parse_usize, read_lines, DatasetType};

fn parse_point(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, n1) = parse_usize(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, n2) = parse_usize(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, n3) = parse_usize(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, (n1, n2, n3)))
}

fn parse_data(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    all_consuming(many0(parse_point))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<(usize, usize, usize)>> {
    let lines = read_lines((2022, 18), dataset_type)?;
    let data = lines.join("\n");
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
