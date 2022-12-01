fn parse() -> Vec<isize> {
    include_str!("../inputs/01.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<isize>().unwrap()).sum())
        .collect()
}

fn part1(calories: &[isize]) -> isize {
    *calories.iter().max().unwrap()
}

fn part2(calories: &mut [isize]) -> isize {
    calories.sort();
    calories[calories.len() - 3..].iter().sum()
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
