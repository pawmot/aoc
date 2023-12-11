use std::fs::{File, self};
use std::io::{self, BufRead};

use nom::bytes::complete::{tag, take_while1, take_while_m_n, take_while};
use nom::combinator::{map_res, opt};
use nom::character::complete::char;
use nom::IResult;

pub enum DatasetType<'a> {
    SAMPLE(Option<&'a str>),
    FULL,
}

type ProblemNumber = (u16, u8);

pub fn read_lines(
    problem_number: ProblemNumber,
    dataset_type: DatasetType,
) -> io::Result<Vec<String>> {
    let path = match dataset_type {
        DatasetType::SAMPLE(filename) => {
            format!(
                "./data/{}/day{}/{}.txt",
                problem_number.0,
                problem_number.1,
                filename.unwrap_or("sample")
            )
        }
        DatasetType::FULL => format!(
            "./data/{}/day{}/full.txt",
            problem_number.0, problem_number.1
        ),
    };

    let file = File::open(path)?;
    let line_results = io::BufReader::new(file).lines();
    line_results.into_iter().collect::<io::Result<Vec<_>>>()
}

pub fn read_file(
    problem_number: ProblemNumber,
    dataset_type: DatasetType,
) -> io::Result<String> {
    let path = match dataset_type {
        DatasetType::SAMPLE(filename) => {
            format!(
                "./data/{}/day{}/{}.txt",
                problem_number.0,
                problem_number.1,
                filename.unwrap_or("sample")
            )
        }
        DatasetType::FULL => format!(
            "./data/{}/day{}/full.txt",
            problem_number.0, problem_number.1
        ),
    };

    fs::read_to_string(path)
}

pub fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

pub fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

pub fn is_letter_or_digit(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
}

// TODO: look into possibilities of making one polymorphic fn to cover all the number types
pub fn parse_u8(input: &str) -> IResult<&str, u8> {
    map_res(take_while1(is_digit), |i: &str| i.parse())(input)
}

pub fn parse_u16(input: &str) -> IResult<&str, u16> {
    map_res(take_while1(is_digit), |i: &str| i.parse())(input)
}

pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(take_while1(is_digit), |i: &str| i.parse())(input)
}

pub fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(take_while1(is_digit), |i: &str| i.parse())(input)
}

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(is_digit), |i: &str| i.parse())(input)
}

pub fn parse_i16(input: &str) -> IResult<&str, i16> {
    let (input, minus) = opt(tag("-"))(input)?;
    let (input, abs) = map_res(take_while1(is_digit), |i: &str| i.parse::<i16>())(input)?;

    if minus.is_some() {
        Ok((input, -abs))
    } else {
        Ok((input, abs))
    }
}

pub fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (input, minus) = opt(tag("-"))(input)?;
    let (input, abs) = map_res(take_while1(is_digit), |i: &str| i.parse::<i32>())(input)?;

    if minus.is_some() {
        Ok((input, -abs))
    } else {
        Ok((input, abs))
    }
}

pub fn parse_i64(input: &str) -> IResult<&str, i64> {
    let (input, minus) = opt(tag("-"))(input)?;
    let (input, abs) = map_res(take_while1(is_digit), |i: &str| i.parse::<i64>())(input)?;

    if minus.is_some() {
        Ok((input, -abs))
    } else {
        Ok((input, abs))
    }
}

/// Parses a word, which is defined as a non-empty sequence of letters and digits starting with a
/// letter
pub fn parse_word(input: &str) -> IResult<&str, String> {
    let (input, head) = take_while_m_n(1, 1, is_letter)(input)?;
    let (input, tail) = take_while(is_letter_or_digit)(input)?;

    Ok((input, format!("{}{}", head, tail)))
}

pub fn parse_new_line(input: &str) -> IResult<&str, char> {
    char('\n')(input)
}