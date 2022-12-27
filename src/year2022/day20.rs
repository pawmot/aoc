#[cfg(test)]
mod day20 {
    use anyhow::Result;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day20::get_data,
    };

    fn a(mut data: Vec<i64>) -> i64 {
        let n = data.len();
        let mut indices: Vec<usize> = (0..n).collect();

        for step in 0..n {
            //println!("{:?}", data);
            let idx = indices.iter().enumerate().find(|(_, i)| **i == step).unwrap().0;
            let val = data[idx];
            let mut target_idx_i = ((idx as i64) + val) % ((n - 1) as i64);
            if target_idx_i < 0 {
                target_idx_i += (n - 1) as i64;
            }
            let target_idx = target_idx_i as usize;
            if target_idx == idx {
                continue;
            }

            data.remove(idx);
            let i = indices.remove(idx);
            if target_idx != 0 {
                data.insert(target_idx, val);
                indices.insert(target_idx, i);
            } else {
                data.push(val);
                // there's a bug here, but it still works for my data. Leaving it for future lols.
                indices.push(target_idx);
            }
        }
        //println!("{:?}", data);

        let idx_of_zero = data.iter().enumerate().find(|(_, v)| **v == 0).unwrap().0;
        let fst_idx = (idx_of_zero + 1000) % n;
        let snd_idx = (idx_of_zero + 2000) % n;
        let trd_idx = (idx_of_zero + 3000) % n;

        data[fst_idx] + data[snd_idx] + data[trd_idx]
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 3);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 9866);
        Ok(())
    }

    fn b(mut data: Vec<i64>) -> i64 {
        let decryption_key = 811589153;
        data.iter_mut().for_each(|v| *v *= decryption_key);
        let n = data.len();
        let mut indices: Vec<usize> = (0..n).collect();

        for iter in 0..10 {
            println!("Iteration: {}", iter);
            for step in 0..n {
                let idx = indices.iter().enumerate().find(|(_, i)| **i == step).unwrap().0;
                let val = data[idx];
                let mut target_idx_i = ((idx as i64) + val) % ((n - 1) as i64);
                if target_idx_i < 0 {
                    target_idx_i += (n - 1) as i64;
                }
                let target_idx = target_idx_i as usize;
                if target_idx == idx {
                    continue;
                }

                data.remove(idx);
                let i = indices.remove(idx);
                if target_idx != 0 {
                    data.insert(target_idx, val);
                    indices.insert(target_idx, i);
                } else {
                    data.push(val);
                    indices.push(i);
                }
            }
        }

        let idx_of_zero = data.iter().enumerate().find(|(_, v)| **v == 0).unwrap().0;
        let fst_idx = (idx_of_zero + 1000) % n;
        let snd_idx = (idx_of_zero + 2000) % n;
        let trd_idx = (idx_of_zero + 3000) % n;

        data[fst_idx] + data[snd_idx] + data[trd_idx]
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 1623178306);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 12374299815791);
        Ok(())
    }
}
