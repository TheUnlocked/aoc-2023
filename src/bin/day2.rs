use aoc_2023::{aoc, example};
use regex::Regex;

aoc! {
    use "./inputs/day2.txt";

    example!(part1(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    ) == 8);

    fn part1(input) {
        const MAX_RED: i32 = 12;
        const MAX_GREEN: i32 = 13;
        const MAX_BLUE: i32 = 14;

        let lines = input.split("\n");
        
        let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

        lines.enumerate().map(|(i, game)| {
            if re.captures_iter(game).any(|caps| {
                let amt = caps[1].parse::<i32>().unwrap();
                match caps[2].chars().nth(0).unwrap() {
                    'r' => amt > MAX_RED,
                    'g' => amt > MAX_GREEN,
                    'b' => amt > MAX_BLUE,
                    _ => panic!("Unexpected color: {}", &caps[2]),
                }
            }) {
                0 // has invalid draw
            }
            else {
                i as i32 + 1 // 1-indexed
            }
        }).sum::<i32>()
    }

    example!(part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green") == 48);
    example!(part2("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue") == 12);
    example!(part2("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red") == 1560);

    example!(part2(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    ) == 2286);


    fn part2(input) {
        struct Cubes { red: i32, green: i32, blue: i32 }

        let lines = input.split("\n");
        
        let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

        lines.map(|game| {
            let mut cubes = Cubes { red: 0, green: 0, blue: 0 };
            
            re.captures_iter(game).for_each(|caps| {
                let amt = caps[1].parse::<i32>().unwrap();
                match caps[2].chars().nth(0).unwrap() {
                    'r' => if cubes.red < amt { cubes.red = amt },
                    'g' => if cubes.green < amt { cubes.green = amt },
                    'b' => if cubes.blue < amt { cubes.blue = amt },
                    _ => panic!("Unexpected color: {}", &caps[2]),
                };
            });
            
            return cubes.red * cubes.green * cubes.blue;
        }).sum::<i32>()
    }
    
}