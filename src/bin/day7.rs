use std::{cmp::Ordering, array};

use aoc_2023::{aoc, example};
use itertools::Itertools;
use phf::phf_map;

aoc! {
    use "./inputs/day7.txt";

    example!(part1(
        "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"
    ) == 6440);

    fn part1(input) {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        enum HandType {
            HighCard,
            OnePair,
            TwoPair,
            ThreeOfAKind,
            FullHouse,
            FourOfAKind,
            FiveOfAKind,
        }

        struct Hand {
            bid: i32,
            cards: [u8; 5],
        }

        fn get_hand_type(cards: [u8; 5]) -> HandType {
            let mut counts: [u8; 13] = [0; 13];

            for card in cards {
                counts[card as usize - 2] += 1;
            }

            let (most_freq, next_most_freq) = counts.iter()
                .enumerate()
                .sorted_by(|a, b| b.1.cmp(a.1))
                .next_tuple()
                .unwrap();

            match most_freq.1 {
                1 => HandType::HighCard,
                2 => if *next_most_freq.1 == 2 { HandType::TwoPair } else { HandType::OnePair },
                3 => if *next_most_freq.1 == 2 { HandType::FullHouse } else { HandType::ThreeOfAKind },
                4 => HandType::FourOfAKind,
                _ => HandType::FiveOfAKind,
            }
        }

        fn compare_hands(&Hand { cards: cards1, .. }: &Hand, &Hand { cards: cards2, .. }: &Hand) -> Ordering {
            match get_hand_type(cards1).cmp(&get_hand_type(cards2)) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => {
                    for i in 0..5 {
                        match cards1[i].cmp(&cards2[i]) {
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => {}
                        }
                    }
                    Ordering::Equal
                }
            }
        }

        const CARD_VALUES: phf::Map<char, u8> = phf_map! {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
        };

        input.lines()
            .map(|hand_str| {
                let (cards_str, bid_str) = hand_str.split(' ').next_tuple().unwrap();
                let mut cards_iter = cards_str.chars().map(|c| CARD_VALUES.get(&c).unwrap());
                let cards = array::from_fn(|_| *cards_iter.next().unwrap());
                let bid = bid_str.parse().unwrap();
                Hand { cards, bid }
            })
            .sorted_by(compare_hands)
            .enumerate()
            .map(|(i, hand)| (i as i32 + 1) * hand.bid)
            .sum::<i32>()

    }

    example!(part2(
        "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"
    ) == 5905);
    
    fn part2(input) {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        enum HandType {
            HighCard,
            OnePair,
            TwoPair,
            ThreeOfAKind,
            FullHouse,
            FourOfAKind,
            FiveOfAKind,
        }

        struct Hand {
            bid: i32,
            cards: [u8; 5],
        }

        fn get_hand_type(cards: [u8; 5]) -> HandType {
            let mut counts: [u8; 12] = [0; 12];
            let mut jokers = 0;

            for card in cards {
                if card == 0 {
                    jokers += 1;
                }
                else {
                    counts[card as usize - 2] += 1;
                }
            }

            let (most_freq, next_most_freq) = counts.iter()
                .enumerate()
                .sorted_by(|a, b| b.1.cmp(a.1))
                .next_tuple()
                .unwrap();

            match most_freq.1 + jokers {
                1 => HandType::HighCard,
                2 => if *next_most_freq.1 == 2 { HandType::TwoPair } else { HandType::OnePair },
                3 => if *next_most_freq.1 == 2 { HandType::FullHouse } else { HandType::ThreeOfAKind },
                4 => HandType::FourOfAKind,
                _ => HandType::FiveOfAKind,
            }
        }

        fn compare_hands(&Hand { cards: cards1, .. }: &Hand, &Hand { cards: cards2, .. }: &Hand) -> Ordering {
            match get_hand_type(cards1).cmp(&get_hand_type(cards2)) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => {
                    for i in 0..5 {
                        match cards1[i].cmp(&cards2[i]) {
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => {}
                        }
                    }
                    Ordering::Equal
                }
            }
        }

        const CARD_VALUES: phf::Map<char, u8> = phf_map! {
            'J' => 0,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
        };

        input.lines()
            .map(|hand_str| {
                let (cards_str, bid_str) = hand_str.split(' ').next_tuple().unwrap();
                let mut cards_iter = cards_str.chars().map(|c| CARD_VALUES.get(&c).unwrap());
                let cards = array::from_fn(|_| *cards_iter.next().unwrap());
                let bid = bid_str.parse().unwrap();
                Hand { cards, bid }
            })
            .sorted_by(compare_hands)
            .enumerate()
            .map(|(i, hand)| (i as i32 + 1) * hand.bid)
            .sum::<i32>()
    }
    
}