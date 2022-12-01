use std::vec;

fn parse_file() -> Vec<String> {
    let mut input: Vec<String> = include_str!("../inputs/01.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    input.push("".into());
    input
}

fn part1(calories: &[String]) -> isize {
    let mut total_calories: Vec<isize> = vec![];
    let mut total = 0;

    for cal in calories.iter() {
        match cal.parse::<isize>() {
            Ok(c) => total += c,
            Err(_) => {
                total_calories.push(total);
                total = 0;
            }
        };
    }

    *total_calories.iter().max().unwrap()
}

fn part2(calories: &[String]) -> isize {
    let mut total_calories: Vec<isize> = vec![];
    let mut total = 0;

    for cal in calories.iter() {
        match cal.parse::<isize>() {
            Ok(c) => total += c,
            Err(_) => {
                total_calories.push(total);
                total = 0;
            }
        };
    }
    total_calories.sort_by(|a, b| b.cmp(a));
    total_calories[..3].iter().sum()
}

pub fn main() {
    let input = parse_file();
    println!("D1P1 result: {}", part1(&input));
    println!("D1P2 result: {}", part2(&input));
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse_file()), 68467);
}

#[test]
fn test_p2() {
    assert_eq!(part2(&parse_file()), 203420);
}
