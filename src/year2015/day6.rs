#[cfg(test)]
mod day6 {
    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2015::day6::{get_data, Instruction, InstructionType},
    };
    use std::io;

    fn a(data: Vec<Instruction>) -> usize {
        let mut grid: [[bool; 1000]; 1000] = [[false; 1000]; 1000];

        data.iter().for_each(|Instruction { instruction_type, start, end }| {
            match instruction_type {
                InstructionType::TurnOn => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            grid[x][y] = true;
                        }
                    }
                },
                InstructionType::TurnOff => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            grid[x][y] = false;
                        }
                    }
                }
                InstructionType::Toggle => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            grid[x][y] = !grid[x][y];
                        }
                    }
                }
            }
        });

        (0..1000)
            .flat_map(move |x| (0..1000).map(move |y| (x, y)))
            .filter(|(x, y)| grid[*x][*y])
            .count()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;

        assert_eq!(a(data), 998996);

        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 543903);
        Ok(())
    }

    fn b(data: Vec<Instruction>) -> i32 {
        let mut grid = vec![[0; 1000]; 1000];

        data.iter().for_each(|Instruction { instruction_type, start, end }| {
            match instruction_type {
                InstructionType::TurnOn => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            grid[x][y] += 1;
                        }
                    }
                },
                InstructionType::TurnOff => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            if grid[x][y] > 0 {
                                grid[x][y] -= 1;
                            }
                        }
                    }
                }
                InstructionType::Toggle => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            grid[x][y] += 2;
                        }
                    }
                }
            }
        });

        (0usize..1000)
            .flat_map(move |x| (0usize..1000).map(move |y| (x, y)))
            .map(|(x, y)| grid[x][y])
            .sum()
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample2")))?;

        assert_eq!(b(data), 2000001i32);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 14687245);
        Ok(())
    }
}
