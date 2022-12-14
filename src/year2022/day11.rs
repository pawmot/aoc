#[cfg(test)]
mod day11 {
    use std::io;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day11::{get_data, Monkey},
    };

    fn a(mut data: Vec<Monkey>) -> usize {
        let monkey_cnt = data.len();
        let mut inspection_cnt = vec![0; monkey_cnt];

        for _ in 0..20 {
            for idx in 0..monkey_cnt {
                let monkey = &mut data[idx];
                let mut thrown_items = vec![vec![]; monkey_cnt];
                inspection_cnt[idx] += monkey.items.len();
                let items = &mut monkey.items;
                items.reverse();

                while let Some(item) = items.pop() {
                    let worry_level = (monkey.operation)(item) / 3;
                    if worry_level % monkey.test_mod == 0 {
                        thrown_items[monkey.true_dest].push(worry_level);
                    } else {
                        thrown_items[monkey.false_dest].push(worry_level);
                    }
                }

                // TODO: why does this work without dropping `monkey`?
                for (idx, mut items) in thrown_items.into_iter().enumerate() {
                    data[idx].items.append(&mut items);
                }
            }
        }

        inspection_cnt.sort();
        inspection_cnt.pop().unwrap() * inspection_cnt.pop().unwrap()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 10605);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 57348);
        Ok(())
    }

    fn b(mut data: Vec<Monkey>) -> usize {
        let monkey_cnt = data.len();
        let module: u64 = data.iter().map(|m| m.test_mod).product();
        let mut inspection_cnt = vec![0; monkey_cnt];

        for _ in 0..10000 {
            for idx in 0..monkey_cnt {
                let monkey = &mut data[idx];
                let mut thrown_items = vec![vec![]; monkey_cnt];
                inspection_cnt[idx] += monkey.items.len();
                let items = &mut monkey.items;
                items.reverse();

                while let Some(item) = items.pop() {
                    let worry_level = (monkey.operation)(item);
                    let worry_level = worry_level % module;
                    if worry_level % monkey.test_mod == 0 {
                        thrown_items[monkey.true_dest].push(worry_level);
                    } else {
                        thrown_items[monkey.false_dest].push(worry_level);
                    }
                }

                // TODO: why does this work without dropping `monkey`?
                for (idx, mut items) in thrown_items.into_iter().enumerate() {
                    data[idx].items.append(&mut items);
                }
            }
        }

        inspection_cnt.sort();
        inspection_cnt.pop().unwrap() * inspection_cnt.pop().unwrap()
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 2713310158);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 14106266886);
        Ok(())
    }
}
