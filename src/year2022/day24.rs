#[cfg(test)]
mod day24 {
    use std::collections::{HashMap, HashSet, VecDeque};

    use anyhow::Result;
    use num::integer::lcm;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day24::{get_data, Blizzard, BlizzardDirection},
    };

    fn move_blizzards(blizzards: HashSet<Blizzard>, (h, w): (usize, usize)) -> HashSet<Blizzard> {
        use BlizzardDirection::*;
        blizzards
            .into_iter()
            .map(|((r, c), d)| match d {
                Up => {
                    let (mut nr, nc) = (r - 1, c);
                    if nr == 0 {
                        nr = h - 2;
                    }
                    ((nr, nc), d)
                }
                Down => {
                    let (mut nr, nc) = (r + 1, c);
                    if nr == h - 1 {
                        nr = 1;
                    }
                    ((nr, nc), d)
                }
                Left => {
                    let (nr, mut nc) = (r, c - 1);
                    if nc == 0 {
                        nc = w - 2;
                    }
                    ((nr, nc), d)
                }
                Right => {
                    let (nr, mut nc) = (r, c + 1);
                    if nc == w - 1 {
                        nc = 1;
                    }
                    ((nr, nc), d)
                }
            })
            .collect()
    }

    fn solve(
        data: (HashSet<Blizzard>, (usize, usize)),
        start: (usize, usize),
        end: (usize, usize),
        from_step: usize,
    ) -> usize {
        let (mut blizzards, (h, w)) = data;

        let mut current_step = 0;
        let mut blizzards_positions: HashSet<(usize, usize)> =
            blizzards.iter().map(|(pos, _)| pos).copied().collect();
        let mut q: VecDeque<((usize, usize), usize)> = VecDeque::from([(start, from_step)]);
        let period = lcm(h - 2, w - 2);
        let mut visited_states: HashSet<((usize, usize), usize)> = HashSet::new();

        while let Some((pos @ (r, c), step)) = q.pop_front() {
            let effective_step = step % period;

            if visited_states.contains(&(pos, effective_step)) {
                continue;
            }

            visited_states.insert((pos, effective_step));

            while current_step != step {
                blizzards = move_blizzards(blizzards, (h, w));
                blizzards_positions = blizzards.iter().map(|(pos, _)| pos).copied().collect();
                current_step += 1;
            }

            // up
            if (c == 1 && r >= 1) || (c != 1 && r >= 2) {
                let new_pos = (r - 1, c);
                if new_pos == end {
                    break;
                }
                if !blizzards_positions.contains(&new_pos) {
                    q.push_back((new_pos, step + 1));
                }
            }

            // down
            if (c == w - 2 && r <= h - 2) || (c != w - 2 && r <= h - 3) {
                let new_pos = (r + 1, c);
                if new_pos == end {
                    break;
                }
                if !blizzards_positions.contains(&new_pos) {
                    q.push_back((new_pos, step + 1));
                }
            }

            // left
            if c >= 2 && r > 0 && r < h - 1 {
                let new_pos = (r, c - 1);
                if !blizzards_positions.contains(&new_pos) {
                    q.push_back((new_pos, step + 1));
                }
            }

            // right
            if c <= w - 3 && r > 0 && r < h - 1 {
                let new_pos = (r, c + 1);
                if !blizzards_positions.contains(&new_pos) {
                    q.push_back((new_pos, step + 1));
                }
            }

            // in place
            if !blizzards_positions.contains(&pos) {
                q.push_back((pos, step + 1));
            }
        }

        current_step
    }

    fn a(data @ (_, (h, w)): (HashSet<Blizzard>, (usize, usize))) -> usize {
        solve(data, (0, 1), (h - 1, w - 2), 1)
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 18);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 292);
        Ok(())
    }

    fn b(data @ (_, (h, w)): (HashSet<Blizzard>, (usize, usize))) -> usize {
        let first_goal = solve(data.clone(), (0, 1), (h - 1, w - 2), 1);
        let back_to_start = solve(data.clone(), (h - 1, w - 2), (0, 1), first_goal + 1);
        let second_goal = solve(data, (0, 1), (h - 1, w - 2), back_to_start + 1);
        second_goal
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 54);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 816);
        Ok(())
    }
}
