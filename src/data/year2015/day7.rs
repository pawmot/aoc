use std::io;

use nom::{combinator::all_consuming, multi::many0, IResult, branch::alt, bytes::complete::tag};

use crate::data::common::{DatasetType, read_lines, parse_u8, parse_word, parse_new_line};

pub enum ConstOrWire {
    Const(u8),
    Wire(String),
}

pub enum Operation {
    Assign(ConstOrWire),
    And(ConstOrWire, ConstOrWire),
    Or(ConstOrWire, ConstOrWire),
    LShift(String, u8),
    RShift(String, u8),
}

pub struct Instruction {
    pub operation: Operation,
    pub target: String,
}

fn parse_const(input: &str) -> IResult<&str, ConstOrWire> {
    let (_, num) = parse_u8(input)?;

    Ok((input, ConstOrWire::Const(num)))
}

fn parse_wire(input: &str) -> IResult<&str, ConstOrWire> {
    let (input, wire) = parse_word(input)?;

    Ok((input, ConstOrWire::Wire(wire)))
}

fn parse_const_or_wire(input: &str) -> IResult<&str, ConstOrWire> {
    let (input, const_or_wire) = alt((parse_const, parse_wire))(input)?;

    Ok((input, const_or_wire))
}

fn parse_assign_operation(input: &str) -> IResult<&str, Operation> {
    let (input, const_or_wire) = parse_const_or_wire(input)?;

    Ok((input, Operation::Assign(const_or_wire)))
}

fn parse_and_operation(input: &str) -> IResult<&str, Operation> {
    let (input, const_or_wire1) = parse_const_or_wire(input)?;
    let (input, _) = tag(" AND ")(input)?;
    let (input, const_or_wire2) = parse_const_or_wire(input)?;

    Ok((input, Operation::And(const_or_wire1, const_or_wire2)))
}

fn parse_or_operation(input: &str) -> IResult<&str, Operation> {
    let (input, const_or_wire1) = parse_const_or_wire(input)?;
    let (input, _) = tag(" OR ")(input)?;
    let (input, const_or_wire2) = parse_const_or_wire(input)?;

    Ok((input, Operation::And(const_or_wire1, const_or_wire2)))
}

fn parse_lshift_operation(input: &str) -> IResult<&str, Operation> {
    let (input, wire1) = parse_word(input)?;
    let (input, _) = tag(" LSHIFT ")(input)?;
    let (input, shift) = parse_u8(input)?;

    Ok((input, Operation::LShift(wire1, shift)))
}

fn parse_rshift_operation(input: &str) -> IResult<&str, Operation> {
    let (input, wire1) = parse_word(input)?;
    let (input, _) = tag(" RSHIFT ")(input)?;
    let (input, shift) = parse_u8(input)?;

    Ok((input, Operation::LShift(wire1, shift)))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, op) = alt((parse_assign_operation, parse_and_operation, parse_or_operation, parse_lshift_operation, parse_rshift_operation))(input)?;

    Ok((input, op))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, op) = parse_operation(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, wire) = parse_word(input)?;
    let (input, _) = parse_new_line(input)?;

    Ok((input, Instruction { operation: op, target: wire }))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    all_consuming(many0(parse_instruction))(input)
}

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<Instruction>> {
    let lines = read_lines((2015, 7), dataset_type)?;
    let data = lines.join("\n");

    let parsed = parse_instructions(&data).unwrap().1;

    Ok(parsed)
}