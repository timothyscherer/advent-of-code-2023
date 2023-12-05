fn main() {
    let input = include_str!("input.txt");
    println!("part1 = {}", part1::run(input).iter().sum::<i32>());
    println!("part2 = {}", part2::run(input).iter().sum::<i32>());
}

#[test]
fn part1() {
    let input = include_str!("example1.txt");
    let values = part1::run(input);
    assert_eq!(values, vec![12, 38, 15, 77]);
    assert_eq!(values.iter().sum::<i32>(), 142);
}

mod part1 {
    pub fn run(input: &str) -> Vec<i32> {
        input
            .lines()
            .map(|line| {
                let filtered = line.chars().filter(|c| c.is_numeric()).collect::<String>();
                let mut first_last = String::new();
                first_last.push(filtered.chars().next().unwrap());
                first_last.push(filtered.chars().last().unwrap());
                first_last.parse::<i32>().unwrap()
            })
            .collect::<Vec<i32>>()
    }
}

#[test]
fn part2() {
    let input = include_str!("example2.txt");
    let values = part2::run(input);
    assert_eq!(values, vec![29, 83, 13, 24, 42, 14, 76]);
    assert_eq!(values.iter().sum::<i32>(), 281);
}

mod part2 {
    pub fn run(input: &str) -> Vec<i32> {
        let mut numbers = std::collections::HashMap::new();
        numbers.insert("0", 0);
        numbers.insert("1", 1);
        numbers.insert("2", 2);
        numbers.insert("3", 3);
        numbers.insert("4", 4);
        numbers.insert("5", 5);
        numbers.insert("6", 6);
        numbers.insert("7", 7);
        numbers.insert("8", 8);
        numbers.insert("9", 9);
        numbers.insert("zero", 0);
        numbers.insert("one", 1);
        numbers.insert("two", 2);
        numbers.insert("three", 3);
        numbers.insert("four", 4);
        numbers.insert("five", 5);
        numbers.insert("six", 6);
        numbers.insert("seven", 7);
        numbers.insert("eight", 8);
        numbers.insert("nine", 9);

        input
            .lines()
            .map(|line| {
                let mut value = 0;
                let mut num = None;
                let mut ind = None;

                for (k, v) in numbers.clone() {
                    if let Some(index) = line.find(k) {
                        if ind.is_none() || index < ind.unwrap() {
                            num = Some(v);
                            ind = Some(index);
                        }
                    }
                }
                value += 10 * num.unwrap_or_else(|| 0);

                for (k, v) in numbers.clone() {
                    if let Some(index) = line.rfind(k) {
                        if ind.is_none() || index > ind.unwrap() {
                            num = Some(v);
                            ind = Some(index);
                        }
                    }
                }
                value += num.unwrap_or_else(|| 0);

                value
            })
            .collect::<Vec<i32>>()
    }
}
