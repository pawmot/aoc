use nom::{branch::alt, bytes::complete::tag, combinator::all_consuming, multi::many0, IResult};

use crate::data::common::{parse_new_line, parse_usize, read_lines, DatasetType};
use std::io;

pub enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

pub struct Coords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Coords {
        Coords { x, y }
    }
}

pub struct Instruction {
    pub instruction_type: InstructionType,
    pub start: Coords,
    pub end: Coords,
}

fn parse_turn_on_instruction_type(input: &str) -> IResult<&str, InstructionType> {
    let (input, _) = tag("turn on")(input)?;

    Ok((input, InstructionType::TurnOn))
}

fn parse_turn_off_instruction_type(input: &str) -> IResult<&str, InstructionType> {
    let (input, _) = tag("turn off")(input)?;

    Ok((input, InstructionType::TurnOff))
}

fn parse_toggle_instruction_type(input: &str) -> IResult<&str, InstructionType> {
    let (input, _) = tag("toggle")(input)?;

    Ok((input, InstructionType::Toggle))
}

fn parse_intruction_type(input: &str) -> IResult<&str, InstructionType> {
    alt((
        parse_turn_on_instruction_type,
        parse_turn_off_instruction_type,
        parse_toggle_instruction_type,
    ))(input)
}

fn parse_coords(input: &str) -> IResult<&str, Coords> {
    let (input, x) = parse_usize(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = parse_usize(input)?;

    Ok((input, Coords::new(x, y)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, instruction_type) = parse_intruction_type(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, start) = parse_coords(input)?;
    let (input, _) = tag(" through ")(input)?;
    let (input, end) = parse_coords(input)?;
    let (input, _) = parse_new_line(input)?;

    Ok((
        input,
        Instruction {
            instruction_type,
            start,
            end,
        },
    ))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    all_consuming(many0(parse_instruction))(input)
}

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<Instruction>> {
    let lines = read_lines((2015, 6), dataset_type)?;
    let data = lines.join("\n");

    let parsed = parse_instructions(&data).unwrap().1;

    Ok(parsed)
}
