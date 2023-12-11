#[cfg(test)]
mod day4 {
    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2015::day4::get_data,
    };
    use std::io;

    fn a(data: String) -> u32 {
        let mut cursor = 0;
        loop {
            let hash = md5::compute(format!("{}{}", data, cursor));
            let hash_str = format!("{:x}", hash);
            if hash_str.starts_with("00000") {
                return cursor;
            }
            cursor += 1;
        }
    }

    #[test]
    fn a_sample0() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample0")))?;

        assert_eq!(a(data), 609043);
        Ok(())
    }

    #[test]
    fn a_sample1() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;

        assert_eq!(a(data), 1048970);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 254575);
        Ok(())
    }

    fn b(data: String) -> u32 {
        let mut cursor = 0;
        loop {
            let hash = md5::compute(format!("{}{}", data, cursor));
            let hash_str = format!("{:x}", hash);
            if hash_str.starts_with("000000") {
                return cursor;
            }
            cursor += 1;
        }
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 1038736);
        Ok(())
    }
}
