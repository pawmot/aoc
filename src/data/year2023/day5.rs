use anyhow::Result;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::char,
    combinator::all_consuming,
    multi::{many0, separated_list1},
    IResult, sequence::tuple,
};

use crate::data::common::{read_lines, DatasetType, parse_new_line, parse_i64};

pub type Mapping = (i64, i64, i64);
pub type Map = Vec<Mapping>;

pub struct Input {
    pub seeds: Vec<i64>,
    pub seed_to_soil_map: Map,
    pub soil_to_fertilizer_map: Map,
    pub fertilizer_to_water_map: Map,
    pub water_to_light_map: Map,
    pub light_to_temparature_map: Map,
    pub temperature_to_humidity_map: Map,
    pub humidity_to_location_map: Map,
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(tag(" "), parse_i64)(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, seeds))
}

fn parse_i64_with_space(input: &str) -> IResult<&str, i64> {
    let (input, i) = parse_i64(input)?;
    let (input, _) = char(' ')(input)?;
    Ok((input, i))
}

fn parse_i64_with_new_line(input: &str) -> IResult<&str, i64> {
    let (input, i) = parse_i64(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, i))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = parse_new_line(input)?;
    let (input, mappings) = many0(tuple((parse_i64_with_space, parse_i64_with_space, parse_i64_with_new_line)))(input)?;
    Ok((input, mappings))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, (seeds, _, seed_to_soil_map, _, soil_to_fertilizer_map, _, fertilizer_to_water_map, _, water_to_light_map, _, light_to_temparature_map, _, temperature_to_humidity_map, _, humidity_to_location_map)) = tuple((
        parse_seeds,
        parse_new_line,
        parse_map,
        parse_new_line,
        parse_map,
        parse_new_line,
        parse_map,
        parse_new_line,
        parse_map,
        parse_new_line,
        parse_map,
        parse_new_line,
        parse_map,
        parse_new_line,
        parse_map,
    ))(input)?;

    Ok((
        input,
        Input {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temparature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        },
    ))
}

fn parse_data(input: &str) -> IResult<&str, Input> {
    all_consuming(parse_input)(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Input> {
    let lines = read_lines((2023, 5), dataset_type)?;
    let data = lines.join("\n");
    // TODO: using `?` instead of `unwrap()` here causes the borrow-checker to lose its shit.
    // Figure it out
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
