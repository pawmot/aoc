#[cfg(test)]
mod day7 {
    use std::{cmp::min, collections::HashMap, io};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day7::{get_data, TermOutput},
    };

    fn a(data: Vec<TermOutput>) -> u32 {
        let mut dirs: HashMap<String, u32> = HashMap::new();
        let mut dir_stack = vec![];

        for output in data {
            match output {
                TermOutput::CdDown(name) => {
                    if name == "/" {
                        dir_stack.push(name.to_string());
                    } else {
                        let parent = dir_stack.last().unwrap();
                        if parent == "/" {
                            dir_stack.push("/".to_string() + &name);
                        } else {
                            dir_stack.push(parent.to_string() + "/" + &name);
                        }
                    }
                }
                TermOutput::CdUp => {
                    let child = dir_stack.pop().unwrap();
                    let parent = dir_stack.last().unwrap();
                    let size = *dirs.get(&child).unwrap();
                    dirs.entry(parent.to_string())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                }
                TermOutput::Ls => {
                    let dir = dir_stack.last().unwrap();
                    dirs.insert(dir.to_string(), 0);
                }
                TermOutput::Dir(_) => {}
                TermOutput::File(_, fsize) => {
                    let dir = dir_stack.last().unwrap();
                    dirs.entry(dir.to_string())
                        .and_modify(|size| *size += fsize)
                        .or_insert(fsize);
                }
                TermOutput::EOF => loop {
                    let dir = dir_stack.pop().unwrap();
                    let parent = dir_stack.last();
                    if parent.is_none() {
                        break;
                    }
                    let size = *dirs.get(&dir).unwrap();
                    dirs.entry(parent.unwrap().to_string())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                },
            }
        }

        let mut result = 0;

        for (_dir, s) in dirs {
            if s <= 100000 {
                result += s;
            }
        }

        result
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 95437);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 1543140);
        Ok(())
    }

    fn b(data: Vec<TermOutput>) -> u32 {
        let mut dirs: HashMap<String, u32> = HashMap::new();
        let mut dir_stack = vec![];

        for output in data {
            match output {
                TermOutput::CdDown(name) => {
                    if name == "/" {
                        dir_stack.push(name.to_string());
                    } else {
                        let parent = dir_stack.last().unwrap();
                        if parent == "/" {
                            dir_stack.push("/".to_string() + &name);
                        } else {
                            dir_stack.push(parent.to_string() + "/" + &name);
                        }
                    }
                }
                TermOutput::CdUp => {
                    let child = dir_stack.pop().unwrap();
                    let parent = dir_stack.last().unwrap();
                    let size = *dirs.get(&child).unwrap();
                    dirs.entry(parent.to_string())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                }
                TermOutput::Ls => {
                    let dir = dir_stack.last().unwrap();
                    dirs.insert(dir.to_string(), 0);
                }
                TermOutput::Dir(_) => {}
                TermOutput::File(_, fsize) => {
                    let dir = dir_stack.last().unwrap();
                    dirs.entry(dir.to_string())
                        .and_modify(|size| *size += fsize)
                        .or_insert(fsize);
                }
                TermOutput::EOF => loop {
                    let dir = dir_stack.pop().unwrap();
                    let parent = dir_stack.last();
                    if parent.is_none() {
                        break;
                    }
                    let size = *dirs.get(&dir).unwrap();
                    dirs.entry(parent.unwrap().to_string())
                        .and_modify(|s| *s += size)
                        .or_insert(size);
                },
            }
        }

        let total_disk_space = 70000000u32;
        let required_space = 30000000u32;

        let free_space = total_disk_space - dirs.get("/").unwrap();

        let mut min_dir = u32::MAX;
        for (_, s) in dirs {
            if free_space + s >= required_space {
                min_dir = min(min_dir, s);
            }
        }

        min_dir
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 24933642);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 1117448);
        Ok(())
    }
}
