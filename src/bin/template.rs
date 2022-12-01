fn parse_file() -> Vec<String> {
    include_str!("../inputs/1.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn part1(_input: &[String]) -> isize {
    todo!();
}

fn part2(_input: &[String]) -> isize {
    todo!();
}

pub fn main() {
    let input = parse_file();
    println!("DxP1 result: {}", part1(&input));
    // println!("DxP2 result: {}", part2(&input));
}

// #[test]
// fn test_p1() {
//     assert_eq!(part1(&parse_file()), 0);
// }

// #[test]
// fn test_p2() {
//     assert_eq!(part2(&parse_file()), 0);
// }
