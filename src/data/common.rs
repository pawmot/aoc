use std::fs::File;
use std::io::{self, BufRead};

pub enum DatasetType<'a> {
    #[allow(dead_code)]
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
