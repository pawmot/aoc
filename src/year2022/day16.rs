#[cfg(test)]
mod day16 {
    use core::hash::Hasher;

    use std::{
        cmp::max,
        collections::{BinaryHeap, HashMap, HashSet, VecDeque},
        hash::Hash,
        rc::Rc,
    };

    use anyhow::Result;
    use itertools::Itertools;

    use crate::data::{
        common::DatasetType::{FULL, SAMPLE},
        year2022::day16::get_data,
    };

    fn a(data: Vec<(String, u32, Vec<String>)>) -> u32 {
        let flow_map: HashMap<String, u32> =
            HashMap::from_iter(data.iter().map(|(name, flow, _)| (name.clone(), *flow)));
        let paths_map: HashMap<String, Vec<String>> =
            HashMap::from_iter(data.into_iter().map(|(name, _, paths)| (name, paths)));

        let start = &String::from("AA");
        let mut queue = VecDeque::from([(start, 30, 0, 0, HashSet::<String>::new())]);

        let mut max_flow_acc = u32::MIN;

        while let Some((current, minutes_left, flow_rate, flow_acc, state)) = queue.pop_front() {
            max_flow_acc = max(max_flow_acc, flow_acc + flow_rate * minutes_left);

            let mut visited = HashSet::new();
            let mut q = VecDeque::from([(current, 0)]);

            while let Some((pos, steps)) = q.pop_front() {
                if visited.contains(pos) {
                    continue;
                }

                visited.insert(pos);
                let pos_flow_rate = flow_map[pos];

                if !state.contains(pos) && pos_flow_rate > 0 {
                    let mut new_state = state.clone();
                    new_state.insert(pos.clone());
                    queue.push_back((
                        pos,
                        minutes_left - steps - 1,
                        flow_rate + flow_map[pos],
                        flow_acc + flow_rate * (steps + 1),
                        new_state,
                    ));
                }

                // make sure at least one minute is left for the flow to actually count, otherwise
                // cut the process short.
                if minutes_left > steps + 2 {
                    for p in &paths_map[pos] {
                        q.push_back((p, steps + 1));
                    }
                }
            }
        }

        max_flow_acc
    }

    #[test]
    fn a_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(a(data), 1651);
        Ok(())
    }

    #[test]
    fn a_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(a(data), 1754);
        Ok(())
    }

    #[derive(Hash, Clone, Copy, PartialEq, Eq)]
    struct BitSet {
        state: u64,
    }

    impl BitSet {
        fn new() -> BitSet {
            BitSet { state: 0 }
        }

        fn contains(&self, v: &usize) -> bool {
            if *v >= 64 {
                panic!("BitSet supports only 0 to 63");
            }

            self.state & (1 << v) != 0
        }

        fn insert(&mut self, v: usize) -> bool {
            if v >= 64 {
                panic!("BitSet supports only 0 to 63");
            }

            let prev_state = self.state;
            self.state |= 1 << v;
            self.state != prev_state
        }
    }

    #[derive(Eq, PartialEq, Hash, Clone, Copy)]
    struct PartialPath {
        pos1: usize,
        pos2: usize,
        flow_rate: u32,
        flow_acc: u32,
        opened: BitSet,
    }

    impl Ord for PartialPath {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.flow_acc.cmp(&other.flow_acc)
        }
    }

    impl PartialOrd for PartialPath {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    fn bb(data: Vec<(String, u32, Vec<String>)>) -> u32 {
        let names = {
            let mut names = data.iter().map(|(n, _, _)| n).cloned().collect_vec();
            names.sort();
            names
        };

        let flows = {
            let mut named_flows = data.iter().map(|(n, f, _)| (n.clone(), *f)).collect_vec();
            named_flows.sort_by_key(|(n, _)| names.binary_search(n).unwrap());
            named_flows.into_iter().map(|(_, f)| f).collect_vec()
        };

        let nonzero_valves = flows
            .iter()
            .enumerate()
            .filter(|(_, f)| **f > 0)
            .map(|(idx, _)| idx)
            .collect_vec();

        let adj = {
            let mut named_adj = data.into_iter().map(|(n, _, adj)| (n, adj)).collect_vec();
            named_adj.sort_by_key(|(n, _)| names.binary_search(n).unwrap());
            named_adj
                .into_iter()
                .map(|(_, adj)| {
                    adj.into_iter()
                        .map(|v| names.binary_search(&v).unwrap())
                        .collect_vec()
                })
                .collect_vec()
        };

        let start = 0;

        // https://www.wikiwand.com/en/Beam_search
        // 750 gives good results for my data, but found it by trial and error
        // algorithm still runs in a couple of seconds even for beam width 10000
        let beam_width = 750;
        let mut beam_heap = BinaryHeap::from([PartialPath {
            pos1: start,
            pos2: start,
            flow_rate: 0,
            flow_acc: 0,
            opened: BitSet::new(),
        }]);
        let mut max_flow_acc = 0;

        for minutes_left in (1..=26).rev() {
            let mut next_beam_heap = BinaryHeap::new();
            let mut seen = HashSet::new();
            for _ in 0..beam_width {
                if beam_heap.is_empty() {
                    break;
                }

                let path @ PartialPath {
                    pos1,
                    pos2,
                    flow_rate,
                    flow_acc,
                    opened,
                } = beam_heap.pop().unwrap();

                if seen.contains(&path) {
                    continue;
                }

                if minutes_left <= 1 {
                    max_flow_acc = max(max_flow_acc, flow_acc + flow_rate);
                    continue;
                }

                let closed_valves = nonzero_valves
                    .iter()
                    .cloned()
                    .filter(|v| !opened.contains(v))
                    .collect_vec();

                if closed_valves.is_empty() {
                    let new_path = PartialPath {
                        pos1,
                        pos2,
                        flow_rate,
                        flow_acc: flow_acc + flow_rate,
                        opened,
                    };
                    if seen.insert(new_path) {
                        next_beam_heap.push(new_path);
                    }
                    continue;
                }

                if closed_valves.contains(&pos1) && closed_valves.contains(&pos2) && pos1 != pos2 {
                    let fr1 = flows[pos1];
                    let fr2 = flows[pos2];
                    let new_opened = {
                        let mut cloned_opened = opened;
                        cloned_opened.insert(pos1);
                        cloned_opened.insert(pos2);
                        cloned_opened
                    };
                    let new_path = PartialPath {
                        pos1,
                        pos2,
                        flow_rate: flow_rate + fr1 + fr2,
                        flow_acc: flow_acc + flow_rate,
                        opened: new_opened,
                    };
                    if seen.insert(new_path) {
                        next_beam_heap.push(new_path);
                    }
                }

                if closed_valves.contains(&pos1) {
                    let fr1 = flows[pos1];
                    let new_opened = {
                        let mut cloned_opened = opened;
                        cloned_opened.insert(pos1);
                        cloned_opened
                    };
                    for v in &adj[pos2] {
                        let new_path = PartialPath {
                            pos1,
                            pos2: *v,
                            flow_rate: flow_rate + fr1,
                            flow_acc: flow_acc + flow_rate,
                            opened: new_opened,
                        };
                        if seen.insert(new_path) {
                            next_beam_heap.push(new_path);
                        }
                    }
                }

                // if pos1==pos2 then the first agent will open the valve, second will move on
                if closed_valves.contains(&pos2) && pos1 != pos2 {
                    let fr2 = flows[pos2];
                    let new_opened = {
                        let mut cloned_opened = opened;
                        cloned_opened.insert(pos2);
                        cloned_opened
                    };
                    for v in &adj[pos1] {
                        let new_path = PartialPath {
                            pos1: *v,
                            pos2,
                            flow_rate: flow_rate + fr2,
                            flow_acc: flow_acc + flow_rate,
                            opened: new_opened,
                        };
                        if seen.insert(new_path) {
                            next_beam_heap.push(new_path);
                        }
                    }
                }

                for v1 in &adj[pos1] {
                    for v2 in &adj[pos2] {
                        let new_path = PartialPath {
                            pos1: *v1,
                            pos2: *v2,
                            flow_rate,
                            flow_acc: flow_acc + flow_rate,
                            opened,
                        };
                        if seen.insert(new_path) {
                            next_beam_heap.push(new_path);
                        }
                    }
                }
            }
            beam_heap = next_beam_heap;
        }

        max_flow_acc
    }

    #[test]
    fn b_sample() -> Result<()> {
        let data = get_data(SAMPLE(None))?;

        assert_eq!(bb(data), 1707);
        Ok(())
    }

    #[test]
    fn b_full() -> Result<()> {
        let data = get_data(FULL)?;

        assert_eq!(bb(data), 2474);
        Ok(())
    }
}
