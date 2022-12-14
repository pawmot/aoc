use std::fs::File;
use std::io::{self, BufRead};

use nom::bytes::complete::{tag, take_while1};
use nom::combinator::map_res;
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

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

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

pub fn parse_new_line(input: &str) -> IResult<&str, &str> {
    tag("\n")(input)
}
