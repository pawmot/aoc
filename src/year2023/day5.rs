#[cfg(test)]
mod day5 {
    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2023::day5::{get_data, Input, Map},
    };
    use anyhow::Result;

    fn map_elem(elem: i64, map: &Map) -> i64 {
        map.iter()
            .find(|(_, src, cnt)| elem >= *src && elem < *src + *cnt)
            .map(|(dst, src, _)| dst + (elem - src))
            .unwrap_or(elem)
    }

    fn a(data: Input) -> i64 {
        data.seeds
            .into_iter()
            .map(|seed| map_elem(seed, &data.seed_to_soil_map))
            .map(|soil| map_elem(soil, &data.soil_to_fertilizer_map))
            .map(|fertilizer| map_elem(fertilizer, &data.fertilizer_to_water_map))
            .map(|water| map_elem(water, &data.water_to_light_map))
            .map(|light| map_elem(light, &data.light_to_temparature_map))
            .map(|temperature| map_elem(temperature, &data.temperature_to_humidity_map))
            .map(|humidity| map_elem(humidity, &data.humidity_to_location_map))
            .min()
            .unwrap()
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 35);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 993500720);
        Ok(())
    }

    fn map_range(range: (i64, i64), map: &Map) -> Vec<(i64, i64)> {
        let mut to_map = vec![range];
        let mut mapped = vec![];

        for (dst_from, src_from, cnt) in map.into_iter().cloned() {
            let (map_to, diff) = (src_from + cnt - 1, dst_from - src_from);
            let mut to_map_new = vec![];
            for rng @ (from, to) in to_map {
                if to < src_from || from > map_to {
                    to_map_new.push(rng);
                } else if from < src_from {
                    to_map_new.push((from, src_from - 1));
                    if to <= map_to {
                        mapped.push((dst_from, to + diff));
                    } else {
                        mapped.push((dst_from, map_to + diff));
                        to_map_new.push((map_to + 1, to));
                    }
                } else {
                    if to <= map_to {
                        mapped.push((from + diff, to + diff));
                    } else {
                        mapped.push((from + diff, map_to + diff));
                        to_map_new.push((map_to + 1, to));
                    }
                }
            }
            to_map = to_map_new;
        }

        mapped.append(&mut to_map);
        mapped
    }

    fn b(data: Input) -> i64 {
        data.seeds
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
            .flat_map(|range| map_range(range, &data.seed_to_soil_map))
            .flat_map(|range| map_range(range, &data.soil_to_fertilizer_map))
            .flat_map(|range| map_range(range, &data.fertilizer_to_water_map))
            .flat_map(|range| map_range(range, &data.water_to_light_map))
            .flat_map(|range| map_range(range, &data.light_to_temparature_map))
            .flat_map(|range| map_range(range, &data.temperature_to_humidity_map))
            .flat_map(|range| map_range(range, &data.humidity_to_location_map))
            .map(|(from, _)| from)
            .min()
            .unwrap()
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 46);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 4917124);
        Ok(())
    }
}