use std::{collections::HashSet, iter, cmp::Ordering};

use aoc_2023::{aoc, example, utils::grid::Grid};
use itertools::Itertools;

aoc! {
    use "./inputs/day11.txt";

    example!(part1(
        "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."
    ) == 374);

    fn part1(input) {
        fn manhattan((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
            x1.abs_diff(x2) + y1.abs_diff(y2)
        }

        let grid: Grid<char> = input.into();

        let empty_cols_iter = grid.columns()
            .enumerate()
            .filter_map(|(x, mut col)| match col.all(|&c| c == '.') { true => Some(x), _ => None });

        let empty_cols = HashSet::<usize>::from_iter(empty_cols_iter);

        let empty_rows = grid.rows()
            .enumerate()
            .filter_map(|(y, row)| match row.iter().all(|&c| c == '.') { true => Some(y), _ => None })
            .collect_vec();

        let expanded_vec = grid.with_indices().flat_map(|((x, y), &c)| {
            let mut fill_n = 1;
            if c == '.' {
                if empty_cols.contains(&x) {
                    fill_n += 1;
                }
                if x == 0 && empty_rows.contains(&y) {
                    fill_n += grid.width + empty_cols.len();
                }
            }
            iter::repeat(c).take(fill_n)
        }).collect_vec();

        let expanded_grid = Grid::new(grid.width + empty_cols.len(), grid.height + empty_rows.len(), expanded_vec);

        expanded_grid.with_indices()
            .filter_map(|(pos, c)| match c { '#' => Some(pos), _ => None })
            .combinations(2)
            .fold(0, |total, pts| total + manhattan(pts[0], pts[1]))
    }

    // example!(part2(
    //     "...#......
    //     .......#..
    //     #.........
    //     ..........
    //     ......#...
    //     .#........
    //     .........#
    //     ..........
    //     .......#..
    //     #...#....."
    // ) == 1030);

    fn part2(input) {
        const EXPAND_BY: usize = 1_000_000 - 1;

        fn sort_pair<T : Ord>(a: T, b: T) -> (T, T) {
            match a.cmp(&b) {
                Ordering::Less => (a, b),
                _ => (b, a),
            }
        }

        let grid: Grid<char> = input.into();

        let empty_cols = grid.columns()
            .enumerate()
            .filter_map(|(x, mut col)| match col.all(|&c| c == '.') { true => Some(x), _ => None })
            .collect_vec();

        let empty_rows = grid.rows()
            .enumerate()
            .filter_map(|(y, row)| match row.iter().all(|&c| c == '.') { true => Some(y), _ => None })
            .collect_vec();
    
        let get_distance = |(x1, y1): (usize, usize), (x2, y2): (usize, usize)| -> usize {
            let (x1, x2) = sort_pair(x1, x2);
            let (y1, y2) = sort_pair(y1, y2);
            let num_cols_between = empty_cols.iter().filter(|&&x| x > x1 && x < x2).count();
            let num_rows_between = empty_rows.iter().filter(|&&y| y > y1 && y < y2).count();

            x1.abs_diff(x2) + y1.abs_diff(y2) + (num_cols_between + num_rows_between) * EXPAND_BY
        };

        grid.with_indices()
            .filter_map(|(pos, c)| match c { '#' => Some(pos), _ => None })
            .combinations(2)
            .fold(0, |total, pts| total + get_distance(pts[0], pts[1]))
    }
    
}