use std::collections::HashSet;
fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn find_unique_window(signal: &str, window_size: usize) -> isize {
    let signal = signal.chars().collect::<Vec<char>>();

    let mut idx = window_size as isize; // first window is of size 4
    for window in signal.windows(window_size) {
        let set: HashSet<_> = HashSet::from_iter(window);
        if set.len() == window_size {
            return idx;
        }
        idx += 1;
    }
    -1
}

fn main() {
    let input = include_str!("../../inputs/day_06/input.txt");
    p1::solve(input);
    p2::solve(input);
}

mod p1 {

    use super::*;

    pub fn solve(input: &str) -> Vec<isize> {
        let parsed: Vec<&str> = parse_input(input);
        let mut answers: Vec<isize> = Vec::new();
        for line in parsed {
            let start: isize = find_unique_window(line, 4);
            answers.push(start);
            dbg!(start);
        }
        answers
    }
}

mod p2 {

    use super::*;

    pub fn solve(input: &str) -> Vec<isize> {
        let parsed: Vec<&str> = parse_input(input);
        let mut answers: Vec<isize> = Vec::new();
        for line in parsed {
            let start: isize = find_unique_window(line, 14);
            answers.push(start);
            dbg!(start);
        }
        answers
    }
}

#[cfg(test)]
mod day04_tests {

    use super::*;

    // const DAY_INPUT: &str = include_str!("../../inputs/DAY_INPUT");
    const SAMPLE: &str = include_str!("../../inputs/day_06/sample.txt");

    #[test]
    fn p1_sample() {
        let expected = vec![5, 6, 10, 11];
        assert_eq!(p1::solve(SAMPLE), expected);
    }

    #[test]
    fn p2_sample() {
        let expected = vec![19, 23, 23, 29, 26];
        assert_eq!(p2::solve(SAMPLE), expected);
    }
}
