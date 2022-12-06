#[cfg(test)]
mod day6 {
    use std::{collections::HashSet, io};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day6::get_data,
    };

    fn find_marker(data: String, marker_len: usize) -> usize {
        let marker = data
            .chars()
            .collect::<Vec<_>>()
            .windows(marker_len)
            .enumerate()
            .map(|(idx, slice)| (idx, HashSet::<char>::from_iter(slice.iter().copied())))
            .find(|(_, set)| set.len() == marker_len)
            .unwrap()
            .0;

        marker + marker_len
    }

    fn a(data: String) -> usize {
        find_marker(data, 4)
    }

    #[test]
    fn a_sample1() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;

        assert_eq!(a(data), 7);
        Ok(())
    }

    #[test]
    fn a_sample2() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample2")))?;

        assert_eq!(a(data), 5);
        Ok(())
    }

    #[test]
    fn a_sample3() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample3")))?;

        assert_eq!(a(data), 6);
        Ok(())
    }

    #[test]
    fn a_sample4() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample4")))?;

        assert_eq!(a(data), 10);
        Ok(())
    }

    #[test]
    fn a_sample5() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample5")))?;

        assert_eq!(a(data), 11);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 1625);
        Ok(())
    }

    fn b(data: String) -> usize {
        find_marker(data, 14)
    }

    #[test]
    fn b_sample6() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample6")))?;

        assert_eq!(b(data), 19);
        Ok(())
    }

    #[test]
    fn b_sample7() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample7")))?;

        assert_eq!(b(data), 23);
        Ok(())
    }

    #[test]
    fn b_sample8() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample8")))?;

        assert_eq!(b(data), 23);
        Ok(())
    }

    #[test]
    fn b_sample9() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample9")))?;

        assert_eq!(b(data), 29);
        Ok(())
    }

    #[test]
    fn b_sample10() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample10")))?;

        assert_eq!(b(data), 26);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 2250);
        Ok(())
    }
}
