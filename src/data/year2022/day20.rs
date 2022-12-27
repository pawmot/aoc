use anyhow::Result;

use nom::{combinator::all_consuming, multi::many0, IResult};

use crate::data::common::{parse_i64, parse_new_line, read_lines, DatasetType};

fn parse_line(input: &str) -> IResult<&str, i64> {
    let (input, n) = parse_i64(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, n))
}

fn parse_data(input: &str) -> IResult<&str, Vec<i64>> {
    all_consuming(many0(parse_line))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<i64>> {
    let lines = read_lines((2022, 20), dataset_type)?;
    let data = lines.join("\n");
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
