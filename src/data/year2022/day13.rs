use anyhow::Result;
use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::data::common::{parse_u16, read_lines, DatasetType};

#[derive(Debug, PartialEq, Clone)]
pub enum PacketData {
    N(u16),
    L(Vec<PacketData>),
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketData::N(n) => f.write_str(&format!("{}", n)),
            PacketData::L(v) => f.write_str(&format!(
                "[{}]",
                v.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )),
        }
    }
}

fn parse_packetdata_number(input: &str) -> IResult<&str, PacketData> {
    let (input, number) = parse_u16(input)?;
    let (input, _) = opt(tag(","))(input)?;
    Ok((input, PacketData::N(number)))
}

fn parse_packetdata_list(input: &str) -> IResult<&str, PacketData> {
    let (input, data) = delimited(
        tag("["),
        many0(alt((parse_packetdata_number, parse_packetdata_list))),
        tag("]"),
    )(input)?;
    let (input, _) = opt(tag(","))(input)?;
    Ok((input, PacketData::L(data)))
}

fn parse_packet(input: &str) -> IResult<&str, PacketData> {
    let (input, packet) = parse_packetdata_list(input)?;
    let (input, _) = tag("\n")(input)?;
    Ok((input, packet))
}

fn parse_pair(input: &str) -> IResult<&str, (PacketData, PacketData)> {
    let (input, packets) = tuple((parse_packet, parse_packet))(input)?;
    let (input, _) = opt(tag("\n"))(input)?;
    Ok((input, packets))
}

fn parse_data(input: &str) -> IResult<&str, Vec<(PacketData, PacketData)>> {
    all_consuming(many0(parse_pair))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<(PacketData, PacketData)>> {
    let lines = read_lines((2022, 13), dataset_type)?;
    let data = lines.join("\n");
    // TODO: using `?` instead of `unwrap()` here causes the borrow-checker to lose its shit.
    // Figure it out
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
