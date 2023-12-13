fn main() {
    let input = include_str!("input.txt");
    println!("part1 = {}", part1::run(input));
    println!("part2 = {}", part2::run(input));
}

#[test]
fn part1() {
    let input = include_str!("example1.txt");
    assert_eq!(part1::run(input), 35);
}

#[test]
fn part2() {
    let input = include_str!("example1.txt");
    assert_eq!(part2::run(input), 46);
}

mod part1 {
    use std::str::Lines;

    #[derive(Debug, Clone)]
    pub struct MapEntry {
        pub destination: i64,
        pub source: i64,
        pub length: i64,
    }

    impl MapEntry {
        pub fn new(destination: i64, source: i64, length: i64) -> Self {
            Self {
                destination,
                source,
                length,
            }
        }

        pub fn get(&self, source: i64) -> Option<i64> {
            if (self.source..self.source + self.length).contains(&source) {
                Some(self.destination + source - self.source)
            } else {
                None
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Map {
        pub name: String,
        pub entries: Vec<MapEntry>,
    }

    impl Map {
        pub fn new(name: String) -> Self {
            Self {
                name,
                entries: vec![],
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Almanac {
        pub seeds: Vec<i64>,
        pub maps: Vec<Map>,
    }

    impl Almanac {
        pub fn new() -> Self {
            Self {
                seeds: vec![],
                maps: vec![],
            }
        }
    }

    pub fn parse(input: &str) -> Almanac {
        let mut almanac = Almanac::new();

        let mut lines = input.lines();
        almanac.seeds = lines
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty() && *s != "seeds:")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        fn parse_map(lines: &mut Lines) -> Option<Map> {
            let mut started = false;
            let mut map = Map::new("".to_string());
            for line in lines {
                if !started && line.contains("map") {
                    map = Map::new(line.split(' ').next().unwrap().to_string());
                    started = true;
                } else if started && line.is_empty() {
                    return Some(map);
                } else if started {
                    let mut parts = line.split(' ').filter(|s| !s.is_empty());
                    map.entries.push(MapEntry::new(
                        parts.next().unwrap().parse::<i64>().unwrap(),
                        parts.next().unwrap().parse::<i64>().unwrap(),
                        parts.next().unwrap().parse::<i64>().unwrap(),
                    ));
                }
            }

            if started {
                Some(map)
            } else {
                None
            }
        }

        while let Some(map) = parse_map(&mut lines) {
            almanac.maps.push(map);
        }

        almanac
    }

    pub fn run(input: &str) -> i64 {
        let mut almanac = parse(input);
        // println!("{:?}", almanac.seeds);
        for map in almanac.maps {
            for seed in almanac.seeds.iter_mut() {
                for entry in map.entries.iter() {
                    if let Some(new) = entry.get(*seed) {
                        // println!("{} -> {} ({:?})", *seed, new, entry);
                        *seed = new;
                        break;
                    }
                }
            }
            // println!("{:?} ({})", almanac.seeds, map.name);
        }

        *almanac.seeds.iter().min().unwrap()
    }
}

mod part2 {
    use std::str::Lines;

    use crate::part1::{Almanac, Map, MapEntry};

    pub fn parse(input: &str) -> Almanac {
        let mut almanac = Almanac::new();

        let mut lines = input.lines();
        let seed_values = lines
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty() && *s != "seeds:")
            .map(|s| s.parse::<i64>().unwrap());

        let mut start = 0;
        println!("getting seeds");
        for (i, value) in seed_values.enumerate() {
            if i % 2 == 0 {
                start = value;
            } else {
                almanac.seeds.extend(start..start + value);
            }
        }
        println!("{:?}", almanac.seeds);

        fn parse_map(lines: &mut Lines) -> Option<Map> {
            let mut started = false;
            let mut map = Map::new("".to_string());
            for line in lines {
                if !started && line.contains("map") {
                    map = Map::new(line.split(' ').next().unwrap().to_string());
                    started = true;
                } else if started && line.is_empty() {
                    return Some(map);
                } else if started {
                    let mut parts = line.split(' ').filter(|s| !s.is_empty());
                    map.entries.push(MapEntry::new(
                        parts.next().unwrap().parse::<i64>().unwrap(),
                        parts.next().unwrap().parse::<i64>().unwrap(),
                        parts.next().unwrap().parse::<i64>().unwrap(),
                    ));
                }
            }

            if started {
                Some(map)
            } else {
                None
            }
        }

        while let Some(map) = parse_map(&mut lines) {
            almanac.maps.push(map);
        }

        almanac
    }

    pub fn run(input: &str) -> i64 {
        let mut almanac = parse(input);
        // println!("{:?}", almanac.seeds);
        for map in almanac.maps {
            for seed in almanac.seeds.iter_mut() {
                for entry in map.entries.iter() {
                    if let Some(new) = entry.get(*seed) {
                        // println!("{} -> {} ({:?})", *seed, new, entry);
                        *seed = new;
                        break;
                    }
                }
            }
            // println!("{:?} ({})", almanac.seeds, map.name);
        }

        *almanac.seeds.iter().min().unwrap()
    }
}
