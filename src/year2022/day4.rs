#[cfg(test)]
mod day4 {
    use std::io;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day4::get_data,
    };

    fn a(data: Vec<((u16, u16), (u16, u16))>) -> usize {
        data.into_iter()
            .filter(|(l, r)| (l.0 <= r.0 && l.1 >= r.1) || (r.0 <= l.0 && r.1 >= l.1))
            .count()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 2);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 456);
        Ok(())
    }

    fn b(data: Vec<((u16, u16), (u16, u16))>) -> usize {
        data.into_iter()
            .filter(|(l, r)| l.0 <= r.1 && r.0 <= l.1)
            .count()
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 4);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 808);
        Ok(())
    }
}
