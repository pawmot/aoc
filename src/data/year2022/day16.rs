use anyhow::Result;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while_m_n},
    combinator::{all_consuming, opt},
    multi::{many0, many1},
    IResult,
};

use crate::data::common::{parse_new_line, parse_u32, read_lines, DatasetType};

#[cfg(test)]
mod data_day16 {
    use crate::data::year2022::day16::parse_data;

    #[test]
    fn parse_data_test() {
        assert_eq!(
            parse_data("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve HH has flow rate=22; tunnel leads to valve GG\n"),
            Ok(("", vec![(String::from("AA"), 0, vec![String::from("DD"), String::from("II"), String::from("BB")]), (String::from("HH"), 22, vec![String::from("GG")])]))
        );
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn parse_path(input: &str) -> IResult<&str, String> {
    let (input, path) = take_while_m_n(2, 2, is_letter)(input)?;
    let (input, _) = opt(tag(", "))(input)?;
    Ok((input, String::from(path)))
}

fn parse_line(input: &str) -> IResult<&str, (String, u32, Vec<String>)> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = take_while_m_n(2, 2, is_letter)(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow) = parse_u32(input)?;
    let (input, _) = alt((
        tag("; tunnel leads to valve "),
        tag("; tunnels lead to valves "),
    ))(input)?;
    let (input, paths) = many1(parse_path)(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, (String::from(name), flow, paths)))
}

fn parse_data(input: &str) -> IResult<&str, Vec<(String, u32, Vec<String>)>> {
    all_consuming(many0(parse_line))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<(String, u32, Vec<String>)>> {
    let lines = read_lines((2022, 16), dataset_type)?;
    let data = lines.join("\n");
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
