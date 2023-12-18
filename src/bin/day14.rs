use std::collections::HashMap;

use aoc_2023::{aoc, example, utils::grid::Grid};
use itertools::Itertools;

aoc! {
    use "./inputs/day14.txt";

    example!(part1(
        "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."
    ) == 136);

    fn part1(input) {
        let grid = Grid::from(input);

        // transpose for cache efficiency (probably)
        let transposed = grid.transpose();

        struct RockStack {
            start_row: usize,
            num_rocks: usize,
        }

        transposed.rows()
            .flat_map(|row| {
                let mut stacks = Vec::<RockStack>::new();
                let mut rocks_in_stack = 0;
                for (row, c) in row.iter().rev().enumerate() {
                    match c {
                        'O' => rocks_in_stack += 1,
                        '#' => {
                            stacks.push(RockStack { start_row: row, num_rocks: rocks_in_stack });
                            rocks_in_stack = 0;
                        },
                        _ => {}
                    }
                }
                stacks.push(RockStack { start_row: row.len(), num_rocks: rocks_in_stack });
                stacks
            })
            .map(|RockStack { start_row, num_rocks }| (start_row - num_rocks + 1..=start_row).sum::<usize>())
            .sum::<usize>()
    }

    example!(part2(
        "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."
    ) == 64);

    fn part2(input) {
        let mut grid = Grid::from(input);

        fn rot_neg_90(grid: Grid<char>) -> Grid<char> { 
            grid.flip_horizontal().transpose()
        }

        fn fall_left(grid: Grid<char>) -> Grid<char> {
            let init = grid.rows().flat_map(|row| {
                let mut row = Vec::from(row);
                let mut swap_to_idx = 0;
                for i in 0..row.len() {
                    match row[i] {
                        '#' => {
                            swap_to_idx = i + 1;
                        }
                        'O' => {
                            row.swap(i, swap_to_idx);
                            swap_to_idx += 1;
                        }
                        _ => {}
                    }
                }
                row
            }).collect_vec();

            Grid::new(grid.width, grid.height, init)
        }

        fn cycle(grid: Grid<char>) -> Grid<char> {
            let mut grid = grid;
            for _ in 0..4 {
                // rotate first because falling left is a bit easier than falling up.
                // rotate negative because falling left is a bit easier than falling right.
                grid = rot_neg_90(grid);
                grid = fall_left(grid);
            }
            grid
        }

        let mut seen_states = Vec::<String>::new();
        let mut seen_states_map = HashMap::<String, usize>::new();

        // Need to flip horizontally so we go N->W->S->E rather than N->E->S->W
        // This is because we rotate CCW instead of CW since the sliding logic
        // is a bit simpler to the left than to the right or up.
        // Flipping horizontally also doesn't impact the total load.
        grid = grid.flip_horizontal();

        for i in 0.. {
            grid = cycle(grid);
            let as_string = grid.to_string();
            if let Some(seen_idx) = seen_states_map.get(&as_string) {
                // Once we hit a cycle, stop
                let loop_len = i - seen_idx;
                let loop_offset = (1_000_000_000 - seen_idx - 1) % loop_len;
                grid = seen_states[seen_idx + loop_offset][..].into();
                break;
            }
            seen_states.push(as_string.clone());
            seen_states_map.insert(as_string, i);
        }

        grid.with_indices()
            .map(|((_, y), c)| {
                match c {
                    'O' => grid.height - y,
                    _ => 0,
                }
            })
            .sum::<usize>()
    }
    
}