use std::{str::FromStr, iter, collections::{HashMap, VecDeque}};

use aoc_2023::{aoc, example};
use itertools::Itertools;

aoc! {
    use "./inputs/day12.txt";

    example!(part1(
        "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"
    ) == 21);

    fn part1(input) {
        #[derive(PartialEq, Eq)]
        enum State { Operational, Damaged, Unknown }

        struct Row {
            state: Vec<State>,
            spans: Vec<usize>,
        }

        impl Row {
            fn count_arrangements(self) -> usize {
                // Treat each specified span as a sequence of '#'s preceded by a '.'. e.g., 4 would be ".####".
                // In order to make this work, we also need to prepend a single '.' to the start of the row.

                // The total length of the row minus the number of slots in these spans (including the '.' character)
                // is now the number of '.'s that we need to insert before/after/between the spans to obtain a
                // row candidate.
                
                // Then filter out only the row candidates which work, and count how many there are.

                let states = iter::once(&State::Operational).chain(self.state.iter()).collect_vec();
                let extra_operationals = states.len() - self.spans.iter().map(|x| x + 1).sum::<usize>();
                let bins = self.spans.len() + 1;
                let stars_bars_bins = extra_operationals + bins - 1;

                (0..stars_bars_bins)
                    .combinations(bins - 1)
                    .filter(|bar_positions| {
                        // This contains the number of '.'s in each position (before/after/between spans)
                        let combination = iter::once(-1)
                            .chain(bar_positions.iter().map(|&n| n as i32))
                            .chain(iter::once(stars_bars_bins as i32))
                            .tuple_windows()
                            .map(|(a, b)| (b - a - 1) as usize);

                        // This is the final candidate
                        let candidate = combination
                            .enumerate()
                            .flat_map(|(i, n)| {
                                iter::repeat(&State::Operational).take(n + 1)
                                    .chain(iter::repeat(&State::Damaged).take({
                                        if i < self.spans.len() { self.spans[i] } else { 0 } 
                                    }))
                            });

                        // Make sure to prepend the '.' to the row, then compare against the candidate
                        iter::once(&State::Operational).chain(self.state.iter())
                            .zip(candidate)
                            .all(|p| match p {
                                (State::Operational, State::Operational) => true,
                                (State::Damaged, State::Damaged) => true,
                                (State::Unknown, _) => true,
                                _ => false,
                            })
                    })
                    .count()
            }
        }

        impl FromStr for Row {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let (states_str, spans_str) = s.split(' ').next_tuple().ok_or(())?;
                Ok(Row {
                    state: states_str.chars()
                        .map(|c| match c {
                            '.' => State::Operational,
                            '#' => State::Damaged,
                            _ => State::Unknown,
                        })
                        .collect_vec(),
                    spans: spans_str.split(',')
                        .map(|s| s.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|_| ())?,
                })
            }
        }

        input.lines()
            .map(|l| l.parse::<Row>().unwrap())
            .map(Row::count_arrangements)
            .sum::<usize>()
    }

    example!(part2(
        "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"
    ) == 525152);
    
    example!(part2(
        "???.### 1,1,3"
    ) == 1);

    example!(part2(
        ".??..??...?##. 1,1,3"
    ) == 16384);

    fn part2(input) {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum State { Operational, Damaged, Unknown }

        struct Row {
            state: Vec<State>,
            spans: Vec<usize>,
        }

        impl Row {
            fn count_arrangements(self) -> usize {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                struct Progress {
                    state_offset: usize,
                    span_idx: usize,
                }

                // Every span is treated as a sequence of n '#'s followed by a '.'.
                // In order for this to be valid, we need to append an extra '.' to the end of the state.

                let state = self.state.into_iter().chain(iter::once(State::Operational)).collect_vec();

                let mut ct = 0;
                let mut progress_counts = HashMap::<Progress, usize>::new();
                let mut progress_queue = VecDeque::<Progress>::new();

                macro_rules! enqueue {
                    ($progress:expr, $n:expr) => {
                        if let Some(ct_ref) = progress_counts.get_mut(&$progress) {
                            *ct_ref += $n;
                        }
                        else {
                            progress_counts.insert($progress, $n);
                            progress_queue.push_back($progress);
                        }
                    };
                }

                enqueue!(Progress { state_offset: 0, span_idx: 0 }, 1);

                while let Some(progress) = progress_queue.pop_front() {
                    let Progress { state_offset, span_idx } = progress;
                    let progress_count = progress_counts[&progress];

                    if span_idx == self.spans.len() {
                        if state[state_offset..].into_iter().all(|&s| s != State::Damaged) {
                            ct += progress_count;
                        }
                        continue;
                    }

                    if state_offset >= state.len() {
                        continue;
                    }

                    if state[state_offset] != State::Damaged {
                        // Effectively operational
                        enqueue!(Progress { state_offset: state_offset + 1, span_idx }, progress_count);
                    }

                    if state[state_offset] != State::Operational {
                        // Effectively damaged
                        let damaged_slice_end = state_offset + self.spans[span_idx];
                        if damaged_slice_end >= state.len() {
                            continue;
                        }

                        if state[damaged_slice_end] != State::Damaged {
                            // The space after the span could be operational
                            
                            let damaged_slice = &state[state_offset + 1..damaged_slice_end];
                            if damaged_slice.into_iter().all(|&s| s != State::Operational) {
                                // The span could all be damaged
                                enqueue!(Progress {
                                    state_offset: damaged_slice_end + 1,
                                    span_idx: span_idx + 1
                                }, progress_count);
                            }
                        }
                    }
                }

                ct
            }
        }

        impl FromStr for Row {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let (states_str, spans_str) = s.split(' ').next_tuple().ok_or(())?;
                let states_str = iter::repeat(states_str).take(5).join("?");
                let spans_str = iter::repeat(spans_str).take(5).join(",");
                Ok(Row {
                    state: states_str.chars()
                        .map(|c| match c {
                            '.' => State::Operational,
                            '#' => State::Damaged,
                            _ => State::Unknown,
                        })
                        .collect_vec(),
                    spans: spans_str.split(',')
                        .map(|s| s.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|_| ())?,
                })
            }
        }

        input.lines()
            .map(|l| l.parse::<Row>().unwrap())
            .map(Row::count_arrangements)
            .sum::<usize>()
    }
    
}