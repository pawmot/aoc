#[cfg(test)]
mod day6 {
    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day6::{get_data, Race},
    };
    use anyhow::Result;

    fn compute_ways_to_win(race: Race) -> u32 {
        let mut ways_to_win = 0;
        for pressed_time in 0..=race.time {
            let time_left = race.time - pressed_time;
            let dist = pressed_time * time_left;
            if dist > race.record {
                ways_to_win += 1;
            }
        }
        ways_to_win
    }

    fn a(data: Vec<Race>) -> u32 {
        data.into_iter()
            .map(compute_ways_to_win)
            .product()
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 288);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 500346);
        Ok(())
    }

    fn compute_ways_to_win_long(time: u64, record: u64) -> u32 {
        let mut ways_to_win = 0;
        for pressed_time in 0..=time {
            let time_left = time - pressed_time;
            let dist = pressed_time * time_left;
            if dist > record {
                ways_to_win += 1;
            }
        }
        ways_to_win
    }

    fn b(data: Vec<Race>) -> u32 {
        let (time_str, record_str) = data.iter()
            .map(|Race { time, record }| (time, record))
            .fold(("".to_owned(), "".to_owned()), |(acc_time, acc_record), (time, record)| {
                (format!("{}{}", acc_time, time), format!("{}{}", acc_record, record))
            });
        let time = time_str.parse::<u64>().unwrap();
        let record = record_str.parse::<u64>().unwrap();
        compute_ways_to_win_long(time, record)
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 71503);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 42515755);
        Ok(())
    }
}