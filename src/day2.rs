const PUZZLE: &str = include_str!("../puzzles/day2.txt");

const RED_CUBES: u16 = 12;
const GREEN_CUBES: u16 = 13;
const BLUE_CUBES: u16 = 14;

pub fn solution() {
    let mut sum = 0;
    for line in PUZZLE.lines() {
        let mut min_green = 0;
        let mut min_red = 0;
        let mut min_blue = 0;

        if line.is_empty() {
            break;
        }

        let (game, handfuls) = line.split_once(':').unwrap();
        let id = game.split(" ").last().unwrap();
        println!("Game with id of {id}");

        for handful in handfuls.split(";") {
            for cubes in handful.split(",") {
                match cubes.trim().split_once(' ').unwrap() {
                    (count, "green") if count.parse::<u16>().unwrap() > min_green => {
                        min_green = count.parse::<u16>().unwrap();
                    }
                    (count, "red") if count.parse::<u16>().unwrap() > min_red => {
                        min_red = count.parse::<u16>().unwrap();
                    }
                    (count, "blue") if count.parse::<u16>().unwrap() > min_blue => {
                        min_blue = count.parse::<u16>().unwrap();
                    }
                    _ => {}
                }
            }
        }
        sum += min_blue * min_green * min_red;
    }
    dbg!(sum);
}
