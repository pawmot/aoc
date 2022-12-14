use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until1, take_while, take_while1},
    combinator::{all_consuming, map, map_res, opt},
    multi::many0,
    IResult,
};

use crate::data::common::{
    parse_new_line, parse_u64, parse_u8, parse_usize, read_lines, DatasetType,
};
use std::io;

pub struct Monkey {
    pub items: Vec<u64>,
    pub operation: Box<dyn Fn(u64) -> u64>,
    pub test_mod: u64,
    pub true_dest: usize,
    pub false_dest: usize,
}

fn monkey_header(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, number) = parse_u8(input)?;
    let (input, _) = tag(":\n")(input)?;
    Ok((input, number))
}

fn monkey_item(input: &str) -> IResult<&str, u64> {
    let (input, item) = map_res(take_while1(|ch: char| ch.is_ascii_digit()), |i: &str| {
        i.parse()
    })(input)?;
    let (input, _) = opt(tag(", "))(input)?;
    Ok((input, item))
}

fn monkey_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = take_while(|ch: char| ch.is_whitespace())(input)?;
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, items) = many0(monkey_item)(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, items))
}

#[allow(dead_code)]
fn fn_add(term: u64) -> Box<dyn Fn(u64) -> u64> {
    Box::new(move |old| old + term)
}

fn fn_mul(multiplier: u64) -> Box<dyn Fn(u64) -> u64> {
    Box::new(move |old| old * multiplier)
}

fn fn_square() -> Box<dyn Fn(u64) -> u64> {
    Box::new(|old| old * old)
}

enum Operation {
    ADD,
    MUL,
}

fn operation_type(input: &str) -> IResult<&str, Operation> {
    let (input, op) = take(2u8)(input)?;
    Ok((
        input,
        match op.chars().next().unwrap() {
            '+' => Operation::ADD,
            '*' => Operation::MUL,
            _ => panic!(),
        },
    ))
}

enum Operand {
    NUM(u64),
    OLD,
}

fn operand(input: &str) -> IResult<&str, Operand> {
    let old_parser = map(tag("old"), |_| Operand::OLD);
    let num_parser = map(parse_u64, |n| Operand::NUM(n));
    alt((old_parser, num_parser))(input)
}

fn monkey_operation(input: &str) -> IResult<&str, Box<dyn Fn(u64) -> u64>> {
    let (input, _) = take_while(|ch: char| ch.is_whitespace())(input)?;
    let (input, _) = tag("Operation: new = old ")(input)?;
    let (input, operation) = operation_type(input)?;
    let (input, operand) = operand(input)?;
    let (input, _) = parse_new_line(input)?;

    let fun = match (operation, operand) {
        (Operation::ADD, Operand::NUM(n)) => fn_add(n),
        (Operation::ADD, Operand::OLD) => panic!(),
        (Operation::MUL, Operand::NUM(n)) => fn_mul(n),
        (Operation::MUL, Operand::OLD) => fn_square(),
    };

    Ok((input, fun))
}

fn monkey_test_mod(input: &str) -> IResult<&str, u64> {
    let (input, _) = take_while(|ch: char| ch.is_whitespace())(input)?;
    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, module) = parse_u64(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, module))
}

fn monkey_true_dest(input: &str) -> IResult<&str, usize> {
    let (input, _) = take_while(|ch: char| ch.is_whitespace())(input)?;
    let (input, _) = tag("If true: throw to monkey ")(input)?;
    let (input, number) = parse_usize(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, number))
}

fn monkey_false_dest(input: &str) -> IResult<&str, usize> {
    let (input, _) = take_while(|ch: char| ch.is_whitespace())(input)?;
    let (input, _) = tag("If false: throw to monkey ")(input)?;
    let (input, number) = parse_usize(input)?;
    let (input, _) = parse_new_line(input)?;
    Ok((input, number))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = monkey_header(input)?;
    let (input, items) = monkey_items(input)?;
    let (input, operation) = monkey_operation(input)?;
    let (input, test_mod) = monkey_test_mod(input)?;
    let (input, true_dest) = monkey_true_dest(input)?;
    let (input, false_dest) = monkey_false_dest(input)?;
    let (input, _) = opt(parse_new_line)(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation,
            test_mod,
            true_dest,
            false_dest,
        },
    ))
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    all_consuming(many0(monkey))(input)
}

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<Monkey>> {
    let lines = read_lines((2022, 11), dataset_type)?;
    let full_input = lines.join("\n");
    let parsed = monkeys(&full_input).unwrap().1;
    Ok(parsed)
}

/*pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<Monkey>> {
    match dataset_type {
        DatasetType::SAMPLE(None) => Ok(vec![
            Monkey {
                items: vec![79, 98],
                operation: Box::new(|old| old * 19),
                test_mod: 23,
                true_dest: 2,
                false_dest: 3,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Box::new(|old| old + 6),
                test_mod: 19,
                true_dest: 2,
                false_dest: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Box::new(|old| old * old),
                test_mod: 13,
                true_dest: 1,
                false_dest: 3,
            },
            Monkey {
                items: vec![74],
                operation: Box::new(|old| old + 3),
                test_mod: 17,
                true_dest: 0,
                false_dest: 1,
            },
        ]),
        DatasetType::FULL => Ok(vec![
            Monkey {
                items: vec![91, 58, 52, 69, 95, 54],
                operation: Box::new(|old| old * 13),
                test_mod: 7,
                true_dest: 1,
                false_dest: 5,
            },
            Monkey {
                items: vec![80, 80, 97, 84],
                operation: Box::new(|old| old * old),
                test_mod: 3,
                true_dest: 3,
                false_dest: 5,
            },
            Monkey {
                items: vec![86, 92, 71],
                operation: Box::new(|old| old + 7),
                test_mod: 2,
                true_dest: 0,
                false_dest: 4,
            },
            Monkey {
                items: vec![96, 90, 99, 76, 79, 85, 98, 61],
                operation: Box::new(|old| old + 4),
                test_mod: 11,
                true_dest: 7,
                false_dest: 6,
            },
            Monkey {
                items: vec![60, 83, 68, 64, 73],
                operation: Box::new(|old| old * 19),
                test_mod: 17,
                true_dest: 1,
                false_dest: 0,
            },
            Monkey {
                items: vec![96, 52, 52, 94, 76, 51, 57],
                operation: Box::new(|old| old + 3),
                test_mod: 5,
                true_dest: 7,
                false_dest: 3,
            },
            Monkey {
                items: vec![75],
                operation: Box::new(|old| old + 5),
                test_mod: 13,
                true_dest: 4,
                false_dest: 2,
            },
            Monkey {
                items: vec![83, 75],
                operation: Box::new(|old| old + 1),
                test_mod: 19,
                true_dest: 2,
                false_dest: 6,
            },
        ]),
        _ => panic!(),
    }
}*/
