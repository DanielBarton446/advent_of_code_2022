#![allow(dead_code, unused_variables)]
fn read_input(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            let ln = line.as_bytes();
            (ln[0] as char, ln[2] as char)
        })
        .collect()
}

fn main() {
    let input = &read_input("../../inputs/day_02_alt/input.txt");
}

mod p1 {
    use super::*;

    pub fn solve(input: &str) -> Vec<(char, char)> {
        read_input(input)
    }
}

mod p2 {
    pub fn solve(input: &str) -> usize {
        0
    }
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    const DAY_INPUT: &str = include_str!("../../inputs/day_02_alt/input.txt");
    const SAMPLE: &str = include_str!("../../inputs/day_02_alt/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')])
    }

    // #[test]
    // #[ignore]
    // fn p1_input() {
    //     let input = &read_input(DAY_INPUT);
    //     assert_eq!(p1::solve(input), 0)
    // }
    //
    // #[test]
    // #[ignore]
    // fn p2_sample() {
    //     assert_eq!(p2::solve(SAMPLE), 0)
    // }
    //
    // #[test]
    // #[ignore]
    // fn p2_input() {
    //     let input = &read_input(DAY_INPUT);
    //     assert_eq!(p2::solve(input), 0)
    // }
    //
}
