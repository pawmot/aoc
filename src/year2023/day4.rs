#[cfg(test)]
mod day4 {
    use std::{collections::HashSet, cmp::min};

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day4::{get_data, Card},
    };
    use anyhow::Result;
    use num::pow;

    fn a(data: Vec<Card>) -> u32 {
        data.into_iter()
            .map(
                 |card| (card.numbers, card.winning_numbers.into_iter().collect::<HashSet<_>>()),
            )
            .map(|(numbers, winning_numbers)| {
                numbers
                    .into_iter()
                    .filter(|number| winning_numbers.contains(number))
                    .count()
            })
            .filter(|count| *count > 0)
            .map(|count| pow(2, count - 1))
            .sum()
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 13);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 24175);
        Ok(())
    }

    fn b(data: Vec<Card>) -> u32 {
        let mut number_of_cards = vec![1; data.len()];

        for i in 0..data.len() {
            let Card { number: _, numbers, winning_numbers } = &data[i];
            let winning_numbers_set = winning_numbers.into_iter().copied().collect::<HashSet<_>>();

            let count = numbers.into_iter().filter(|n| winning_numbers_set.contains(*n)).count();
            
            for idx in (i+1)..=min(i+count, data.len() - 1) {
                number_of_cards[idx] += number_of_cards[i];
            }
        }

        number_of_cards.into_iter().sum()
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 30);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 18846301);
        Ok(())
    }
}