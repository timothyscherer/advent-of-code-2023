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

#[test]
fn part2() {
    let input = include_str!("example1.txt");
    // assert_eq!(
    //     part1::has_symbol(
    //         input,
    //         part1::SearchRegion {
    //             start: part1::Point { x: 0, y: 0 },
    //             end: part1::Point { x: 3, y: 1 }
    //         }
    //     ),
    //     true
    // );
    assert_eq!(part2::run(input), 467835);
}

mod part1 {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    use super::part1::{self, Number, Point};
    use std::collections::HashMap;

    pub fn run(input: &str) -> i32 {
        get_ratios(input).iter().sum::<usize>() as i32
    }

    pub fn collect_gears(s: &str, number: Number) -> Vec<Point> {
        let mut points = vec![];
        let mut position = Point {
            x: number.location.start.x,
            y: number.location.start.y,
        };
        for line in s.lines().collect::<Vec<&str>>()
            [number.location.start.y..number.location.end.y + 1]
            .to_vec()
        {
            for c in line.chars().collect::<Vec<char>>()
                [number.location.start.x..number.location.end.x + 1]
                .to_vec()
            {
                if c == '*' {
                    points.push(position);
                }
                position.x += 1;
            }
            position.x = number.location.start.x;
            position.y += 1;
        }
        points
    }

    pub fn get_ratios(s: &str) -> Vec<usize> {
        let mut map: HashMap<Point, Vec<Number>> = HashMap::new();
        let numbers = part1::get_numbers(s);
        for number in numbers {
            let points = collect_gears(s, number);
            for point in points {
                if map.contains_key(&point) {
                    map.get_mut(&point).unwrap().push(number);
                } else {
                    map.insert(point, vec![number]);
                }
            }
        }
        let mut ratios = vec![];
        for (_, numbers) in map {
            if numbers.len() > 1 {
                let mut ratio = 1;
                for number in numbers {
                    ratio *= number.value;
                }
                ratios.push(ratio);
            }
        }
        ratios
    }
}
