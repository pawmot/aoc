use anyhow::Result;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    combinator::all_consuming,
    multi::many1,
    IResult,
};

use crate::data::common::{parse_new_line, parse_u8, read_lines, DatasetType};

fn is_map_char(ch: char) -> bool {
    ch == ' ' || ch == '.' || ch == '#'
}

fn parse_map_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, line) = take_while1(is_map_char)(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, line.chars().collect()))
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, lines) = many1(parse_map_line)(input)?;
    let (input, _) = parse_new_line(input)?;

    let max_line_len = lines.iter().map(|l| l.len()).max().unwrap();
    let lines = lines
        .into_iter()
        .map(|l| {
            let mut result = l;
            while result.len() < max_line_len {
                result.push(' ');
            }
            result
        })
        .collect();

    Ok((input, lines))
}

#[derive(Debug, PartialEq)]
pub enum Move {
    Walk(u8),
    TurnLeft,
    TurnRight,
}

fn parse_walk(input: &str) -> IResult<&str, Move> {
    let (input, n) = parse_u8(input)?;
    Ok((input, Move::Walk(n)))
}

fn parse_turn_left(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("L")(input)?;
    Ok((input, Move::TurnLeft))
}

fn parse_turn_right(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("R")(input)?;
    Ok((input, Move::TurnRight))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, moves) = many1(alt((parse_walk, parse_turn_left, parse_turn_right)))(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, moves))
}

fn parse_data(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Move>)> {
    let (input, map) = parse_map(input)?;
    let (input, moves) = all_consuming(parse_moves)(input)?;
    Ok((input, (map, moves)))
}

pub fn get_data(dataset_type: DatasetType) -> Result<(Vec<Vec<char>>, Vec<Move>)> {
    let lines = read_lines((2022, 22), dataset_type)?;
    let data = lines.join("\n");
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
