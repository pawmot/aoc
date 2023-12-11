#[cfg(test)]
mod day10 {
    use std::collections::{HashSet, VecDeque};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day10::get_data,
    };
    use anyhow::Result;

    type Position = (usize, usize);

    fn move_in_pipeline(data: &Vec<Vec<char>>, ((prev_row, prev_col), cur @ (cur_row, cur_col)): (Position, Position)) -> Option<(Position, Position)> {
        let pipe = data[cur_row][cur_col];
        match pipe {
            '.' => None,
            '|' => {
                if prev_row == cur_row {
                    None
                } else if prev_row < cur_row {
                    if cur_row < data.len() -1 {
                        Some((cur, (cur_row + 1, cur_col)))
                    } else {
                        None
                    }
                } else {
                    if cur_row > 0 {
                        Some((cur, (cur_row -1, cur_col)))
                    } else {
                        None
                    }
                }
            },
            '-' => {
                if prev_col == cur_col {
                    None
                } else if prev_col < cur_col {
                    if cur_col < data[cur_row].len() - 1 {
                        Some((cur, (cur_row, cur_col + 1)))
                    } else {
                        None
                    }
                } else {
                    if cur_col > 0 {
                        Some((cur, (cur_row, cur_col - 1)))
                    } else {
                        None
                    }
                }
            },
            'F' => {
                if prev_col < cur_col || prev_row < cur_row {
                    None
                } else if prev_row > cur_row {
                    if cur_col < data[cur_row].len() - 1 {
                        Some((cur, (cur_row, cur_col + 1)))
                    } else {
                        None
                    }
                } else {
                    if cur_row < data.len() - 1 {
                        Some((cur, (cur_row + 1, cur_col)))
                    } else {
                        None
                    }
                }
            },
            '7' => {
                if prev_col > cur_col || prev_row < cur_row {
                    None
                } else if prev_row > cur_row {
                    if cur_col > 0 {
                        Some((cur, (cur_row, cur_col - 1)))
                    } else {
                        None
                    }
                } else {
                    if cur_row < data.len() - 1 {
                        Some((cur, (cur_row + 1, cur_col)))
                    } else {
                        None
                    }
                }
            },
            'J' => {
                if prev_col > cur_col || prev_row > cur_row {
                    None
                } else if prev_row < cur_row {
                    if cur_col > 0 {
                        Some((cur, (cur_row, cur_col -1)))
                    } else {
                        None
                    }
                } else {
                    if cur_row > 0 {
                        Some((cur, (cur_row - 1, cur_col)))
                    } else {
                        None
                    }
                }
            },
            'L' => {
                if prev_col < cur_col || prev_row > cur_row {
                    None
                } else if prev_row < cur_row {
                    if cur_col < data[cur_row].len() - 1 {
                        Some((cur, (cur_row, cur_col + 1)))
                    } else {
                        None
                    }
                } else {
                    if cur_row > 0 {
                        Some((cur, (cur_row - 1, cur_col)))
                    } else {
                        None
                    }
                }
            },
            'S' => panic!("Shouldn't be here"),
            _ => panic!("Unkonwn symbol: {}", pipe)
        }
    }

    fn find_start(data: &Vec<Vec<char>>) -> Position {
        for row in 0..data.len() {
            for col in 0..data[row].len() {
                if data[row][col] == 'S' {
                    return (row, col);
                }
            }
        }
        panic!("No start found");
    }

    fn a(data: Vec<Vec<char>>) -> usize {
        let start @ (start_row, start_col) = find_start(&data);
        let mut path_heads: Vec<(Position, Position)> = vec![];
        if start_row > 0 {
            path_heads.push((start, (start_row - 1, start_col)));
        }
        if start_row < data.len() - 1 {
            path_heads.push((start, (start_row + 1, start_col)));
        }
        if start_col > 0 {
            path_heads.push((start, (start_row, start_col - 1)));
        }
        if start_col < data[0].len() - 1 {
            path_heads.push((start, (start_row, start_col + 1)));
        }
        let mut step = 1;

        'outer: loop {
            path_heads = path_heads.into_iter().filter_map(|p| move_in_pipeline(&data, p)).collect();

            let mut prev_set: HashSet<Position> = HashSet::new();
            for (prev, _) in &path_heads {
                if !prev_set.insert(*prev) {
                    break 'outer;
                }
            }

            step += 1;
        }

        step
    }

    #[test]
    fn a_sample1() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;

        assert_eq!(a(data), 4);
        Ok(())
    }

    #[test]
    fn a_sample2() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample2")))?;

        assert_eq!(a(data), 8);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 6831);
        Ok(())
    }

    fn build_path_if_positive_oriented(data: &Vec<Vec<char>>, head: (Position, Position)) -> Option<Vec<(Position, Position)>> {
        let head = (head.1, head.0);
        let mut path = vec![head];
        let mut rightmost_col_on_path = 0;
        let mut rightmost_vertical_segment_on_path = None;

        loop {
            let next @ ((_, pc), (nr, nc)) = move_in_pipeline(data, path[path.len() - 1]).unwrap();
            path.push(next);
            if pc == nc && nc > rightmost_col_on_path {
                rightmost_col_on_path = nc;
                rightmost_vertical_segment_on_path = Some(next);
            }
            if data[nr][nc] == 'S' {
                break;
            }
        }

        let ((fr, _), (tr, _)) = rightmost_vertical_segment_on_path.unwrap();

        if fr > tr {
            Some(path)
        } else {
            None
        }
    }

    fn flood(data: &Vec<Vec<char>>, inside: &mut HashSet<Position>, on_path: &HashSet<Position>, start: Position) {
        let mut q = VecDeque::new();
        q.push_back(start);

        while let Some(cur @ (cur_row, cur_col)) = q.pop_front() {
            if on_path.contains(&cur) || !inside.insert(cur) {
                continue;
            }

            if cur_row > 0 {
                q.push_back((cur_row - 1, cur_col));
            }
            if cur_row < data.len() - 1 {
                q.push_back((cur_row + 1, cur_col));
            }
            if cur_col > 0 {
                q.push_back((cur_row, cur_col - 1));
            }
            if cur_col < data[cur_row].len() - 1 {
                q.push_back((cur_row, cur_col + 1));
            }
        }
    }

    fn find_inside_area_of_pos_oriented_path(data: &Vec<Vec<char>>, path: &Vec<(Position, Position)>) -> usize {
        let mut inside: HashSet<Position> = HashSet::new();
        let on_path: HashSet<Position> = path.iter().map(|(_, p)| *p).collect();

        for (prev @ (prev_row, prev_col), cur @ (cur_row, cur_col)) in path {
            let pipe = data[*cur_row][*cur_col];
            match pipe {
                '|' => {
                    if prev_row < cur_row {
                        flood(&data, &mut inside, &on_path, (*cur_row, *cur_col + 1));
                    } else {
                        flood(&data, &mut inside, &on_path, (*cur_row, *cur_col - 1));
                    }
                },
                '-' => {
                    if prev_col < cur_col {
                        flood(&data, &mut inside, &on_path, (*cur_row - 1, *cur_col));
                    } else {
                        flood(&data, &mut inside, &on_path, (*cur_row + 1, *cur_col));
                    }
                },
                'F' => {
                    if prev_row > cur_row {
                        flood(&data, &mut inside, &on_path, (*cur_row, *cur_col - 1));
                        flood(&data, &mut inside, &on_path, (*cur_row - 1, *cur_col));
                    }
                },
                '7' => {
                    if prev_col < cur_col {
                        flood(&data, &mut inside, &on_path, (*cur_row, *cur_col + 1));
                        flood(&data, &mut inside, &on_path, (*cur_row - 1, *cur_col));
                    }
                },
                'J' => {
                    if prev_row < cur_row {
                        flood(&data, &mut inside, &on_path, (*cur_row, *cur_col + 1));
                        flood(&data, &mut inside, &on_path, (*cur_row + 1, *cur_col));
                    }
                },
                'L' => {
                    if prev_col > cur_col {
                        flood(&data, &mut inside, &on_path, (*cur_row, *cur_col -1));
                        flood(&data, &mut inside, &on_path, (*cur_row + 1, *cur_col));
                    }
                },
                'S' => { 
                    // do nothing, maybe it's enough 
                },
                _ => panic!("Unkonwn symbol: {}", pipe)
            }
        }

        inside.len()
    }

    fn b(data: Vec<Vec<char>>) -> usize {
        let start @ (start_row, start_col) = find_start(&data);
        let mut path_heads: Vec<(Position, Position)> = vec![];
        if start_row > 0 {
            path_heads.push((start, (start_row - 1, start_col)));
        }
        if start_row < data.len() - 1 {
            path_heads.push((start, (start_row + 1, start_col)));
        }
        if start_col > 0 {
            path_heads.push((start, (start_row, start_col - 1)));
        }
        if start_col < data[0].len() - 1 {
            path_heads.push((start, (start_row, start_col + 1)));
        }

        loop {
            path_heads = path_heads.into_iter().filter_map(|p| move_in_pipeline(&data, p)).collect();

            let mut cur_set: HashSet<char> = HashSet::new();
            for (_, cur) in &path_heads {
                cur_set.insert(data[cur.0][cur.1]);
            }
            if cur_set.contains(&'S') {
                break;
            }
        }

        assert_eq!(2, path_heads.len());
        
        let path_pos_oriented = build_path_if_positive_oriented(&data, path_heads[0]).unwrap_or_else(|| build_path_if_positive_oriented(&data, path_heads[1]).unwrap());

        //println!("{:?}", path_pos_oriented.into_iter().map(|(p, _)| p).collect::<Vec<_>>());

        find_inside_area_of_pos_oriented_path(&data, &path_pos_oriented)
    }

    #[test]
    fn b_sample3() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample3")))?;

        assert_eq!(b(data), 4);
        Ok(())
    }

    #[test]
    fn b_sample4() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample4")))?;

        assert_eq!(b(data), 8);
        Ok(())
    }

    #[test]
    fn b_sample5() -> Result<()> {
        let data = get_data(SAMPLE(Some("sample5")))?;

        assert_eq!(b(data), 10);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 305);
        Ok(())
    }
}
