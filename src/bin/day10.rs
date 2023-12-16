use std::collections::HashSet;

use aoc_2023::{aoc, example, utils::grid::Grid};
use itertools::Itertools;
use phf::phf_map;

aoc! {
    use "./inputs/day10.txt";

    example!(part1(
        ".....
        .S-7.
        .|.|.
        .L-J.
        ....."
    ) == 4);

    example!(part1(
        "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ..."
    ) == 8);

    fn part1(input) {
        let grid: Grid<char> = input.into();
        let grid = grid.grow(1, '.');

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum Dir { Left, Right, Up, Down }

        impl Dir {
            fn add(self, (x, y): (usize, usize)) -> (usize, usize) {
                match self {
                    Dir::Up => (x, y - 1),
                    Dir::Down => (x, y + 1),
                    Dir::Left => (x - 1, y),
                    Dir::Right => (x + 1, y),
                }
            }

            fn reverse(self) -> Dir {
                match self {
                    Dir::Up => Dir::Down,
                    Dir::Down => Dir::Up,
                    Dir::Left => Dir::Right,
                    Dir::Right => Dir::Left,
                }
            }
        }

        const DIR_MAP: phf::Map<char, &[Dir]> = phf_map! {
            'S' => &[Dir::Up, Dir::Down, Dir::Left, Dir::Right],
            '|' => &[Dir::Up, Dir::Down],
            '-' => &[Dir::Left, Dir::Right],
            'L' => &[Dir::Up, Dir::Right],
            'J' => &[Dir::Up, Dir::Left],
            '7' => &[Dir::Down, Dir::Left],
            'F' => &[Dir::Down, Dir::Right],
            '.' => &[],
        };

        type GraphItem = ((usize, usize), Option<Dir>);

        fn get_neighbors(grid: &Grid<char>, (pos, from_dir): GraphItem) -> Vec<GraphItem> {
            let directions = DIR_MAP.get(&grid[pos]).unwrap();

            let result = directions.iter().filter_map(|&dir| {
                if from_dir == Some(dir.reverse()) {
                    return None;
                }
                let neighbor_pos = dir.add(pos);
                let neighbor_valid = DIR_MAP.get(&grid[neighbor_pos])
                        .unwrap()
                        .iter()
                        .any(|neighbor_dir| *neighbor_dir == dir.reverse());
                match neighbor_valid {
                    true => Some((neighbor_pos, Some(dir))),
                    false => None,
                }
            }).collect_vec();

            result
        }

        let start_pos = grid.with_indices().find(|&(_, c)| *c == 'S').unwrap().0;

        get_neighbors(&grid, (start_pos, None)).iter()
            .find_map(|&loc| {
                let mut path_len = 1;
                let mut next = loc;
                while next.0 != start_pos {
                    path_len += 1;
                    match get_neighbors(&grid, next).iter().next() {
                        None => return None,
                        Some(&n) => next = n,
                    }
                }
                Some(path_len)
            })
            .unwrap() / 2
    }


    example!(part2(
        "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ..........."
    ) == 4);

    example!(part2(
        ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ..."
    ) == 8);

    example!(part2(
        "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L"
    ) == 10);
    
    fn part2(input) {
        let grid: Grid<char> = input.into();
        let expanded_grid = grid.grow(1, '.');

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum Dir { Left, Right, Up, Down }

        impl Dir {
            fn add(self, (x, y): (usize, usize)) -> (usize, usize) {
                match self {
                    Dir::Up => (x, y - 1),
                    Dir::Down => (x, y + 1),
                    Dir::Left => (x - 1, y),
                    Dir::Right => (x + 1, y),
                }
            }

            fn reverse(self) -> Dir {
                match self {
                    Dir::Up => Dir::Down,
                    Dir::Down => Dir::Up,
                    Dir::Left => Dir::Right,
                    Dir::Right => Dir::Left,
                }
            }
        }

        const DIR_MAP: phf::Map<char, &[Dir]> = phf_map! {
            'S' => &[Dir::Up, Dir::Down, Dir::Left, Dir::Right],
            '|' => &[Dir::Up, Dir::Down],
            '-' => &[Dir::Left, Dir::Right],
            'L' => &[Dir::Up, Dir::Right],
            'J' => &[Dir::Up, Dir::Left],
            '7' => &[Dir::Down, Dir::Left],
            'F' => &[Dir::Down, Dir::Right],
            '.' => &[],
        };

        type GraphItem = ((usize, usize), Option<Dir>);

        fn get_neighbors(grid: &Grid<char>, (pos, from_dir): GraphItem) -> Vec<GraphItem> {
            let directions = DIR_MAP.get(&grid[pos]).unwrap();

            let result = directions.iter().filter_map(|&dir| {
                if from_dir == Some(dir.reverse()) {
                    return None;
                }
                let neighbor_pos = dir.add(pos);
                let neighbor_valid = DIR_MAP.get(&grid[neighbor_pos])
                        .unwrap()
                        .iter()
                        .any(|neighbor_dir| *neighbor_dir == dir.reverse());
                match neighbor_valid {
                    true => Some((neighbor_pos, Some(dir))),
                    false => None,
                }
            }).collect_vec();

            result
        }

        let start_pos = expanded_grid.with_indices().find(|&(_, c)| *c == 'S').unwrap().0;

        let path = get_neighbors(&expanded_grid, (start_pos, None)).iter()
            .find_map(|&loc| {
                let mut path = HashSet::<(usize, usize)>::from_iter([start_pos]);
                let mut next = loc;
                while next.0 != start_pos {
                    path.insert(next.0);
                    match get_neighbors(&expanded_grid, next).iter().next() {
                        None => return None,
                        Some(&n) => next = n,
                    }
                }
                Some(path)
            })
            .unwrap();

        let cleaned_grid_items = grid.with_indices().map(|((x, y), &c)| {
            match path.contains(&(x + 1, y + 1)) {
                true => c,
                false => '.',
            }
        }).collect_vec();

        let cleaned_grid = Grid::new(grid.width, grid.height, cleaned_grid_items);

        let empty_spaces = cleaned_grid.iter().filter(|&&c| c == '.').count();

        // We'll subdivide the grid into 3x3 sections so we can flood fill it and get between the pipes
        const SUBDIVISION_MAP: phf::Map<char, [[char; 3]; 3]> = phf_map! {
            'S' => [['x', 'x', 'x'],
                    ['x', 'x', 'x'],
                    ['x', 'x', 'x']],

            '|' => [['.', 'x', '.'],
                    ['.', 'x', '.'],
                    ['.', 'x', '.']],

            '-' => [['.', '.', '.'],
                    ['x', 'x', 'x'],
                    ['.', '.', '.']],

            'L' => [['.', 'x', '.'],
                    ['.', 'x', 'x'],
                    ['.', '.', '.']],

            'J' => [['.', 'x', '.'],
                    ['x', 'x', '.'],
                    ['.', '.', '.']],

            '7' => [['.', '.', '.'],
                    ['x', 'x', '.'],
                    ['.', 'x', '.']],

            'F' => [['.', '.', '.'],
                    ['.', 'x', 'x'],
                    ['.', 'x', '.']],

            '.' => [[' ', ' ', ' '],
                    [' ', ' ', ' '],
                    [' ', ' ', ' ']],
        };

        let mut fillable_grid = cleaned_grid.subdivide_by(|c| *SUBDIVISION_MAP.get(&c).unwrap());
        
        fn is_emptyish(c: char) -> bool {
            c == ' ' || c == '.'
        }
        
        fn get_filled(c: char) -> char {
            match c { '.' => '`', _ => '\'' }
        }
        
        // Flood fill begins!
        // Algorithm taken from https://en.wikipedia.org/wiki/Flood_fill#Span_filling
        fn scan(lx: usize, rx: usize, y: usize, fill_stack: &mut Vec<(usize, usize)>, grid: &Grid<char>) {
            let mut span_added = false;
            for x in lx..=rx {
                if x >= grid.width || y >= grid.height || !is_emptyish(grid[(x, y)]) {
                    span_added = false;
                }
                else if !span_added {
                    fill_stack.push((x, y));
                    span_added = true;
                }
            }
        }

        let mut fill_stack = vec![(0, 0)];

        while let Some((mut x, y)) = fill_stack.pop() {
            let mut lx = x;
            while lx > 0 && is_emptyish(fillable_grid[(lx - 1, y)]) {
                fillable_grid[(lx - 1, y)] = get_filled(fillable_grid[(lx - 1, y)]);
                lx -= 1;
            }
            while x < fillable_grid.width && y < fillable_grid.height && is_emptyish(fillable_grid[(x, y)]) {
                fillable_grid[(x, y)] = get_filled(fillable_grid[(x, y)]);
                x += 1;
            }

            scan(lx, x.max(1) - 1, y + 1, &mut fill_stack, &fillable_grid);
            scan(lx, x.max(1) - 1, y.max(1) - 1, &mut fill_stack, &fillable_grid);
        }
        // Flood fill ends

        let filled_empty_spaces = fillable_grid.iter().filter(|&&c| c == '\'').count();
        
        empty_spaces - (filled_empty_spaces / 9)
    }
    
}