use anyhow::Result;
use nom::{
    character::complete::char,
    combinator::all_consuming,
    multi::many0,
    IResult, sequence::tuple, bytes::complete::take_until,
};

use crate::data::common::{read_lines, DatasetType, parse_new_line, parse_u32};

pub struct Hand {
    pub cards: String,
    pub bid: u32,
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, _, bid, _)) = tuple((take_until(" "), char(' '), parse_u32, parse_new_line))(input)?;
    Ok((input, Hand { cards: cards.to_string(), bid }))
}

fn parse_data(input: &str) -> IResult<&str, Vec<Hand>> {
    all_consuming(many0(parse_hand))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<Hand>> {
    let lines = read_lines((2023, 7), dataset_type)?;
    let data = lines.join("\n");
    // TODO: using `?` instead of `unwrap()` here causes the borrow-checker to lose its shit.
    // Figure it out
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
