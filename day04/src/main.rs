fn main() {
    let input = include_str!("input.txt");
    println!("part1 = {}", part1::run(input));
}

#[test]
fn part1() {
    let input = include_str!("example1.txt");
    assert_eq!(
        part1::parse(input.lines().next().unwrap()),
        part1::Card {
            winners: vec![41, 48, 83, 86, 17],
            players: vec![83, 86, 6, 31, 17, 9, 48, 53],
        }
    );
    assert_eq!(part1::run(input), 13);
}

mod part1 {
    #[derive(Debug, PartialEq)]
    pub struct Card {
        pub winners: Vec<i32>,
        pub players: Vec<i32>,
    }

    pub fn run(input: &str) -> i32 {
        input.lines().map(parse).map(score).sum()
    }

    pub fn score(card: Card) -> i32 {
        let mut score = 0;
        for &player in card.players.iter() {
            if card.winners.contains(&player) {
                score += 1;
            }
        }
        if score < 1 {
            return 0;
        }
        (2 as i32).pow(score - 1)
    }

    pub fn parse(input: &str) -> Card {
        let mut parts = input.split(':');
        let _ = parts.next().unwrap();
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
        Card { winners, players }
    }
}
