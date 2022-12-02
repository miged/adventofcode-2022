fn parse() -> Vec<String> {
    include_str!("../inputs/02.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn part1(input: &[String]) -> isize {
    let mut score = 0;
    for pair in input {
        let pair: Vec<&str> = pair.split(' ').collect();
        score += match pair[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        } + match (pair[0], pair[1]) {
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            _ => 0,
        };
    }
    score
}

fn part2(input: &[String]) -> isize {
    let mut score = 0;
    for pair in input {
        let pair: Vec<&str> = pair.split(' ').collect();
        score += match pair[1] {
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        } + match (pair[1], pair[0]) {
            ("X", "B") | ("Y", "A") | ("Z", "C") => 1,
            ("X", "C") | ("Y", "B") | ("Z", "A") => 2,
            ("X", "A") | ("Y", "C") | ("Z", "B") => 3,
            _ => 0,
        }
    }
    score
}

pub fn main() {
    let input = parse();
    println!("D2P1 result: {}", part1(&input));
    println!("D2P2 result: {}", part2(&input));
}

#[test]
fn test_p1() {
    assert_eq!(part1(&parse()), 13682);
}

#[test]
fn test_p2() {
    assert_eq!(part2(&parse()), 12881);
}
