use aoc_2023::{aoc, example, utils::grid::Grid};

aoc! {
    use "./inputs/day3.txt";

    example!(part1(
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
    ) == 4361);

    fn part1(input) {
        fn is_symbol(ch: char) -> bool {
            !(ch.is_digit(10) || ch == '.')
        }

        let height = input.chars().filter(|c| *c == '\n').count() + 1;
        let width = input.char_indices().find(|(_, c)| *c == '\n').unwrap().0;

        let grid = Grid::<char>::from(input);
        
        // Annotate which grid cells are next to symbols 
        let mut adjacency_grid = Grid::<bool>::new_from(width, height, |_| false);
        
        for ((x, y), c) in grid.with_indices() {
            if is_symbol(*c) {
                for y_off in 0..3 {
                    for x_off in 0..3 {
                        adjacency_grid.get_mut((
                            (x + x_off).wrapping_sub(1),
                            (y + y_off).wrapping_sub(1)
                        )).map(|r| { *r = true });
                    }
                }
            }
        }

        // Find and sum numbers
        let mut total = 0;
        for (y, row) in grid.rows().enumerate() {
            let mut num = String::new();
            let mut contact_symbol = false;
            
            macro_rules! handle_non_symbol {
                ($x:expr, $then:block) => {
                    if num.len() > 0 {
                        if contact_symbol {
                            total += num.parse::<i32>().unwrap();
                        }
                        $then
                    }
                };
            }

            for (x, &ch) in row.iter().enumerate() {
                if ch.is_digit(10) {
                    num.push(ch);
                    contact_symbol |= adjacency_grid[(x, y)];
                }
                else {
                    handle_non_symbol!(x, {
                        num = String::new();
                        contact_symbol = false;
                    });
                }
            }

            handle_non_symbol!(width, {});
        }

        total
    }


    example!(part2(
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
    ) == 467835);

    fn part2(input) {
        #[derive(Clone, Copy)]
        struct GearInfo {
            neighbors: i32,
            ratio: i32,
        }

        fn is_gear(ch: char) -> bool {
            ch == '*'
        }

        let height = input.chars().filter(|c| *c == '\n').count() + 1;
        let width = input.char_indices().find(|(_, c)| *c == '\n').unwrap().0;

        let grid = Grid::<char>::from(input);
        
        // Annotate the gear values for each cell. For non-gears, we'll lock the ratio to 0
        let mut gear_grid = Grid::<GearInfo>::new_from(
            width,
            height,
            |coord| GearInfo { neighbors: 0, ratio: if is_gear(grid[coord]) { 1 } else { 0 } }
        );

        // Find gear info
        for (y, row) in grid.rows().enumerate() {
            let mut num = String::new();
            
            macro_rules! handle_non_symbol {
                ($x:expr, $then:block) => {
                    if num.len() > 0 {
                        let value = num.parse::<i32>().unwrap();
                        for y_off in 0..3 {
                            for x_neg_off in 0..num.len() + 2 {
                                gear_grid.get_mut((
                                    $x.wrapping_sub(x_neg_off),
                                    (y + y_off).wrapping_sub(1)
                                )).map(|r| {
                                    r.neighbors += 1;
                                    r.ratio *= value;
                                });
                            }
                        }
                        $then
                    }
                };
            }

            for (x, &ch) in row.iter().enumerate() {
                if ch.is_digit(10) {
                    num.push(ch);
                }
                else {
                    handle_non_symbol!(x, {
                        num = String::new();
                    });
                }
            }

            handle_non_symbol!(width, {});
        }

        gear_grid.iter()
            .filter_map(|g| if g.neighbors == 2 && g.ratio > 0 { Some(g.ratio) } else { None })
            .sum::<i32>()
    }
    
}