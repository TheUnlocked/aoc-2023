use std::collections::HashMap;

use aoc_2023::{aoc, example};
use itertools::Itertools;
use phf::phf_map;
use regex::Regex;

aoc! {
    use "./inputs/day8.txt";

    example!(part1(
        "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"
    ) == 2);

    example!(part1(
        "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
    ) == 6);

    fn part1(input) {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        struct Location(u32);

        impl Location {
            fn new(s: &str) -> Location {
                Location(s.chars().take(3).enumerate().map(|(i, c)| (c as u32) << (i * 8)).sum())
            }

            const AAA: Location = Location((('A' as u32) << 16) + (('A' as u32) << 8) + ('A' as u32));
            const ZZZ: Location = Location((('Z' as u32) << 16) + (('Z' as u32) << 8) + ('Z' as u32));
        }

        #[derive(Clone, Copy)]
        enum Choice { Left, Right }
        const PATH_MAP: phf::Map<char, Choice> = phf_map! { 'L' => Choice::Left, 'R' => Choice::Right };

        trait Directional<T> {
            fn in_direction(&self, choice: Choice) -> T;
        }

        impl Directional<Location> for (Location, Location) {
            fn in_direction(&self, choice: Choice) -> Location {
                match choice {
                    Choice::Left => self.0,
                    Choice::Right => self.1,
                }
            }
        }

        let mut map = HashMap::<Location, (Location, Location)>::new();

        let mut lines_iter = input.lines();
        let route_str = lines_iter.next().unwrap();

        let line_re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

        for line in lines_iter.skip(1) {
            let (at, left, right) = line_re.captures(line).unwrap()
                .iter()
                .skip(1)
                .map(|cap| Location::new(cap.unwrap().as_str()))
                .next_tuple()
                .unwrap();
            map.insert(at, (left, right));
        }

        let route = route_str.chars()
            .map(|c| PATH_MAP.get(&c).unwrap())
            .cycle();

        let mut pos = Location::AAA;
        let mut steps = 0;

        for dir in route {
            steps += 1;
            pos = map.get(&pos).unwrap().in_direction(*dir);
            if pos == Location::ZZZ {
                return steps;
            }
        }

        panic!()
    }

    example!(part2(
        "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"
    ) == 6);
    
    fn part2(input) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct Location(u32);

        impl Location {
            fn new(s: &str) -> Location {
                Location(s.chars().take(3).enumerate().map(|(i, c)| (c as u32) << ((2 - i) * 8)).sum())
            }

            fn is_start(&self) -> bool {
                self.0 & 0xFF == ('A' as u32)
            }

            fn is_end(&self) -> bool {
                self.0 & 0xFF == ('Z' as u32)
            }
        }

        #[derive(Clone, Copy)]
        enum Choice { Left, Right }
        const PATH_MAP: phf::Map<char, Choice> = phf_map! { 'L' => Choice::Left, 'R' => Choice::Right };

        trait Directional<T> {
            fn in_direction(&self, choice: Choice) -> T;
        }

        impl Directional<Location> for (Location, Location) {
            fn in_direction(&self, choice: Choice) -> Location {
                match choice {
                    Choice::Left => self.0,
                    Choice::Right => self.1,
                }
            }
        }

        let mut map = HashMap::<Location, (Location, Location)>::new();
        let mut locations = Vec::<Location>::new();

        let mut lines_iter = input.lines();
        let route_str = lines_iter.next().unwrap();

        let line_re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();

        for line in lines_iter.skip(1) {
            let (at, left, right) = line_re.captures(line).unwrap()
                .iter()
                .skip(1)
                .map(|cap| Location::new(cap.unwrap().as_str()))
                .next_tuple()
                .unwrap();
            locations.push(at);
            map.insert(at, (left, right));
        }

        let route = route_str.chars()
            .map(|c| PATH_MAP.get(&c).unwrap())
            .enumerate()
            .collect_vec();

        // While this doesn't handle the case where they all line up before one path cycles,
        // the inputs are likely designed so that doesn't happen (since the problem would be trivial if so).
        // Commented out is code to support a scenario where there are multiple exits as part of the cycle,
        // though in practice the problem input was not designed that way.
        #[derive(Debug)]
        struct CycleInfo {
            steps_until_exit_in_cycle: u64,
            // exit_spacing: Vec<u64>,
        }

        let starting_locs = locations.iter().filter(|l| l.is_start()).map(|l| *l).collect_vec();
        let mut cycles = Vec::<CycleInfo>::new();

        for loc in starting_locs {
            let mut steps = 0;
            let mut current_loc = loc;
            let mut visited = HashMap::<(Location, usize), u64>::new();
            let mut found_exits = Vec::<(Location, usize)>::new();

            for &(steps_into_route, dir) in route.iter().cycle() {
                if let Some(steps_until_cycle) = visited.get(&(current_loc, steps_into_route)) {
                    let found_exits_in_cycle = found_exits.iter()
                        .filter(|ex| visited.get(ex).unwrap() >= steps_until_cycle)
                        .collect_vec();
                    
                    let steps_until_exit_in_cycle = *visited.get(&found_exits_in_cycle[0]).unwrap();
                    
                    // let mut exit_spacing = Vec::<u64>::new();
                    // let mut steps_since_last_exit = steps_until_exit_in_cycle;
                    // for exit in found_exits_in_cycle.iter().skip(1) {
                    //     let &steps_to_exit = visited.get(exit).unwrap();
                    //     exit_spacing.push(steps_to_exit - steps_since_last_exit);
                    //     steps_since_last_exit = steps_to_exit;
                    // }
                    // exit_spacing.push(
                    //     (steps - steps_since_last_exit) + (steps_until_exit_in_cycle - steps_until_cycle)
                    // );

                    cycles.push(CycleInfo { steps_until_exit_in_cycle, /* exit_spacing */ });
                    break;
                }

                visited.insert((current_loc, steps_into_route), steps);
                if current_loc.is_end() {
                    found_exits.push((current_loc, steps_into_route));
                }

                steps += 1;
                current_loc = map.get(&current_loc).unwrap().in_direction(*dir);
            }
        }

        // Because my input was designed such that there is only one cycle spacing
        // which is the same as the number of steps until the first cycle,
        // this last part can simply be implemented with LCM. I assume all inputs are
        // structured that way, but if not, this may not work for some.
        
        fn gcd(a: u64, b: u64) -> u64 {
            if b == 0 {
                a
            }
            else {
                gcd(b, a % b)
            }
        }

        fn lcm(mut nums: impl Iterator<Item = u64>) -> Option<u64> {
            if let Some(first) = nums.next() {
                let second = lcm(nums)?;
                Some((first * second) / gcd(first, second))
            }
            else {
                Some(1)
            }
        }

        lcm(cycles.iter().map(|c| c.steps_until_exit_in_cycle)).unwrap()
    }
    
}