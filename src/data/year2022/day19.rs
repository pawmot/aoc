use anyhow::Result;

use nom::{
    bytes::complete::tag,
    combinator::all_consuming,
    multi::many0,
    IResult,
};

use crate::data::common::{parse_new_line, read_lines, DatasetType, parse_u16};

fn parse_blueprint_header(input: &str) -> IResult<&str, u16> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, id) = parse_u16(input)?;
    let (input, _) = tag(": ")(input)?;
    Ok((input, id))
}

fn parse_ore_robot_cost(input: &str) -> IResult<&str, (u16, u16, u16)> {
    let (input, _) = tag("Each ore robot costs ")(input)?;
    let (input, ore) = parse_u16(input)?;
    let (input, _) = tag(" ore. ")(input)?;
    Ok((input, (ore, 0, 0)))
}

fn parse_clay_robot_cost(input: &str) -> IResult<&str, (u16, u16, u16)> {
    let (input, _) = tag("Each clay robot costs ")(input)?;
    let (input, ore) = parse_u16(input)?;
    let (input, _) = tag(" ore. ")(input)?;
    Ok((input, (ore, 0, 0)))
}

fn parse_obsidian_robot_cost(input: &str) -> IResult<&str, (u16, u16, u16)> {
    let (input, _) = tag("Each obsidian robot costs ")(input)?;
    let (input, ore) = parse_u16(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, clay) = parse_u16(input)?;
    let (input, _) = tag(" clay. ")(input)?;
    Ok((input, (ore, clay, 0)))
}

fn parse_geode_robot_cost(input: &str) -> IResult<&str, (u16, u16, u16)> {
    let (input, _) = tag("Each geode robot costs ")(input)?;
    let (input, ore) = parse_u16(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obsidian) = parse_u16(input)?;
    let (input, _) = tag(" obsidian.")(input)?;
    Ok((input, (ore, 0, obsidian)))
}

fn parse_blueprint(input: &str) -> IResult<&str, (u16, (u16, u16, u16), (u16, u16, u16), (u16, u16, u16), (u16, u16, u16))> {
    let (input, id) = parse_blueprint_header(input)?;
    let (input, ore_robot_cost) = parse_ore_robot_cost(input)?;
    let (input, clay_robot_cost) = parse_clay_robot_cost(input)?;
    let (input, obsidian_robot_cost) = parse_obsidian_robot_cost(input)?;
    let (input, geode_robot_cost) = parse_geode_robot_cost(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, (id, ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost)))
}

fn parse_data(input: &str) -> IResult<&str, Vec<(u16, (u16, u16, u16), (u16, u16, u16), (u16, u16, u16), (u16, u16, u16))>> {
    all_consuming(many0(parse_blueprint))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<(u16, (u16, u16, u16), (u16, u16, u16), (u16, u16, u16), (u16, u16, u16))>> {
    let lines = read_lines((2022, 19), dataset_type)?;
    let data = lines.join("\n");
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}

