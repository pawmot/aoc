#[cfg(test)]
mod day19 {
    use std::{cmp::max, collections::VecDeque};

    use anyhow::Result;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day19::get_data,
    };

    fn a(
        data: Vec<(
            u16,
            (u16, u16, u16),
            (u16, u16, u16),
            (u16, u16, u16),
            (u16, u16, u16),
        )>,
    ) -> u16 {
        let mut result = 0;
        for blueprint @ (
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        ) in data
        {
            println!("Processing: {:?}", blueprint);
            let mut max_geodes_mined = u16::MIN;
            let mut queue = VecDeque::from([((0, 0, 0, 0), (1, 0, 0, 0), 24)]);
            while let Some((
                (ore, clay, obsidian, geode),
                (ore_robots, clay_robots, obsidian_robots, geode_robots),
                minutes_left,
            )) = queue.pop_front()
            {
                max_geodes_mined = max(max_geodes_mined, geode + geode_robots * minutes_left);

                if minutes_left == 0 {
                    continue;
                }

                if ore >= ore_robot_cost.0 {
                    queue.push_back((
                        (
                            ore + ore_robots - ore_robot_cost.0,
                            clay + clay_robots,
                            obsidian + obsidian_robots,
                            geode + geode_robots,
                        ),
                        (ore_robots + 1, clay_robots, obsidian_robots, geode_robots),
                        minutes_left - 1,
                    ));
                } else {
                    let diff = ore_robot_cost.0 - ore;
                    let mut minutes = diff / ore_robots + 1;
                    if diff % ore_robots != 0 {
                        minutes += 1;
                    }
                    if minutes < minutes_left {
                        queue.push_back((
                            (
                                ore + ore_robots * minutes - ore_robot_cost.0,
                                clay + clay_robots * minutes,
                                obsidian + obsidian_robots * minutes,
                                geode + geode_robots * minutes,
                            ),
                            (ore_robots + 1, clay_robots, obsidian_robots, geode_robots),
                            minutes_left - minutes,
                        ));
                    }
                }

                if ore >= clay_robot_cost.0 {
                    queue.push_back((
                        (
                            ore + ore_robots - clay_robot_cost.0,
                            clay + clay_robots,
                            obsidian + obsidian_robots,
                            geode + geode_robots,
                        ),
                        (ore_robots, clay_robots + 1, obsidian_robots, geode_robots),
                        minutes_left - 1,
                    ));
                } else {
                    let diff = clay_robot_cost.0 - ore;
                    let mut minutes = diff / ore_robots + 1;
                    if diff % ore_robots != 0 {
                        minutes += 1;
                    }
                    if minutes < minutes_left {
                        queue.push_back((
                            (
                                ore + ore_robots * minutes - clay_robot_cost.0,
                                clay + clay_robots * minutes,
                                obsidian + obsidian_robots * minutes,
                                geode + geode_robots * minutes,
                            ),
                            (ore_robots, clay_robots + 1, obsidian_robots, geode_robots),
                            minutes_left - minutes,
                        ));
                    }
                }

                if ore >= obsidian_robot_cost.0 && clay >= obsidian_robot_cost.1 {
                    queue.push_back((
                        (
                            ore + ore_robots - obsidian_robot_cost.0,
                            clay + clay_robots - obsidian_robot_cost.1,
                            obsidian + obsidian_robots,
                            geode + geode_robots,
                        ),
                        (ore_robots, clay_robots, obsidian_robots + 1, geode_robots),
                        minutes_left - 1,
                    ));
                } else if clay_robots > 0 {
                    let ore_minutes = if ore < obsidian_robot_cost.0 {
                        let ore_diff = obsidian_robot_cost.0 - ore;
                        let mut ore_minutes = ore_diff / ore_robots;
                        if ore_diff % ore_robots != 0 {
                            ore_minutes += 1;
                        }
                        ore_minutes
                    } else {
                        1
                    };
                    let clay_minutes = if clay < obsidian_robot_cost.1 {
                        let clay_diff = obsidian_robot_cost.1 - clay;
                        let mut clay_minutes = clay_diff / clay_robots;
                        if clay_diff % clay_robots != 0 {
                            clay_minutes += 1;
                        }
                        clay_minutes
                    } else {
                        1
                    };
                    let minutes = max(ore_minutes, clay_minutes) + 1;
                    if minutes < minutes_left {
                        queue.push_back((
                            (
                                ore + ore_robots * minutes - obsidian_robot_cost.0,
                                clay + clay_robots * minutes - obsidian_robot_cost.1,
                                obsidian + obsidian_robots * minutes,
                                geode + geode_robots * minutes,
                            ),
                            (ore_robots, clay_robots, obsidian_robots + 1, geode_robots),
                            minutes_left - minutes,
                        ));
                    }
                }

                if ore >= geode_robot_cost.0 && obsidian >= geode_robot_cost.2 {
                    queue.push_back((
                        (
                            ore + ore_robots - geode_robot_cost.0,
                            clay + clay_robots,
                            obsidian + obsidian_robots - geode_robot_cost.2,
                            geode + geode_robots,
                        ),
                        (ore_robots, clay_robots, obsidian_robots, geode_robots + 1),
                        minutes_left - 1,
                    ));
                } else if obsidian_robots > 0 {
                    let ore_minutes = if ore < geode_robot_cost.0 {
                        let ore_diff = geode_robot_cost.0 - ore;
                        let mut ore_minutes = ore_diff / ore_robots;
                        if ore_diff % ore_robots != 0 {
                            ore_minutes += 1;
                        }
                        ore_minutes
                    } else {
                        1
                    };
                    let obsidian_minutes = if obsidian < geode_robot_cost.2 {
                        let obsidian_diff = geode_robot_cost.2 - obsidian;
                        let mut obsidian_minutes = obsidian_diff / obsidian_robots;
                        if obsidian_diff % obsidian_robots != 0 {
                            obsidian_minutes += 1;
                        }
                        obsidian_minutes
                    } else {
                        1
                    };
                    let minutes = max(ore_minutes, obsidian_minutes) + 1;
                    if minutes < minutes_left {
                        queue.push_back((
                            (
                                ore + ore_robots * minutes - geode_robot_cost.0,
                                clay + clay_robots * minutes,
                                obsidian + obsidian_robots * minutes - geode_robot_cost.2,
                                geode + geode_robots * minutes,
                            ),
                            (ore_robots, clay_robots, obsidian_robots, geode_robots + 1),
                            minutes_left - minutes,
                        ));
                    }
                }
            }
            println!("Max geodes: {}", max_geodes_mined);
            result += id * max_geodes_mined;
        }
        result
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 33);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 1834);
        Ok(())
    }

    fn b(
        data: Vec<(
            u16,
            (u16, u16, u16),
            (u16, u16, u16),
            (u16, u16, u16),
            (u16, u16, u16),
        )>,
    ) -> u16 {
        let data: Vec<(
            u16,
            (u16, u16, u16),
            (u16, u16, u16),
            (u16, u16, u16),
            (u16, u16, u16),
        )> = data.into_iter().take(3).collect();
        let mut results = vec![];
        for blueprint @ (
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        ) in data
        {
            println!("Processing: {:?}", blueprint);
            let mut max_geodes_mined = u16::MIN;
            let mut queue = VecDeque::from([((0, 0, 0, 0), (1, 0, 0, 0), 32)]);
            while let Some((
                (ore, clay, obsidian, geode),
                (ore_robots, clay_robots, obsidian_robots, geode_robots),
                minutes_left,
            )) = queue.pop_front()
            {
                max_geodes_mined = max(max_geodes_mined, geode + geode_robots * minutes_left);

                if minutes_left <= 1 {
                    continue;
                }
                let can_mine = geode + minutes_left * (2 * geode_robots + minutes_left - 1) / 2;

                if ore_robots >= geode_robot_cost.0 && obsidian_robots >= geode_robot_cost.2 {
                    max_geodes_mined = max(max_geodes_mined, can_mine);
                    continue;
                }

                if can_mine < max_geodes_mined {
                    continue;
                }

                if ore >= ore_robot_cost.0 {
                    queue.push_back((
                        (
                            ore + ore_robots - ore_robot_cost.0,
                            clay + clay_robots,
                            obsidian + obsidian_robots,
                            geode + geode_robots,
                        ),
                        (ore_robots + 1, clay_robots, obsidian_robots, geode_robots),
                        minutes_left - 1,
                    ));
                } else {
                    let diff = ore_robot_cost.0 - ore;
                    let mut minutes = diff / ore_robots + 1;
                    if diff % ore_robots != 0 {
                        minutes += 1;
                    }
                    if minutes < minutes_left {
                        queue.push_back((
                            (
                                ore + ore_robots * minutes - ore_robot_cost.0,
                                clay + clay_robots * minutes,
                                obsidian + obsidian_robots * minutes,
                                geode + geode_robots * minutes,
                            ),
                            (ore_robots + 1, clay_robots, obsidian_robots, geode_robots),
                            minutes_left - minutes,
                        ));
                    }
                }

                if ore >= clay_robot_cost.0 {
                    queue.push_back((
                        (
                            ore + ore_robots - clay_robot_cost.0,
                            clay + clay_robots,
                            obsidian + obsidian_robots,
                            geode + geode_robots,
                        ),
                        (ore_robots, clay_robots + 1, obsidian_robots, geode_robots),
                        minutes_left - 1,
                    ));
                } else {
                    let diff = clay_robot_cost.0 - ore;
                    let mut minutes = diff / ore_robots + 1;
                    if diff % ore_robots != 0 {
                        minutes += 1;
                    }
                    if minutes < minutes_left {
                        queue.push_back((
                            (
                                ore + ore_robots * minutes - clay_robot_cost.0,
                                clay + clay_robots * minutes,
                                obsidian + obsidian_robots * minutes,
                                geode + geode_robots * minutes,
                            ),
                            (ore_robots, clay_robots + 1, obsidian_robots, geode_robots),
                            minutes_left - minutes,
                        ));
                    }
                }

                if ore >= obsidian_robot_cost.0 && clay >= obsidian_robot_cost.1 {
                    queue.push_back((
                        (
                            ore + ore_robots - obsidian_robot_cost.0,
                            clay + clay_robots - obsidian_robot_cost.1,
                            obsidian + obsidian_robots,
                            geode + geode_robots,
                        ),
                        (ore_robots, clay_robots, obsidian_robots + 1, geode_robots),
                        minutes_left - 1,
                    ));
                } else if clay_robots > 0 {
                    let ore_minutes = if ore < obsidian_robot_cost.0 {
                        let ore_diff = obsidian_robot_cost.0 - ore;
                        let mut ore_minutes = ore_diff / ore_robots;
                        if ore_diff % ore_robots != 0 {
                            ore_minutes += 1;
                        }
                        ore_minutes
                    } else {
                        1
                    };
                    let clay_minutes = if clay < obsidian_robot_cost.1 {
                        let clay_diff = obsidian_robot_cost.1 - clay;
                        let mut clay_minutes = clay_diff / clay_robots;
                        if clay_diff % clay_robots != 0 {
                            clay_minutes += 1;
                        }
                        clay_minutes
                    } else {
                        1
                    };
                    let minutes = max(ore_minutes, clay_minutes) + 1;
                    if minutes < minutes_left {
                        queue.push_back((
                            (
                                ore + ore_robots * minutes - obsidian_robot_cost.0,
                                clay + clay_robots * minutes - obsidian_robot_cost.1,
                                obsidian + obsidian_robots * minutes,
                                geode + geode_robots * minutes,
                            ),
                            (ore_robots, clay_robots, obsidian_robots + 1, geode_robots),
                            minutes_left - minutes,
                        ));
                    }
                }

                if ore >= geode_robot_cost.0 && obsidian >= geode_robot_cost.2 {
                    queue.push_back((
                        (
                            ore + ore_robots - geode_robot_cost.0,
                            clay + clay_robots,
                            obsidian + obsidian_robots - geode_robot_cost.2,
                            geode + geode_robots,
                        ),
                        (ore_robots, clay_robots, obsidian_robots, geode_robots + 1),
                        minutes_left - 1,
                    ));
                } else if obsidian_robots > 0 {
                    let ore_minutes = if ore < geode_robot_cost.0 {
                        let ore_diff = geode_robot_cost.0 - ore;
                        let mut ore_minutes = ore_diff / ore_robots;
                        if ore_diff % ore_robots != 0 {
                            ore_minutes += 1;
                        }
                        ore_minutes
                    } else {
                        1
                    };
                    let obsidian_minutes = if obsidian < geode_robot_cost.2 {
                        let obsidian_diff = geode_robot_cost.2 - obsidian;
                        let mut obsidian_minutes = obsidian_diff / obsidian_robots;
                        if obsidian_diff % obsidian_robots != 0 {
                            obsidian_minutes += 1;
                        }
                        obsidian_minutes
                    } else {
                        1
                    };
                    let minutes = max(ore_minutes, obsidian_minutes) + 1;
                    if minutes < minutes_left {
                        queue.push_back((
                            (
                                ore + ore_robots * minutes - geode_robot_cost.0,
                                clay + clay_robots * minutes,
                                obsidian + obsidian_robots * minutes - geode_robot_cost.2,
                                geode + geode_robots * minutes,
                            ),
                            (ore_robots, clay_robots, obsidian_robots, geode_robots + 1),
                            minutes_left - minutes,
                        ));
                    }
                }
            }
            println!("Max geodes: {}", max_geodes_mined);
            results.push(max_geodes_mined);
        }
        results.into_iter().product()
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(b(data), 3472);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(b(data), 2240);
        Ok(())
    }
}
