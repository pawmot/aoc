use anyhow::Result;

use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, opt},
    multi::{many0, many1},
    sequence::tuple,
    IResult,
};

use crate::data::common::{parse_new_line, parse_usize, read_lines, DatasetType};

#[cfg(test)]
mod data_day14 {
    use crate::data::year2022::day14::parse_point;

    #[test]
    fn test_parse_point() {
        assert_eq!(parse_point("13,2"), Ok(("", (13, 2))));
    }
}

fn parse_point(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, n1) = parse_usize(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, n2) = parse_usize(input)?;
    let (input, _) = opt(tag(" -> "))(input)?;
    Ok((input, (n1, n2)))
}

fn parse_path(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    let (input, v) = many1(parse_point)(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, v))
}

fn parse_data(input: &str) -> IResult<&str, Vec<Vec<(usize, usize)>>> {
    all_consuming(many0(parse_path))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<Vec<(usize, usize)>>> {
    let lines = read_lines((2022, 14), dataset_type)?;
    let data = lines.join("\n");
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
