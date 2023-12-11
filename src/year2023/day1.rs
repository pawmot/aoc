#[cfg(test)]
mod day1 {
    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day1::get_data,
    };
    use std::io;

    fn a(data: Vec<String>) -> u32 {
        data.into_iter()
            .map(|str| str.chars().collect::<Vec<char>>())
            .map(|chars| {
                chars
                    .into_iter()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>()
            })
            .map(|digits| {
                (
                    digits.first().unwrap().clone(),
                    digits.last().unwrap().clone(),
                )
            })
            .map(|(first, last)| first * 10 + last)
            .sum()
    }

    #[test]
    fn a_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample1")))?;
        println!("{:?}", data);

        assert_eq!(a(data), 142);
        Ok(())
    }

    #[test]
    fn a_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 55130);
        Ok(())
    }

    fn b(data: Vec<String>) -> u32 {
        data.into_iter()
            .map(|str| str.chars().collect::<Vec<char>>())
            .map(chars_to_digits)
            .map(|digits| {
                (
                    digits.first().unwrap().clone(),
                    digits.last().unwrap().clone(),
                )
            })
            .map(|(first, last)| first * 10 + last)
            .sum()
    }

    fn chars_to_digits(chars: Vec<char>) -> Vec<u32> {
        let mut digits = Vec::new();
        let mut idx: usize = 0;
        loop {
            if let Some(digit) = chars[idx].to_digit(10) {
                digits.push(digit);
                idx += 1;
            } else if find_word(&chars, idx, "one") {
                digits.push(1);
                idx += 2;
            } else if find_word(&chars, idx, "two") {
                digits.push(2);
                idx += 2;
            } else if find_word(&chars, idx, "three") {
                digits.push(3);
                idx += 4;
            } else if find_word(&chars, idx, "four") {
                digits.push(4);
                idx += 4;
            } else if find_word(&chars, idx, "five") {
                digits.push(5);
                idx += 3;
            } else if find_word(&chars, idx, "six") {
                digits.push(6);
                idx += 3;
            } else if find_word(&chars, idx, "seven") {
                digits.push(7);
                idx += 4;
            } else if find_word(&chars, idx, "eight") {
                digits.push(8);
                idx += 4;
            } else if find_word(&chars, idx, "nine") {
                digits.push(9);
                idx += 3;
            } else {
                idx += 1;
            }

            if idx >= chars.len() {
                break;
            }
        }
        digits
    }

    fn find_word(chars: &Vec<char>, start: usize, word: &str) -> bool {
        let mut idx: usize = 0;
        let word_chars: Vec<_> = word.chars().collect();
        loop {
            if idx >= word.len() {
                break;
            }

            if start + idx >= chars.len() {
                return false;
            }

            if chars[start + idx] == word_chars[idx] {
                idx += 1;
            } else {
                return false;
            }
        }
        true
    }

    #[test]
    fn b_sample() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("sample2")))?;

        assert_eq!(b(data), 281);
        Ok(())
    }

    #[test]
    fn b_full() -> io::Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 54985);
        Ok(())
    }
}