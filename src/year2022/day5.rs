#[cfg(test)]
mod day5 {
    use std::io;

    use crate::{
        data::{
            common::DatasetType::{FULL, SAMPLE},
            year2022::day5::get_data,
        },
        utils::borrow2_from_vec_mut,
    };

    fn a(data: (Vec<Vec<char>>, Vec<(u8, u8, u8)>)) -> String {
        let (mut stacks, moves) = data;

        for m in moves {
            let (n, f, t) = m;
            let f_idx = (f - 1) as usize;
            let t_idx = (t - 1) as usize;

            let (source, target) = borrow2_from_vec_mut(&mut stacks, f_idx, t_idx);
            let source_cutoff_idx = source.len() - n as usize;

            // drain returns a double-ended iter so it can be reversed cheaply
            target.extend(source.drain(source_cutoff_idx..).rev());
        }

        stacks.into_iter().map(|mut s| s.pop().unwrap()).collect()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), "CMZ");
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), "SBPQRSCDF");
        Ok(())
    }

    fn b(data: (Vec<Vec<char>>, Vec<(u8, u8, u8)>)) -> String {
        let (mut stacks, moves) = data;

        for m in moves {
            let (n, f, t) = m;
            let f_idx = (f - 1) as usize;
            let t_idx = (t - 1) as usize;

            let (source, target) = borrow2_from_vec_mut(&mut stacks, f_idx, t_idx);
            let source_cutoff_idx = source.len() - n as usize;

            target.extend(source.drain(source_cutoff_idx..));
        }

        stacks.into_iter().map(|mut s| s.pop().unwrap()).collect()
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), "MCD");
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), "RGLVRCQSB");
        Ok(())
    }
}
