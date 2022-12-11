use crate::data::common::{read_lines, DatasetType};
use std::io;

pub struct Monkey {
    pub items: Vec<u64>,
    pub operation: Box<dyn Fn(u64) -> u64>,
    pub test_mod: u64,
    pub true_dest: usize,
    pub false_dest: usize,
}

pub fn get_data(dataset_type: DatasetType) -> io::Result<Vec<Monkey>> {
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
}
