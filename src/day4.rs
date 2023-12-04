use std::collections::HashMap;

const PUZZLE: &str = include_str!("../puzzles/day4.txt");

#[derive(Debug, Clone, Copy)]
struct Card {
    id: u16,
    matches: u8,
}

struct Solver {
    cards: Vec<Card>,
    scratchcards: HashMap<u16, Vec<u16>>, // Mapping of id to amount
}
impl Solver {
    fn new(cards: Vec<Card>) -> Self {
        let mut scratchcards = HashMap::new();

        for card in &cards {
            *scratchcards.entry(card.id).or_insert(Vec::new()) = cards
                .iter()
                .skip(card.id.into())
                .take(card.matches.into())
                .map(|f| f.id)
                .collect::<Vec<u16>>();
        }
        dbg!(scratchcards.clone());

        Self {
            scratchcards,
            cards,
        }
    }
    fn solve(&mut self, id: u16) -> u32 {
        // For one card
        let mut sum = 1;
        let ids = &self.scratchcards[&id].clone();
        for id in ids {
            sum += self.solve(*id);
        }

        return sum;
    }
}

pub fn solution() {
    //    let mut sum = 0;
    //  let mut points = 0;

    let mut cards: Vec<Card> = vec![];

    for line in PUZZLE.lines() {
        let (left, right) = line.split_once("|").unwrap();

        let (card, winning) = left.split_once(":").unwrap();

        let id = card
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u16>()
            .unwrap();

        let mut card = Card { id, matches: 0 };

        let win = winning
            .trim()
            .split_whitespace()
            .filter_map(|f| f.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let yours = right
            .trim()
            .split_whitespace()
            .filter_map(|f| f.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        for num in yours {
            if let Some(_) = win.iter().find(|f| **f == num) {
                card.matches += 1;

                /* Part 1
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }*/
            }
        }

        cards.push(card);
    }

    let mut solver = Solver::new(cards.clone());

    let mut sum = 0;
    for card in cards {
        sum += solver.solve(card.id);
    }
    dbg!(sum);
}
