const PUZZLE: &str = include_str!("../puzzles/day2.txt");

const RED_CUBES: u16 = 12;
const GREEN_CUBES: u16 = 13;
const BLUE_CUBES: u16 = 14;

pub fn solution() {
    let mut sum = 0;
    for line in PUZZLE.lines() {
        let (mut min_green, mut min_red, mut min_blue) = (0, 0, 0);

        if line.is_empty() {
            break;
        }

        let (game, handfuls) = line.split_once(':').unwrap();
        let id = game.split(" ").last().unwrap();

        for handful in handfuls.split(";") {
            for cubes in handful.split(",") {
                match cubes.trim().split_once(' ').unwrap() {
                    (count, "green") => min_green = min_green.max(count.parse().unwrap()),
                    (count, "red") => min_red = min_green.max(count.parse().unwrap()),
                    (count, "blue") => min_blue = min_green.max(count.parse().unwrap()),
                    _ => {}
                }
            }
        }
        sum += min_blue * min_green * min_red;
    }
    dbg!(sum);
}
