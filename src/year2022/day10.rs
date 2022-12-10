#[cfg(test)]
mod day10 {
    use std::io;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day10::{get_data, Op},
    };

    fn a(data: Vec<Op>) -> i16 {
        let mut x = 1;
        let mut clock = 0;
        let mut sampling_ticks = vec![220, 180, 140, 100, 60, 20];

        let mut result = 0;

        for op in data {
            let sampling_tick = sampling_ticks.last().copied();
            if sampling_tick.is_none() {
                break;
            }

            clock += match op {
                Op::Noop => 1,
                Op::Addx(_) => 2,
            };

            if clock >= sampling_tick.unwrap() {
                result += sampling_tick.unwrap() * x;
                sampling_ticks.pop();
            }

            x += match op {
                Op::Noop => 0,
                Op::Addx(v) => v,
            }
        }

        result
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 13140);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 13060);
        Ok(())
    }

    fn b(data: Vec<Op>) -> String {
        let mut x = 1;
        let mut clock = 0;

        let mut result = String::from("");

        for op in data {
            let ticks = match op {
                Op::Noop => 1,
                Op::Addx(_) => 2,
            };

            for _ in 0..ticks {
                result += if clock % 40 >= x - 1 && clock % 40 < x + 2 {
                    "#"
                } else {
                    "."
                };
                clock += 1;
            }

            x += match op {
                Op::Noop => 0,
                Op::Addx(v) => v,
            }
        }

        result.truncate(240);

        let mut break_idx = 40;

        while break_idx < result.len() {
            result.insert(break_idx, '\n');
            break_idx += 41;
        }

        result
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(
            b(data),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(
            b(data),
            "####...##.#..#.###..#..#.#....###..####.
#.......#.#..#.#..#.#..#.#....#..#....#.
###.....#.#..#.###..#..#.#....#..#...#..
#.......#.#..#.#..#.#..#.#....###...#...
#....#..#.#..#.#..#.#..#.#....#.#..#....
#.....##...##..###...##..####.#..#.####."
        );
        Ok(())
    }
}
