use aoc_2023::{aoc, example};
use itertools::Itertools;

aoc! {
    use "./inputs/day6.txt";

    example!(part1(
        "Time:      7  15   30
        Distance:  9  40  200"
    ) == 288);

    fn part1(input) {
        struct Race {
            time: i32,
            dist: i32,
        }

        fn count_ways_to_win(Race { time, dist }: Race) -> i32 {
            (0..time).filter(|t| t * (time - t) > dist).count() as i32
        }

        let (time_str, dist_str) = input.lines()
            .into_iter()
            .map(|s| s
                .split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<i32>().unwrap()))
            .next_tuple()
            .unwrap();

        let races = time_str.zip(dist_str).map(|(time, dist)| Race { time, dist });
        
        races.map(count_ways_to_win).product::<i32>()
    }

    example!(part2(
        "Time:      7  15   30
        Distance:  9  40  200"
    ) == 71503);
    
    fn part2(input) {
        let (time, dist) = input.lines()
            .into_iter()
            .map(|s| s.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<i64>()
                .unwrap())
            .next_tuple()
            .unwrap();

        // t_all: total time
        // t_mov: moving time
        // d: dist
        // d = (t_all - t_mov) * t_mov
        // d = (t_all * t_mov) - t_mov^2
        // t_mov^2 - (t_all * t_mov) + d = 0
        // t_mov = (t_all +- sqrt(t_all^2 - 4d)) / 2

        // high(t_mov) - low(t_mov) = total range (inclusive)
        // ceil the high and floor the low to get the true edges
        // floor (or trunc) both to get +1 in the count so the high edge is included

        let discrim = (time * time) - (4 * dist);
        let sqrt_discrim = (discrim as f64).sqrt();
        let high = (time as f64 + sqrt_discrim) / 2f64;
        let low = (time as f64 - sqrt_discrim) / 2f64;

        high as i64 - low as i64
    }
    
}