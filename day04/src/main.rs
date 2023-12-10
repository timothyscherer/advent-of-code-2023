fn main() {
    let input = include_str!("input.txt");
    println!("part1 = {}", part1::run(input));
    println!("part2 = {}", part2::run(input));
}

#[test]
fn part1() {
    let input = include_str!("example1.txt");
    assert_eq!(
        part1::parse(input.lines().next().unwrap()),
        part1::Card {
            id: 1,
            winners: vec![41, 48, 83, 86, 17],
            players: vec![83, 86, 6, 31, 17, 9, 48, 53],
        }
    );
    assert_eq!(part1::run(input), 13);
}

#[test]
fn part2() {
    let input = include_str!("example1.txt");
    assert_eq!(part2::run(input), 30);
}

mod part1 {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Card {
        pub id: usize,
        pub winners: Vec<i32>,
        pub players: Vec<i32>,
    }

    pub fn run(input: &str) -> i32 {
        input.lines().map(parse).map(|card| score(&card)).sum()
    }

    pub fn count(card: &Card) -> i32 {
        let mut count = 0;
        for &player in card.players.iter() {
            if card.winners.contains(&player) {
                count += 1;
            }
        }
        count
    }

    pub fn score(card: &Card) -> i32 {
        let score = count(card);
        if score < 1 {
            0
        } else {
            (2 as i32).pow(score as u32 - 1)
        }
    }

    pub fn parse(input: &str) -> Card {
        let mut parts = input.split(':');
        let id = parts
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty() && *s != "Card")
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut raw = parts.next().unwrap().split('|');
        let winners = raw
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        let players = raw
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        Card {
            id,
            winners,
            players,
        }
    }
}

mod part2 {
    use crate::part1::{self, Card};

    pub fn run(input: &str) -> i32 {
        let mut winners = input
            .lines()
            .map(part1::parse)
            .collect::<Vec<Card>>()
            .iter()
            .map(|card| part1::count(&card))
            .collect::<Vec<i32>>();

        for i in (0..winners.len()).rev() {
            for j in 1..=winners[i] as usize {
                winners[i] += winners[i + j];
            }
        }

        winners.iter().sum::<i32>() + winners.len() as i32
    }
}
