use anyhow::Result;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    combinator::all_consuming,
    multi::many0,
    IResult,
};

use crate::data::common::{is_letter, parse_i64, parse_new_line, read_lines, DatasetType};

#[derive(PartialEq, Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

pub enum Expression {
    Num(i64),
    Operation(String, Operator, String)
}

fn parse_monkey_name(input: &str) -> IResult<&str, String> {
    let (input, name) = take_while(is_letter)(input)?;
    Ok((input, String::from(name)))
}

fn parse_monkey_header(input: &str) -> IResult<&str, String> {
    let (input, name) = parse_monkey_name(input)?;
    let (input, _) = tag(": ")(input)?;
    Ok((input, name))
}

fn parse_arithmetic_expression(input: &str) -> IResult<&str, Expression> {
    use Expression::Operation;
    use Operator::*;

    let (input, name_l) = parse_monkey_name(input)?;
    let (input, op) = take(3u8)(input)?;
    let (input, name_r) = parse_monkey_name(input)?;

    let name_left = String::from(name_l);
    let name_right = String::from(name_r);

    let expr = match op.chars().nth(1).unwrap() {
        '+' => Operation(name_left, Add, name_right),
        '-' => Operation(name_left, Sub, name_right),
        '*' => Operation(name_left, Mul, name_right),
        '/' => Operation(name_left, Div, name_right),
        _ => panic!(),
    };

    Ok((input, expr))
}

fn parse_number_expression(input: &str) -> IResult<&str, Expression> {
    let (input, num) = parse_i64(input)?;
    Ok((input, Expression::Num(num)))
}

fn parse_monkey_expression(input: &str) -> IResult<&str, Expression> {
    alt((parse_number_expression, parse_arithmetic_expression))(input)
}

fn parse_monkey(input: &str) -> IResult<&str, (String, Expression)> {
    let (input, name) = parse_monkey_header(input)?;
    let (input, expr) = parse_monkey_expression(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, (name, expr)))
}

fn parse_data(input: &str) -> IResult<&str, Vec<(String, Expression)>> {
    all_consuming(many0(parse_monkey))(input)
}

pub fn get_data(dataset_type: DatasetType) -> Result<Vec<(String, Expression)>> {
    let lines = read_lines((2022, 21), dataset_type)?;
    let data = lines.join("\n");
    let parsed = parse_data(&data).unwrap().1;
    Ok(parsed)
}
