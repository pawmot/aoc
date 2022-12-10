#[cfg(test)]
mod day9 {
    use std::{
        cmp::{max, min},
        collections::{HashMap, HashSet},
        io,
    };

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day9::get_data,
    };

    fn move_knot(pos: (i16, i16), motion: (i16, i16)) -> (i16, i16) {
        (pos.0 + motion.0, pos.1 + motion.1)
    }

    fn move_head(pos: (i16, i16), dir: char) -> (i16, i16) {
        let d: (i16, i16) = match dir {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!(),
        };

        move_knot(pos, d)
    }

    fn manhattan_dist(pos1: (i16, i16), pos2: (i16, i16)) -> i16 {
        (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
    }

    fn a(data: Vec<(char, u16)>) -> usize {
        let mut h: (i16, i16) = (0, 0);
        let mut t: (i16, i16) = (0, 0);
        let mut visited: HashSet<(i16, i16)> = HashSet::new();
        visited.insert(t);

        for (dir, cnt) in data {
            for _ in 0..cnt {
                let last_h = h;
                h = move_head(h, dir);
                if (h.0 - t.0).abs() > 1 || (h.1 - t.1).abs() > 1 {
                    t = last_h;
                    visited.insert(t);
                }
            }
        }

        visited.len()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;

        assert_eq!(a(data), 13);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 6314);
        Ok(())
    }

    fn debug_print(knots: &Vec<(i16, i16)>) {
        let mut min_x = i16::MAX;
        let mut max_x = i16::MIN;
        let mut min_y = i16::MAX;
        let mut max_y = i16::MIN;

        for k in knots.iter().copied() {
            min_x = min(min_x, k.0);
            max_x = max(max_x, k.0);
            min_y = min(min_y, k.1);
            max_y = max(max_y, k.1);
        }

        //min_x -= 4;
        //max_x += 4;
        //min_y -= 4;
        //max_y += 4;

        let mut translated_knots = knots
            .into_iter()
            .map(|pos| (pos.0 - min_x, pos.1 - min_y))
            .enumerate()
            .map(|(n, pos)| {
                (
                    if n == 0 {
                        "H".to_string()
                    } else {
                        n.to_string()
                    },
                    pos,
                )
            })
            .collect::<Vec<_>>();
        translated_knots.reverse();

        let mut pos_to_name = HashMap::new();

        for (name, pos) in translated_knots {
            pos_to_name.insert(pos, name);
        }

        for y in (min_y..=max_y).rev() {
            let mut line = String::from("");
            for x in min_x..=max_x {
                let char = pos_to_name
                    .get(&(x, y))
                    .map(|s| String::from(s))
                    .unwrap_or(String::from("."));
                line += &char;
            }
            println!("{}", line);
        }

        println!();
    }

    fn b(data: Vec<(char, u16)>) -> usize {
        let mut knots: Vec<(i16, i16)> = vec![(0, 0); 10];
        let mut visited: HashSet<(i16, i16)> = HashSet::new();
        visited.insert(knots.last().copied().unwrap());
        debug_print(&knots);

        for (dir, cnt) in data {
            for _ in 0..cnt {
                let mut new_knots: Vec<(i16, i16)> = vec![];
                let h = move_head(knots[0], dir);
                new_knots.push(h);
                let mut prev_move_diagonal = false;
                let mut diagonal_motion: (i16, i16) = (0, 0);
                for (idx, k) in knots.iter().copied().enumerate().skip(1) {
                    let prev = new_knots[idx - 1];
                    if (prev.0 - k.0).abs() <= 1 && (prev.1 - k.1).abs() <= 1 {
                        new_knots.push(k);
                    } else {
                        if !prev_move_diagonal {
                            let new_k = knots[idx - 1];
                            new_knots.push(new_k);
                            if manhattan_dist(k, new_k) == 2 {
                                prev_move_diagonal = true;
                                diagonal_motion = (new_k.0 - k.0, new_k.1 - k.1);
                            }
                        } else {
                            if k.0 == prev.0 {
                                new_knots
                                    .push(move_knot(k, (0, (prev.1 - k.1) / (prev.1 - k.1).abs())));
                                prev_move_diagonal = false;
                            } else if k.1 == prev.1 {
                                new_knots
                                    .push(move_knot(k, ((prev.0 - k.0) / (prev.0 - k.0).abs(), 0)));
                                prev_move_diagonal = false;
                            } else {
                                new_knots.push(move_knot(k, diagonal_motion));
                            }
                        }
                    }
                }

                visited.insert(new_knots.last().copied().unwrap());
                knots = new_knots;
                debug_print(&knots);
            }
        }

        visited.len()
    }

    #[test]
    fn b_sample1() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;

        assert_eq!(b(data), 1);
        Ok(())
    }

    #[test]
    fn b_sample2() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample2")))?;

        assert_eq!(b(data), 36);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 2504);
        Ok(())
    }
}
