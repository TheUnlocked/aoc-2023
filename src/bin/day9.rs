use aoc_2023::{aoc, example};
use itertools::Itertools;


aoc! {
    use "./inputs/day9.txt";

    example!(part1(
        "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"
    ) == 114);

    fn part1(input) {
        fn extrapolate_next(vals: Vec<i32>) -> i32 {
            if vals.len() == 0 {
                return 0;
            }
            let next_row = vals.windows(2).map(|a| a[1] - a[0]).collect_vec();

            vals.last().unwrap() + extrapolate_next(next_row)
        }

        input.lines()
            .map(|l| l.split(' ').map(|s| s.parse().unwrap()).collect_vec())
            .map(|n| extrapolate_next(n))
            .sum::<i32>()
    }

    example!(part2(
        "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"
    ) == 2);
    
    fn part2(input) {
        fn extrapolate_prev(vals: Vec<i32>) -> i32 {
            if vals.len() == 0 {
                return 0;
            }
            let next_row = vals.windows(2).map(|a| a[1] - a[0]).collect_vec();

            vals.first().unwrap() - extrapolate_prev(next_row)
        }

        input.lines()
            .map(|l| l.split(' ').map(|s| s.parse().unwrap()).collect_vec())
            .map(|n| extrapolate_prev(n))
            .sum::<i32>()
    }
    
}