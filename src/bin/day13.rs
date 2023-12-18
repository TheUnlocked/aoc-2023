use aoc_2023::{aoc, example, utils::grid::Grid};

aoc! {
    use "./inputs/day13.txt";

    example!(part1(
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
    ) == 405);
    
    example!(part1(
        "..##..##.....#...
        .##.#..###..#..#.
        ###...###.#...##.
        #..#.###...#.###.
        #..#.###...#.###.
        ###...###.#..###.
        .##.#..###..#..#.
        ..##..##.....#...
        #..#..#.#.#.#.##.
        #.#..##.##..#.#.#
        ##....####.####..
        ##.###.#.#.#####.
        ##.###.#.#.#####."
    ) == 1200);

    fn part1(input) {
        enum Reflection {
            Vertical(usize),
            Horizontal(usize),
        }

        fn find_horizontal_reflection(grid: &Grid<char>) -> Option<usize> {
            (1..grid.height)
                .filter(|&refl_y| {
                    let lower_range = (0..refl_y).rev();
                    let upper_range = refl_y..grid.height;
                    upper_range.zip(lower_range)
                        .all(|(y1, y2)| {
                            (0..grid.width).all(|x| grid[(x, y1)] == grid[(x, y2)])
                        })
                })
                .next()
        }

        fn find_reflection(grid: Grid<char>) -> Option<Reflection> {
            if let Some(y) = find_horizontal_reflection(&grid) {
                Some(Reflection::Horizontal(y))
            }
            else if let Some(x) = find_horizontal_reflection(&grid.transpose()) {
                Some(Reflection::Vertical(x))
            }
            else {
                None
            }
        }

        input.split("\n\n")
            .map(Grid::from)
            .map(find_reflection)
            .map(|r| match r.unwrap() {
                Reflection::Vertical(x) => x,
                Reflection::Horizontal(y) => 100 * y,
            })
            .sum::<usize>()
    }

    example!(part2(
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
    ) == 400);

    fn part2(input) {
        enum Reflection {
            Vertical(usize),
            Horizontal(usize),
        }

        fn find_horizontal_reflection(grid: &Grid<char>) -> Option<usize> {
            (1..grid.height)
                .filter(|&refl_y| {
                    let lower_range = (0..refl_y).rev();
                    let upper_range = refl_y..grid.height;
                    let mut seen_defect = false;
                    for (y1, y2) in upper_range.zip(lower_range) {
                        for x in 0..grid.width {
                            if grid[(x, y1)] != grid[(x, y2)] {
                                if seen_defect {
                                    return false;
                                }
                                else {
                                    seen_defect = true;
                                }
                            }
                        }
                    }
                    seen_defect
                })
                .next()
        }

        fn find_reflection(grid: Grid<char>) -> Option<Reflection> {
            if let Some(y) = find_horizontal_reflection(&grid) {
                Some(Reflection::Horizontal(y))
            }
            else if let Some(x) = find_horizontal_reflection(&grid.transpose()) {
                Some(Reflection::Vertical(x))
            }
            else {
                None
            }
        }

        input.split("\n\n")
            .map(Grid::from)
            .map(find_reflection)
            .map(|r| match r.unwrap() {
                Reflection::Vertical(x) => x,
                Reflection::Horizontal(y) => 100 * y,
            })
            .sum::<usize>()
    }
    
}