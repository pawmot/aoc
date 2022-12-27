#[cfg(test)]
mod day17 {
    use anyhow::Result;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day17::get_data,
    };

    fn a(data: Vec<char>) -> usize {
        let mut lines: Vec<Vec<char>> = vec![];

        let mut move_idx = 0;

        for n in 0..2022 {
            let shape = match n % 5 {
                0 => vec!["  @@@@ "],
                1 => vec!["   @   ", "  @@@  ", "   @   "],
                2 => vec!["  @@@  ", "    @  ", "    @  "],
                3 => vec!["  @    ", "  @    ", "  @    ", "  @    "],
                4 => vec!["  @@   ", "  @@   "],
                _ => panic!(),
            };

            let shape_height = shape.len();

            lines.push("       ".chars().collect());
            lines.push("       ".chars().collect());
            lines.push("       ".chars().collect());

            lines.extend(shape.into_iter().map(|s| s.chars().collect()));
            let mut shape_bottom_row = lines.len() - shape_height;

            loop {
                let wind_dir = data[move_idx];
                move_idx += 1;
                move_idx %= data.len();

                let mut can_move_sideways = true;
                'move_sideways_check: for line_idx in
                    shape_bottom_row..(shape_bottom_row + shape_height)
                {
                    for (ci, _) in lines[line_idx]
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| **c == '@')
                    {
                        if (wind_dir == '<' && (ci == 0 || lines[line_idx][ci - 1] == '#'))
                            || (wind_dir == '>' && (ci == 6 || lines[line_idx][ci + 1] == '#'))
                        {
                            can_move_sideways = false;
                            break 'move_sideways_check;
                        }
                    }
                }

                if can_move_sideways {
                    let idx_range = shape_bottom_row..(shape_bottom_row + shape_height);

                    for row_idx in idx_range {
                        let mut new_line = vec![];
                        for idx in 0..7 {
                            if lines[row_idx][idx] == '#' {
                                new_line.push('#');
                            } else if (wind_dir == '<' && idx < 6 && lines[row_idx][idx + 1] == '@')
                                || (wind_dir == '>' && idx > 0 && lines[row_idx][idx - 1] == '@')
                            {
                                new_line.push('@');
                            } else {
                                new_line.push(' ');
                            }
                        }
                        lines[row_idx] = new_line;
                    }
                }

                let mut can_move_down = true;
                if shape_bottom_row == 0 {
                    can_move_down = false;
                } else {
                    'move_down_check: for line_idx in
                        shape_bottom_row..(shape_bottom_row + shape_height)
                    {
                        for (ci, _) in lines[line_idx]
                            .iter()
                            .enumerate()
                            .filter(|(_, c)| **c == '@')
                        {
                            if lines[line_idx - 1][ci] == '#' {
                                can_move_down = false;
                                break 'move_down_check;
                            }
                        }
                    }
                }

                if can_move_down {
                    let idx_range = (shape_bottom_row - 1)..(shape_bottom_row + shape_height);
                    for idx in idx_range {
                        let mut new_line = vec![];
                        for ci in 0..7 {
                            if lines[idx][ci] == '#' {
                                new_line.push('#');
                            } else if idx + 1 < lines.len() && lines[idx + 1][ci] == '@' {
                                new_line.push('@');
                            } else {
                                new_line.push(' ');
                            }
                        }
                        lines[idx] = new_line;
                    }

                    if lines[lines.len() - 1].iter().copied().all(|c| c == ' ') {
                        lines.pop();
                    }

                    shape_bottom_row -= 1;
                } else {
                    let idx_range = (shape_bottom_row)..(shape_bottom_row + shape_height);
                    for idx in idx_range {
                        let mut new_line = vec![];
                        for ci in 0..7 {
                            if lines[idx][ci] == '@' {
                                new_line.push('#');
                            } else {
                                new_line.push(lines[idx][ci]);
                            }
                        }
                        lines[idx] = new_line;
                    }
                    break;
                }
            }
        }

        lines.len()
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 3068);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 3081);
        Ok(())
    }

    fn b(data: Vec<char>, iter: usize) -> usize {
        let mut lines: Vec<Vec<char>> = vec![];
        let mut height_diffs: Vec<usize> = vec![];

        let mut move_idx = 0;

        for n in 0..10000 {
            let init_height = lines.len();
            let shape = match n % 5 {
                0 => vec!["  @@@@ "],
                1 => vec!["   @   ", "  @@@  ", "   @   "],
                2 => vec!["  @@@  ", "    @  ", "    @  "],
                3 => vec!["  @    ", "  @    ", "  @    ", "  @    "],
                4 => vec!["  @@   ", "  @@   "],
                _ => panic!(),
            };

            let shape_height = shape.len();

            lines.push("       ".chars().collect());
            lines.push("       ".chars().collect());
            lines.push("       ".chars().collect());

            lines.extend(shape.into_iter().map(|s| s.chars().collect()));
            let mut shape_bottom_row = lines.len() - shape_height;

            loop {
                let wind_dir = data[move_idx];
                move_idx += 1;
                move_idx %= data.len();

                let mut can_move_sideways = true;
                'move_sideways_check: for line_idx in
                    shape_bottom_row..(shape_bottom_row + shape_height)
                {
                    for (ci, _) in lines[line_idx]
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| **c == '@')
                    {
                        if (wind_dir == '<' && (ci == 0 || lines[line_idx][ci - 1] == '#'))
                            || (wind_dir == '>' && (ci == 6 || lines[line_idx][ci + 1] == '#'))
                        {
                            can_move_sideways = false;
                            break 'move_sideways_check;
                        }
                    }
                }

                if can_move_sideways {
                    let idx_range = shape_bottom_row..(shape_bottom_row + shape_height);

                    for row_idx in idx_range {
                        let mut new_line = vec![];
                        for idx in 0..7 {
                            if lines[row_idx][idx] == '#' {
                                new_line.push('#');
                            } else if (wind_dir == '<' && idx < 6 && lines[row_idx][idx + 1] == '@')
                                || (wind_dir == '>' && idx > 0 && lines[row_idx][idx - 1] == '@')
                            {
                                new_line.push('@');
                            } else {
                                new_line.push(' ');
                            }
                        }
                        lines[row_idx] = new_line;
                    }
                }

                let mut can_move_down = true;
                if shape_bottom_row == 0 {
                    can_move_down = false;
                } else {
                    'move_down_check: for line_idx in
                        shape_bottom_row..(shape_bottom_row + shape_height)
                    {
                        for (ci, _) in lines[line_idx]
                            .iter()
                            .enumerate()
                            .filter(|(_, c)| **c == '@')
                        {
                            if lines[line_idx - 1][ci] == '#' {
                                can_move_down = false;
                                break 'move_down_check;
                            }
                        }
                    }
                }

                if can_move_down {
                    let idx_range = (shape_bottom_row - 1)..(shape_bottom_row + shape_height);
                    for idx in idx_range {
                        let mut new_line = vec![];
                        for ci in 0..7 {
                            if lines[idx][ci] == '#' {
                                new_line.push('#');
                            } else if idx + 1 < lines.len() && lines[idx + 1][ci] == '@' {
                                new_line.push('@');
                            } else {
                                new_line.push(' ');
                            }
                        }
                        lines[idx] = new_line;
                    }

                    if lines[lines.len() - 1].iter().copied().all(|c| c == ' ') {
                        lines.pop();
                    }

                    shape_bottom_row -= 1;
                } else {
                    let idx_range = (shape_bottom_row)..(shape_bottom_row + shape_height);
                    for idx in idx_range {
                        let mut new_line = vec![];
                        for ci in 0..7 {
                            if lines[idx][ci] == '@' {
                                new_line.push('#');
                            } else {
                                new_line.push(lines[idx][ci]);
                            }
                        }
                        lines[idx] = new_line;
                    }
                    break;
                }
            }
            height_diffs.push(lines.len() - init_height);
        }

        let mut cycle_length = 0;
        'cycles: for cycle_len in 1..=3000 {
            let range = (height_diffs.len() - cycle_len, height_diffs.len());
            let candidate = &height_diffs[range.0..range.1];

            for n in 0..3 {
                let check_range = (range.0 - cycle_len * n, range.1 - cycle_len * n);
                let check = &height_diffs[check_range.0..check_range.1];

                for idx in 0..cycle_len {
                    if candidate[idx] != check[idx] {
                        continue 'cycles;
                    }
                }
            }

            println!("Found cycle with lenght {}", cycle_len);
            cycle_length = cycle_len;
            break;
        }

        if cycle_length == 0 {
            panic!("No cycle found");
        }

        let cycle = &height_diffs[(height_diffs.len() - cycle_length)..(height_diffs.len())];
        let cycle_total_height: usize = cycle.iter().cloned().sum();

        let iter_to_add = iter - 10000;
        lines.len() + cycle_total_height * (iter_to_add / cycle_length) + cycle.iter().take(iter_to_add % cycle_length).cloned().sum::<usize>()
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data, 1000000000000), 1514285714288);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data, 1000000000000), 1524637681145);
        Ok(())
    }
}
