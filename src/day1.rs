const PUZZLE: &str = include_str!("../puzzles/day1.txt");

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solution() {
    let mut total = 0;

    for line in PUZZLE.lines() {
        if line.is_empty() {
            break;
        }

        let bytes = line.as_bytes().iter();
        let mut vals: Vec<u32> = vec![];

        for i in 0..(bytes.len()) {
            let mut buff = String::new();
            let new = bytes.clone().skip(i).cloned().collect::<Vec<u8>>();

            // Found just a regular number
            if (*new.first().unwrap() as char).is_numeric() {
                vals.push((*new.first().unwrap() as char).to_string().parse().unwrap());
                continue;
            }

            for b in new {
                buff.push(b as char);

                // Check if the buffer is a digit
                if !DIGITS.iter().any(|f| f.starts_with(&buff)) {
                    break;
                }

                // Add the digit to the vector
                if let Some(digit) = DIGITS.iter().find(|f| f.to_string() == buff) {
                    println!("Found digit : {}", digit);
                    match digit.as_ref() {
                        "one" => vals.push(1),
                        "two" => vals.push(2),
                        "three" => vals.push(3),
                        "four" => vals.push(4),
                        "five" => vals.push(5),
                        "six" => vals.push(6),
                        "seven" => vals.push(7),
                        "eight" => vals.push(8),
                        "nine" => vals.push(9),
                        _ => (),
                    }
                }
            }
        }

        total += vals.first().unwrap() * 10 + vals.last().unwrap();
    }
    println!("Total : {}", total);
}
