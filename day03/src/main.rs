fn main() {
    let input = include_str!("input.txt");
    println!("part1 = {}", part1::run(input));
    println!("part2 = {}", part2::run(input));
}

#[test]
fn part1() {
    let input = include_str!("example1.txt");
    assert_eq!(
        part1::has_symbol(
            input,
            part1::SearchRegion {
                start: part1::Point { x: 0, y: 0 },
                end: part1::Point { x: 3, y: 1 }
            }
        ),
        true
    );
    assert_eq!(part1::run(input), 4361);
}

mod part1 {
    #[derive(Debug, Clone, Copy)]
    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct SearchRegion {
        pub start: Point,
        pub end: Point,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Number {
        pub value: usize,
        pub location: SearchRegion,
    }

    pub fn run(input: &str) -> i32 {
        // println!("{:?}", get_numbers(input));
        reduce(input, get_numbers(input)).iter().sum::<usize>() as i32
    }

    pub fn is_symbol(c: char) -> bool {
        !c.is_numeric() && c != '.'
    }

    pub fn has_symbol(s: &str, location: SearchRegion) -> bool {
        for line in s.lines().collect::<Vec<&str>>()[location.start.y..location.end.y + 1].to_vec()
        {
            for c in
                line.chars().collect::<Vec<char>>()[location.start.x..location.end.x + 1].to_vec()
            {
                if is_symbol(c) {
                    return true;
                }
            }
        }
        false
    }

    pub fn reduce(s: &str, numbers: Vec<Number>) -> Vec<usize> {
        let mut reduced = vec![];
        for n in numbers {
            if has_symbol(s, n.location) {
                reduced.push(n.value);
            }
        }
        reduced
    }

    pub fn get_numbers(s: &str) -> Vec<Number> {
        let mut numbers = vec![];
        let mut continued = false;

        let mut current_number = String::new();
        let mut location = SearchRegion {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 0 },
        };

        for line in s.lines().enumerate() {
            for c in line.1.chars().enumerate() {
                if c.1.is_numeric() {
                    current_number.push(c.1);
                    if !continued {
                        location.start.x = if c.0 == 0 { c.0 } else { c.0 - 1 };
                        location.start.y = if line.0 == 0 { line.0 } else { line.0 - 1 };
                        location.end.y = if line.0 == s.lines().count() - 1 {
                            line.0
                        } else {
                            line.0 + 1
                        };
                    }
                    continued = true;
                }
                if !c.1.is_numeric() || c.0 == line.1.chars().count() - 1 {
                    if continued {
                        location.end.x = if c.0 == line.1.chars().count() - 1 {
                            c.0
                        } else {
                            c.0
                        };
                        numbers.push(Number {
                            value: current_number.parse::<usize>().unwrap(),
                            location: location,
                        });
                        current_number.clear();
                    }
                    continued = false;
                }
            }
            continued = false;
        }
        numbers
    }
}

mod part2 {
    pub fn run(input: &str) -> i32 {
        todo!();
    }
}
