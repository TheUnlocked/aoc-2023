use aho_corasick::AhoCorasick;
use aoc_2023::{aoc, example};

aoc! {
    use "./inputs/day1.txt";

    example!(
        part1(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"
        ) == 142
    );

    fn part1(input) {
        let lines = input.split("\n");
        
        lines.map(|line| {
            let digits = line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>();

            return [digits.first().unwrap(), digits.last().unwrap()]
                .into_iter()
                .collect::<String>()
                .parse()
                .unwrap_or(0)
        }).sum::<i32>()
    }

    example!(
        part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"
        ) == 281
    );


    example!(
        part2(
            "eightwo"
        ) == 82
    );

    fn part2(input) {

        let fsm_forwards = AhoCorasick::builder()
            .build(&[
                "_", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                "_", "1", "2", "3", "4", "5", "6", "7", "8", "9",
            ])
            .unwrap();

        let fsm_backwards = AhoCorasick::builder()
            .build(&[
                "_", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
                "_", "1", "2", "3", "4", "5", "6", "7", "8", "9",
            ])
            .unwrap();

        input.split("\n").map(|line| {
            let first = fsm_forwards.find(line).unwrap().pattern().as_i32() % 10;

            let backwards_line = line.chars().rev().collect::<String>();
            let last = fsm_backwards.find(&backwards_line).unwrap().pattern().as_i32() % 10;

            return first * 10 + last
        }).sum::<i32>()
    }
    
}