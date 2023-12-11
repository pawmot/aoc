use anyhow::Result;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, opt},
    multi::many0,
    IResult,
};

use crate::data::common::{parse_u8, parse_word, read_lines, DatasetType, parse_new_line};

pub struct Game {
    pub number: u8,
    pub draws: Vec<(u8, u8, u8)>,
}

fn parse_color(input: &str) -> IResult<&str, (u8, String)> {
    let (input, number) = parse_u8(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = parse_word(input)?;
    let (input, _) = opt(tag(", "))(input)?;
    Ok((input, (number, color)))
}

fn parse_draw(input: &str) -> IResult<&str, (u8, u8, u8)> {
    let (input, _) = tag(" ")(input)?;
    let (input, colors) = many0(parse_color)(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    let r = colors
        .iter()
        .filter(|(_, c)| c == "red")
        .map(|(n, _)| n)
        .cloned()
        .nth(0)
        .unwrap_or(0);
    let g = colors
        .iter()
        .filter(|(_, c)| c == "green")
        .map(|(n, _)| n)
        .cloned()
        .nth(0)
        .unwrap_or(0);
    let b = colors
        .iter()
        .filter(|(_, c)| c == "blue")
        .map(|(n, _)| n)
        .cloned()
        .nth(0)
        .unwrap_or(0);
    Ok((input, (r, g, b)))
}

fn parse_draws(input: &str) -> IResult<&str, Vec<(u8, u8, u8)>> {
    let (input, draws) = many0(parse_draw)(input)?;
    Ok((input, draws))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, number) = parse_u8(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, draws) = parse_draws(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, Game { number, draws }))
}

fn parse_data(input: &str) -> IResult<&str, Vec<Game>> {
    all_consuming(many0(parse_game))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<Game>> {
    let lines = read_lines((2023, 2), dataset_type)?;
    let data = lines.join("\n");
    // TODO: using `?` instead of `unwrap()` here causes the borrow-checker to lose its shit.
    // Figure it out
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}