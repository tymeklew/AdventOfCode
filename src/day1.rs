const PUZZLE: &str = include_str!("./day1.txt");

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solution() {
    let mut total = 0;

    for line in PUZZLE.lines() {
        dbg!(line);

        if line.is_empty() {
            break;
        }

        let bytes = line.as_bytes().iter();
        let mut vals: Vec<u32> = vec![];

        for i in 0..(bytes.len()) {
            let mut buff = String::new();
            let new = bytes.clone().skip(i).cloned().collect::<Vec<u8>>();

            if (*new.first().unwrap() as char).is_numeric() {
                vals.push((*new.first().unwrap() as char).to_string().parse().unwrap());
            }

            for b in new {
                buff.push(b as char);

                if !DIGITS.iter().any(|f| f.starts_with(&buff)) {
                    break;
                }

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

        println!("LINE : {line}");
        println!("{:?}", vals);
        println!(
            "First : {}\nLast : {}",
            vals.first().unwrap(),
            vals.last().unwrap()
        );

        total += vals.first().unwrap() * 10 + vals.last().unwrap();
    }
    println!("Total : {}", total);
}
