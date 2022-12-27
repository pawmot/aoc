#[cfg(test)]
mod day25 {
    use anyhow::Result;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day25::get_data,
    };

    fn from_snafu(snafu: String) -> u64 {
        let mut result = 0;
        for ch in snafu.chars() {
            result *= 5;
            result = match ch {
                '=' => result - 2,
                '-' => result - 1,
                '0' => result,
                '1' => result + 1,
                '2' => result + 2,
                _ => unreachable!(),
            }
        }
        result
    }

    fn to_snafu(mut num: u64) -> String {
        let mut digits = vec![];
        while num > 0 {
            digits.push(num % 5);
            num /= 5;
        }

        if digits.is_empty() {
            digits.push(0);
        }

        let mut chars = vec![];
        let mut n = 0;
        while n < digits.len() {
            let mut digit = digits[n];
            if digit > 4 {
                let carry = digit / 5;
                digit = digit % 5;
                if n < digits.len() - 1 {
                    digits[n + 1] = digits[n + 1] + carry;
                } else {
                    digits.push(carry);
                }
            }

            match digit {
                0 => chars.push('0'),
                1 => chars.push('1'),
                2 => chars.push('2'),
                3 => {
                    chars.push('=');
                    if n < digits.len() - 1 {
                        digits[n + 1] = digits[n + 1] + 1;
                    } else {
                        digits.push(1);
                    }
                }
                4 => {
                    chars.push('-');
                    if n < digits.len() - 1 {
                        digits[n + 1] = digits[n + 1] + 1;
                    } else {
                        digits.push(1);
                    }
                }
                _ => unreachable!(),
            }

            n += 1;
        }

        chars.into_iter().rev().collect()
    }

    fn a(data: Vec<String>) -> String {
        to_snafu(data.into_iter().map(from_snafu).sum())
    }

    fn get_brochure_data() -> Vec<(String, u64)> {
        vec![
            ("1".to_owned(), 1),
            ("2".to_owned(), 2),
            ("1=".to_owned(), 3),
            ("1-".to_owned(), 4),
            ("10".to_owned(), 5),
            ("11".to_owned(), 6),
            ("12".to_owned(), 7),
            ("2=".to_owned(), 8),
            ("2-".to_owned(), 9),
            ("20".to_owned(), 10),
            ("1=0".to_owned(), 15),
            ("1-0".to_owned(), 20),
            ("1=11-2".to_owned(), 2022),
            ("1-0---0".to_owned(), 12345),
            ("1121-1110-1=0".to_owned(), 314159265),
        ]
    }

    #[test]
    fn test_from_snafu() {
        for (str, num) in get_brochure_data() {
            assert_eq!(from_snafu(str), num);
        }
    }

    #[test]
    fn test_to_snafu() {
        for (str, num) in get_brochure_data() {
            assert_eq!(to_snafu(num), str);
        }
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), String::from("2=-1=0"));
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), String::from("2-=102--02--=1-12=22"));
        Ok(())
    }
}
