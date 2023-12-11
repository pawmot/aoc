#[cfg(test)]
mod day15 {
    use anyhow::Result;
    use std::{
        cmp::{max, min},
        collections::HashSet,
    };

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day15::get_data,
    };

    fn manhattan_dist(pos1: (i32, i32), pos2: (i32, i32)) -> i32 {
        (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
    }

    fn distance_to_hline(pos: (i32, i32), hline_y: i32) -> i32 {
        (pos.1 - hline_y).abs()
    }

    fn a(data: Vec<((i32, i32), (i32, i32))>, sampling_y: i32) -> usize {
        let beacons_on_sampling_hline = data
            .iter()
            .map(|(_, b)| b)
            .filter(|b| b.1 == sampling_y)
            .collect::<HashSet<_>>()
            .len();
        let ranges = data
            .into_iter()
            .map(|(s, b)| (s, manhattan_dist(s, b)))
            //.map(|v| { println!("{:?}", v); v })
            .map(|(s, d)| {
                let dist_to_hline = distance_to_hline(s, sampling_y);
                (s, d - dist_to_hline)
            })
            .filter(|(_, d)| *d >= 0)
            .map(|(s, d)| (s.0 - d, s.0 + d))
            .collect::<Vec<_>>();

        let mut min_b = i32::MAX;
        let mut max_e = i32::MIN;

        for (b, e) in &ranges {
            min_b = min(min_b, *b);
            max_e = max(max_e, *e);
        }

        let positions_in_range_of_sensors = (min_b..=max_e)
            .filter(|x| {
                for (b, e) in &ranges {
                    if *b <= *x && *x <= *e {
                        return true;
                    }
                }

                return false;
            })
            .count();

        positions_in_range_of_sensors - beacons_on_sampling_hline
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data, 10), 26);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data, 2000000), 5040643);
        Ok(())
    }

    fn ranges_overlap(r1: (i32, i32), r2: (i32, i32)) -> bool {
        let (b1, e1) = r1;
        let (b2, e2) = r2;

        b2 <= e1 && b1 <= e2
    }

    fn combine_overlapping(ranges: Vec<(i32, i32)>) -> (i32, i32) {
        ranges.into_iter().fold((i32::MAX, i32::MIN), |acc, r| {
            (min(acc.0, r.0), max(acc.1, r.1))
        })
    }

    fn intersect_overlapping(r1: (i32, i32), r2: (i32, i32)) -> (i32, i32) {
        (max(r1.0, r2.0), min(r1.1, r2.1))
    }

    fn intersect_range_with_disjoint_ranges(
        range: (i32, i32),
        disjoint_ranges: Vec<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        disjoint_ranges
            .into_iter()
            .filter(|r| ranges_overlap(*r, range))
            .map(|r| intersect_overlapping(r, range))
            .collect()
    }

    fn b(data: Vec<((i32, i32), (i32, i32))>, search_space_size: i32) -> u64 {
        for y in 0..=search_space_size {
            let ranges = data
                .iter()
                .copied()
                .map(|(s, b)| (s, manhattan_dist(s, b)))
                //.map(|v| { println!("{:?}", v); v })
                .map(|(s, d)| {
                    let dist_to_hline = distance_to_hline(s, y);
                    (s, d - dist_to_hline)
                })
                .filter(|(_, d)| *d >= 0)
                .map(|(s, d)| (s.0 - d, s.0 + d))
                .collect::<Vec<_>>();

            let mut combined_ranges = vec![];
            for r in ranges {
                let (mut overlapping, non_overlapping): (Vec<_>, Vec<_>) = combined_ranges
                    .into_iter()
                    .partition(|cr| ranges_overlap(r, *cr));
                overlapping.push(r);
                combined_ranges = non_overlapping;
                combined_ranges.push(combine_overlapping(overlapping));
            }

            let mut search_space_result =
                intersect_range_with_disjoint_ranges((0, search_space_size), combined_ranges);

            if search_space_result.len() != 1 || search_space_result[0] != (0, search_space_size) {
                search_space_result.sort_by_key(|r| r.1);
                let x;
                if search_space_result[0].0 != 0 {
                    x = 0;
                } else {
                    x = search_space_result[0].1 + 1;
                }

                return x as u64 * 4000000 + y as u64;
            }
        }

        panic!("Exhausted search with no result");
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data, 20), 56000011);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data, 4000000), 11016575214126);
        Ok(())
    }
}