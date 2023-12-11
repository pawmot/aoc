#[cfg(test)]
mod day7 {
    use std::collections::HashMap;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day7::{get_data, Hand},
    };
    use anyhow::Result;

    #[derive(PartialEq, PartialOrd, Eq, Ord)]
    enum HandType {
        HighCard = 0,
        OnePair = 1,
        TwoPairs = 2,
        ThreeOfAKind = 3,
        FullHouse = 4,
        FourOfAKind = 5,
        FiveOfAKind = 6
    }

    fn find_hand_type_no_joker(hand: &Hand) -> HandType {
        use HandType::*;
        let mut card_counts: HashMap<char, usize> = HashMap::new();

        for card in hand.cards.chars() {
            card_counts.entry(card).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        if card_counts.len() == 1 {
            FiveOfAKind
        } else if card_counts.len() == 2 {
            let mut values = card_counts.values().cloned().collect::<Vec<_>>();
            values.sort();
            if values[0] == 1 {
                FourOfAKind
            } else {
                FullHouse
            }
        } else if card_counts.len() == 3 {
            let mut values = card_counts.values().cloned().collect::<Vec<_>>();
            values.sort();
            if values[2] == 3 {
                ThreeOfAKind
            } else {
                TwoPairs
            }
        } else if card_counts.len() == 4 {
            OnePair
        } else {
            HighCard
        }
    }

    fn map_card_to_value_no_joker(card: char) -> u32 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }

    fn compare_hands_card_by_card_no_joker(a: &Hand, b: &Hand) -> std::cmp::Ordering {
        let cards_a = a.cards.chars().map(map_card_to_value_no_joker).collect::<Vec<_>>();
        let cards_b = b.cards.chars().map(map_card_to_value_no_joker).collect::<Vec<_>>();

        for (card_a, card_b) in cards_a.iter().zip(cards_b.iter()) {
            if card_a > card_b {
                return std::cmp::Ordering::Greater;
            } else if card_a < card_b {
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    }

    fn a(mut data: Vec<Hand>) -> u32 {
        data.sort_by(compare_hands_card_by_card_no_joker);
        data.sort_by_key(find_hand_type_no_joker);
        data.into_iter()
            .enumerate()
            .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
            .sum()
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 6440);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 248396258);
        Ok(())
    }

    fn find_hand_type_joker(hand: &Hand) -> HandType {
        use HandType::*;
        let mut card_counts: HashMap<char, usize> = HashMap::new();

        for card in hand.cards.chars() {
            card_counts.entry(card).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        let joker_cnt = if card_counts.contains_key(&'J') {
            card_counts.remove(&'J').unwrap()
        } else {
            0
        };

        if card_counts.len() == 1 || card_counts.len() == 0 {
            FiveOfAKind
        } else if card_counts.len() == 2 {
            let mut values = card_counts.values().cloned().collect::<Vec<_>>();
            values.sort();
            if values[0] == 1 {
                FourOfAKind
            } else {
                FullHouse
            }
        } else if card_counts.len() == 3 {
            let mut values = card_counts.values().cloned().collect::<Vec<_>>();
            values.sort();
            if values[2] + joker_cnt == 3 {
                ThreeOfAKind
            } else {
                TwoPairs
            }
        } else if card_counts.len() == 4 {
            OnePair
        } else {
            HighCard
        }
    }

    fn map_card_to_value_joker(card: char) -> u32 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }

    fn compare_hands_card_by_card_joker(a: &Hand, b: &Hand) -> std::cmp::Ordering {
        let cards_a = a.cards.chars().map(map_card_to_value_joker).collect::<Vec<_>>();
        let cards_b = b.cards.chars().map(map_card_to_value_joker).collect::<Vec<_>>();

        for (card_a, card_b) in cards_a.iter().zip(cards_b.iter()) {
            if card_a > card_b {
                return std::cmp::Ordering::Greater;
            } else if card_a < card_b {
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    }

    fn b(mut data: Vec<Hand>) -> u32 {
        data.sort_by(compare_hands_card_by_card_joker);
        data.sort_by_key(find_hand_type_joker);
        data.into_iter()
            .enumerate()
            .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
            .sum()
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 5905);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 246436046);
        Ok(())
    }
}
