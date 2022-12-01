use std::vec;

fn parse() -> Vec<isize> {
    let mut calories: Vec<String> = include_str!("../inputs/01.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    let mut total_calories = vec![];
    let mut total = 0;
    calories.push("".into());

    for cal in calories.iter() {
        match cal.parse::<isize>() {
            Ok(c) => total += c,
            Err(_) => {
                total_calories.push(total);
                total = 0;
            }
        };
    }

    total_calories
}

fn part1(calories: &[isize]) -> isize {
    *calories.iter().max().unwrap()
}

fn part2(calories: &mut [isize]) -> isize {
    calories.sort_by(|a, b| b.cmp(a));
    calories[..3].iter().sum()
}

pub fn main() {
    let mut input = parse();
    println!("D1P1 result: {}", part1(&input));
    println!("D1P2 result: {}", part2(&mut input));
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse()), 68467);
}

#[test]
fn test_p2() {
    assert_eq!(part2(&mut parse()), 203420);
}
