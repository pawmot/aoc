#[cfg(test)]
mod day13 {
    use anyhow::Result;
    use std::{cmp::Ordering, io};

    use itertools::{
        EitherOrBoth::{Both, Left, Right},
        Itertools,
    };

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day13::{
            get_data, PacketData,
            PacketData::{L, N},
        },
    };

    fn compare(pair: (PacketData, PacketData)) -> Ordering {
        match pair {
            (N(l), N(r)) => l.cmp(&r),
            (l @ L(_), r @ N(_)) => compare((l, L(vec![r]))),
            (l @ N(_), r @ L(_)) => compare((L(vec![l]), r)),
            (L(left), L(right)) => {
                let result = left
                    .into_iter()
                    .zip_longest(right.into_iter())
                    .map(|eob| match eob {
                        Left(_) => Ordering::Greater,
                        Right(_) => Ordering::Less,
                        Both(l, r) => compare((l, r)),
                    })
                    .find(|ord| *ord != Ordering::Equal);

                result.unwrap_or(Ordering::Equal)
            }
        }
    }

    fn a(data: Vec<(PacketData, PacketData)>) -> usize {
        data.into_iter()
            .enumerate()
            .map(|(i, p)| (i, compare(p)))
            .filter(|(_, ord)| *ord == Ordering::Less)
            .map(|(i, _)| i + 1)
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

        assert_eq!(a(data), 5529);
        Ok(())
    }

    fn b(data: Vec<(PacketData, PacketData)>) -> usize {
        let mut data = data
            .into_iter()
            .flat_map(|(l, r)| vec![l, r].into_iter())
            .collect::<Vec<_>>();
        let div2 = L(vec![L(vec![N(2)])]);
        let div6 = L(vec![L(vec![N(6)])]);
        data.push(div2.clone());
        data.push(div6.clone());

        data.sort_by(|a, b| compare((a.clone(), b.clone())));

        let pos2 = data.iter().find_position(|p| **p == div2).unwrap().0 + 1;
        let pos6 = data.iter().find_position(|p| **p == div6).unwrap().0 + 1;

        pos2 * pos6
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 140);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 27690);
        Ok(())
    }
}
