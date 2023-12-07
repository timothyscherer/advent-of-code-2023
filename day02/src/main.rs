fn main() {
    let input = include_str!("input.txt");
    println!(
        "part1 = {}",
        part1::run(
            input,
            Colors {
                red: 12,
                green: 13,
                blue: 14
            }
        )
    );
    println!("part2 = {}", part2::run(input));
}

#[test]
fn part1() {
    let input = include_str!("example1.txt");
    assert_eq!(
        part1::run(
            input,
            Colors {
                red: 12,
                green: 13,
                blue: 14,
            },
        ),
        8
    );
}

#[test]
fn part2() {
    let input = include_str!("example1.txt");
    assert_eq!(part2::run(input), 2286);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colors {
    red: i32,
    green: i32,
    blue: i32,
}

impl Colors {
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    id: i32,
    rounds: Vec<Colors>,
}

impl Game {
    pub fn new(id: i32) -> Self {
        Self { id, rounds: vec![] }
    }
}

pub fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let line = line.replace("Game ", "");
            let mut parts = line.split(":");

            let mut game = Game::new(parts.next().unwrap().parse::<i32>().unwrap());
            let rounds = parts.next().unwrap().split(";");

            for round in rounds {
                let samples = round.split(",");
                for sample in samples {
                    let contents = sample.split(" ");
                    let mut contents = contents.filter(|s| !s.is_empty());

                    let count = contents.next().unwrap().parse::<i32>().unwrap();
                    let color = contents.next().unwrap();
                    let mut colors = Colors::new();
                    match color {
                        "red" => {
                            colors.red = count;
                        }
                        "green" => {
                            colors.green = count;
                        }
                        "blue" => {
                            colors.blue = count;
                        }
                        _ => {
                            panic!("Invalid color string");
                        }
                    }
                    game.rounds.push(colors);
                }
            }
            game
        })
        .collect()
}

impl Game {
    fn validate(&self, contents: Colors) -> bool {
        for round in &self.rounds {
            if round.red > contents.red
                || round.green > contents.green
                || round.blue > contents.blue
            {
                return false;
            }
        }
        true
    }

    fn minimum(&self) -> Colors {
        let mut result = Colors::new();
        for round in &self.rounds {
            if round.red > result.red {
                result.red = round.red;
            }
            if round.green > result.green {
                result.green = round.green;
            }
            if round.blue > result.blue {
                result.blue = round.blue;
            }
        }
        result
    }
}

mod part1 {
    use crate::Colors;

    pub fn run(input: &str, contents: Colors) -> i32 {
        let mut result = 0;
        let records = crate::parse(input)
            .into_iter()
            .filter(|game| game.validate(contents));
        records.for_each(|game| {
            result += game.id;
        });
        result
    }
}

mod part2 {
    pub fn run(input: &str) -> i32 {
        let powers = crate::parse(input).into_iter().map(|game| {
            let min = game.minimum();
            min.red * min.green * min.blue
        });
        powers.sum()
    }
}
