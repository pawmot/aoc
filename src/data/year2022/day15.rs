use anyhow::Result;

use nom::{bytes::complete::tag, combinator::all_consuming, multi::many0, IResult};

use crate::data::common::{parse_i32, parse_new_line, parse_usize, read_lines, DatasetType};

#[cfg(test)]
mod data_day15 {
    use crate::data::year2022::day15::parse_data;

    #[test]
    fn parse_data_test() {
        assert_eq!(
            parse_data("Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\n"),
            Ok(("", vec![((2, 18), (-2, 15)), ((9, 16), (10, 16))]))
        );
    }
}
// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn parse_point(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("x=")(input)?;
    let (input, n1) = parse_i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, n2) = parse_i32(input)?;
    Ok((input, (n1, n2)))
}

fn parse_line(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = parse_point(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, beacon) = parse_point(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, (sensor, beacon)))
}

fn parse_data(input: &str) -> IResult<&str, Vec<((i32, i32), (i32, i32))>> {
    all_consuming(many0(parse_line))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<((i32, i32), (i32, i32))>> {
    let lines = read_lines((2022, 15), dataset_type)?;
    let data = lines.join("\n");
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
