#[cfg(test)]
mod day8 {
    use std::collections::{HashMap, HashSet};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day8::get_data,
    };
    use anyhow::Result;
    use num::integer::lcm;

    fn a(data: (String, Vec<(String, String, String)>)) -> usize {
        let mut map: HashMap<String, (String, String)> = HashMap::new();

        for (from, left, right) in data.1 {
            map.insert(from, (left, right));
        }

        let mut current = "AAA".to_owned();
        let mut step = 0;
        let instructions = data.0.chars().collect::<Vec<_>>();
        let instruction_len = instructions.len();

        while current != "ZZZ" {
            let (left, right) = map.get(&current).unwrap();
            let mov = instructions[step % instruction_len];
            if mov == 'L' {
                current = left.to_owned();
            } else {
                current = right.to_owned();
            }
            step += 1
        }

        step
    }

    #[test]
    fn a_sample1() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;

        assert_eq!(a(data), 2);
        Ok(())
    }

    #[test]
    fn a_sample2() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample2")))?;

        assert_eq!(a(data), 6);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 11309);
        Ok(())
    }

    fn b(data: (String, Vec<(String, String, String)>)) -> usize {
        let mut map: HashMap<String, (String, String)> = HashMap::new();

        for (from, left, right) in data.1 {
            map.insert(from, (left, right));
        }

        let starting_points = map
            .keys()
            .filter(|k| k.ends_with("A"))
            .cloned()
            .collect::<Vec<_>>();
        let mut steps: HashSet<usize> = HashSet::new();
        let instructions = data.0.chars().collect::<Vec<_>>();
        let instruction_len = instructions.len();

        for start in starting_points {
            let mut current = start;
            let mut step = 0;

            while !current.ends_with("Z") {
                let (left, right) = map.get(&current).unwrap();
                let mov = instructions[step % instruction_len];
                if mov == 'L' {
                    current = left.to_owned();
                } else {
                    current = right.to_owned();
                }
                step += 1
            }

            steps.insert(step);
        }

        steps.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample3")))?;

        assert_eq!(b(data), 6);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 13740108158591);
        Ok(())
    }
}
