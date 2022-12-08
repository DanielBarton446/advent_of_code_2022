const ROCK: i64 = 1;
const PAPER: i64 = 2;
const SCISSORS: i64 = 3;
const LOSE: i64 = 0;
const TIE: i64 = 3;
const WIN: i64 = 6;

fn translate_line(line: &[u8]) -> (i64, i64) {
    // translate A -> 1, X -> 1
    // translate B -> 2, Y -> 2
    // translate C -> 3, Z -> 3
    ((line[0] - b'A' + 1) as i64, (line[2] - b'X' + 1) as i64)
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| translate_line(line.as_bytes()))
        .collect()
}

fn solve_p1(input: &str) {
    let rounds = parse_input(input);
    let mut total: i64 = 0;
    for round in rounds {
        let them = round.0;
        let me = round.1;
        total += match me - them {
            -1 | 2 => LOSE + me,
            0 => TIE + me,
            -2 | 1 => WIN + me,
            _ => unreachable!("not a number"),
        };
    }
    dbg!(total);
}

fn solve_p2(input: &str) {
    let rounds = parse_input(input);
    let mut total: i64 = 0;
    let state_decoding = [ROCK, PAPER, SCISSORS];
    for round in rounds {
        let them_zero_indexed = (round.0 - 1) as usize;
        let my_result = round.1;
        total += match my_result {
            1 => LOSE + state_decoding[(them_zero_indexed + 2) % 3],
            2 => TIE + state_decoding[(them_zero_indexed) % 3],
            3 => WIN + state_decoding[(them_zero_indexed + 1) % 3],
            _ => unreachable!("not a number"),
        }
    }
    dbg!(total);
}

fn main() {
    let input = include_str!("../../inputs/day_2.txt");
    solve_p1(input);
    solve_p2(input);
}
