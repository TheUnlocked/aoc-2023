use std::collections::HashSet;

use aoc_2023::{aoc, example};
use itertools::Itertools;
use regex::Regex;

aoc! {
    use "./inputs/day4.txt";

    example!(part1(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ) == 13);

    fn part1(input) {
        let card_re = Regex::new(r"Card\s+\d+: (.*?) \| (.*)").unwrap();
        
        fn get_numbers(nums: &str) -> impl Iterator<Item = i32> + '_ {
            nums.split_ascii_whitespace().map(|num| num.parse().unwrap())
        }
        
        input.lines().map(|card| {
            let caps = card_re.captures(card).unwrap();
            let winners = get_numbers(caps.get(1).unwrap().as_str());
            let winners_set = HashSet::<i32>::from_iter(winners);
            let ours = get_numbers(caps.get(2).unwrap().as_str());

            // Starting with 1, then we divide by 2 at the end which will floor to 0 when there are no matches
            ours.fold(1, |score, next| {
                if winners_set.contains(&next) {
                    score * 2
                }
                else {
                    score
                }
            }) / 2

        }).sum::<i32>()
    }

    example!(part2(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ) == 30);
    
    fn part2(input) {
        let card_re = Regex::new(r"Card\s+\d+: (.*?) \| (.*)").unwrap();
        
        fn get_numbers(nums: &str) -> impl Iterator<Item = i32> + '_ {
            nums.split_ascii_whitespace().map(|num| num.parse().unwrap())
        }

        struct CardInfo {
            num_wins: usize,
            num_cards: i32,
        }

        let mut card_info = input.lines().map(|card| {
            let caps = card_re.captures(card).unwrap();
            let winners = get_numbers(caps.get(1).unwrap().as_str());
            let winners_set = HashSet::<i32>::from_iter(winners);
            let ours = get_numbers(caps.get(2).unwrap().as_str());

            let num_wins = ours.fold(0, |score, next| {
                if winners_set.contains(&next) {
                    score + 1
                }
                else {
                    score
                }
            });

            CardInfo { num_wins, num_cards: 1 }

        }).collect_vec();

        for idx in 0..card_info.len() {
            let CardInfo { num_wins, num_cards } = card_info[idx];
            for add_idx in 0..num_wins {
                card_info.get_mut(idx + add_idx + 1).map(|r| {
                    r.num_cards += num_cards;
                });
            }
        }

        card_info.iter().map(|info| info.num_cards).sum::<i32>()
    }
    
}