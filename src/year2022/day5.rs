#[cfg(test)]
mod day4 {
    use std::io;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day5::get_data,
    };

    fn a(data: (Vec<Vec<char>>, Vec<(u8, u8, u8)>)) -> String {
        let (mut stacks, moves) = data;
        
        for m in moves {
            let (n, f, t) = m;

            for _ in 0..n {
                // TODO: fix this fugly code
                let source = &mut stacks[(f-1) as usize];
                let v = source.pop().unwrap();
                drop(source);
                let target = &mut stacks[(t-1) as usize];
                target.push(v);
            }
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

            let mut intermediate = vec![];
            let source = &mut stacks[(f-1) as usize];

            for _ in 0..n {
                let v = source.pop().unwrap();
                intermediate.push(v);
            }

            drop(source);
            let target = &mut stacks[(t-1) as usize];

            for _ in 0..n {
                let v = intermediate.pop().unwrap();
                target.push(v);
            }
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
