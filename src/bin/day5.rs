use std::convert::identity;

use aoc_2023::{aoc, example};
use itertools::Itertools;

aoc! {
    use "./inputs/day5.txt";

    example!(part1(
        "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"
    ) == 35);

    fn part1(input) {
        let [seeds_str, map_strs @ ..] = &input.split("\n\n").collect_vec()[..] else { panic!() };

        let seeds = seeds_str.split(" ").skip(1).map(|s| s.parse::<u64>().unwrap());

        struct Rule {
            from: u64,
            to: u64,
            dest: u64,
        }
        struct Map(Vec<Rule>);

        fn run_map(num: u64, Map(rules): &Map) -> u64 {
            for rule in rules {
                if num >= rule.from && num < rule.to {
                    return num - rule.from + rule.dest;
                }
            }
            num
        }

        let maps = map_strs.iter().map(|map_str| Map({
            map_str.lines().skip(1).map(|rule_str| {
                if let [dest, from, len] = rule_str
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .take(3)
                    .collect_vec()[..]
                {
                    Rule { from, to: from + len, dest }
                }
                else {
                    panic!()
                }
            }).collect_vec()
        })).collect_vec();

        seeds.map(|seed| {
            maps.iter().fold(seed, run_map)
        }).min().unwrap()
    }

    example!(part2(
        "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"
    ) == 46);
    
    fn part2(input) {
        let [seeds_str, map_strs @ ..] = &input.split("\n\n").collect_vec()[..] else { panic!() };

        /// Represents the bounds of an left-closed/right-open interval
        #[derive(PartialEq, Eq)]
        struct Range(u64, u64);

        let seed_ranges = seeds_str
            .split(" ")
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap())
            .chunks(2)
            .into_iter()
            .map(|pair| {
                let (from, len) = pair.collect_tuple().unwrap();
                Range(from, from + len)
            })
            .collect_vec();

        struct Rule {
            from: u64,
            to: u64,
            dest: u64,
        }
        struct Map(Vec<Rule>);

        fn get_intersection(&Range(from, to): &Range, rule: &Rule) -> Option<Range> {
            if from > rule.to || to < rule.from {
                // Disjoint
                return None;
            }
            else if from < rule.from {
                if to < rule.to {
                    // Overlapping on left side
                    return Some(Range(rule.from, to));
                }
                else {
                    // Fully contains rule
                    return Some(Range(rule.from, rule.to));
                }
            }
            else {
                if rule.to < to {
                    // Overlapping on the right side
                    return Some(Range(from, rule.to));
                }
                else {
                    // Fully contained by rule
                    return Some(Range(from, to));
                }
            }
        }

        fn apply_map_to_range<'a>(range: &'a Range, Map(rules): &'a Map) -> Vec<Range> {
            let intersections = rules.iter()
                .map(|rule| get_intersection(range, rule).map(|i| (rule, i)))
                .filter_map(identity)
                // Need to sort so we can find the non-intersecting ranges from sequential gaps
                .sorted_by(|a, b| Ord::cmp(&a.1.0, &b.1.0));


            //   [-------------------------)   range
            //       [----)         [----)     intersections
            //   [---)    [---------)    [-)   gaps

            let mut start = range.0;
            let end = range.1;
            let mut mapped_ranges = Vec::new();

            for (rule, intersection) in intersections {
                // Add previous gap
                mapped_ranges.push(Range(start, intersection.0));
                // Add intersection (adjusted by mapping)
                mapped_ranges.push(Range(
                    intersection.0 + rule.dest - rule.from,
                    intersection.1 + rule.dest - rule.from
                ));
                start = intersection.1;
            }
            // Add final gap
            mapped_ranges.push(Range(start, end));

            mapped_ranges
        }

        fn run_map(ranges: Vec<Range>, map: Map) -> Vec<Range> {
            ranges.iter().flat_map(|range| apply_map_to_range(range, &map))
                .filter(|range| range.0 != range.1) // Filter out 0-length ranges
                .collect_vec()
        }

        let maps = map_strs.iter().map(|map_str| Map({
            map_str.lines().skip(1).map(|rule_str| {
                if let [dest, from, len] = rule_str
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .take(3)
                    .collect_vec()[..]
                {
                    Rule { from, to: from + len, dest }
                }
                else {
                    panic!()
                }
            }).collect_vec()
        }));

        maps.fold(seed_ranges, run_map)
            .iter()
            .map(|range| range.0)
            .min()
            .unwrap()
    }
    
}