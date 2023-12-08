fn main() {
    let input = include_str!("input.txt");
    println!("part1 = {}", part1::run(input));
}

#[test]
fn part1() {
    let input = include_str!("example1.txt");
    // assert_eq!(part1::run(input), todo!());
}

mod part1 {
    pub fn run(input: &str) -> i32 {
        todo!();
    }
}
