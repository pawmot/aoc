use anyhow::Result;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, opt},
    multi::many0,
    IResult, sequence::tuple,
};

use crate::data::common::{parse_u8, read_lines, DatasetType, parse_new_line};

pub struct Card {
    pub number: u8,
    pub numbers: Vec<u8>,
    pub winning_numbers: Vec<u8>,
}

fn parse_number(input: &str) -> IResult<&str, u8> {
    let (input, (_, number, _)) = tuple((opt(tag(" ")), parse_u8, opt(tag(" "))))(input)?;
    Ok((input, number))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, (_, numbers)) = tuple((opt(tag(" ")), many0(parse_number)))(input)?;
    Ok((input, numbers))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (_, (_, number), _, numbers, _, winning_numbers, _)) = tuple((
        tag("Card "),
        tuple((many0(tag(" ")), parse_u8)),
        tag(":"),
        parse_numbers,
        tag("|"),
        parse_numbers,
        parse_new_line
    ))(input)?;
    Ok((input, Card { number, numbers, winning_numbers }))
}

fn parse_data(input: &str) -> IResult<&str, Vec<Card>> {
    all_consuming(many0(parse_card))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<Card>> {
    let lines = read_lines((2023, 4), dataset_type)?;
    let data = lines.join("\n");
    // TODO: using `?` instead of `unwrap()` here causes the borrow-checker to lose its shit.
    // Figure it out
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
