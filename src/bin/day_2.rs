#![allow(dead_code)]
const LOSE: i64 = 0;
const TIE: i64 = 3;
const WIN: i64 = 6;

fn translate_line(line: &[u8]) -> (i64, i64) {
    ((line[0]-b'A'+1) as i64, (line[2]-b'X'+1) as i64)
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input.lines()
        .map(|line| {
            translate_line(line.as_bytes())
        }).collect()
}

fn solve_p1(input: &str){
    let rounds = parse_input(input);
    let mut total: i64 = 0;
    for round in rounds{
        total += match round.1 - round.0 {
            0 => round.1 + TIE,
            -1 | 2 => round.1 + LOSE,
            -2 | 1 => round.1 + WIN,
            _ => unreachable!("not a number")
        };
    }
    dbg!(total);
}

fn solve_p2(input: &str){
    let rounds = parse_input(input);
    let mut total: i64 = 0;
    let vals = [1,2,3];
    for round in rounds{
        total += match round.1 {
            1 => vals[(round.0 as usize + 1) % 3] + LOSE,
            2 => round.0 + TIE, // good
            3 => vals[(round.0 as usize) % 3] + WIN,
            _ => unreachable!("not a number")
        }
    }
    dbg!(total);
}

fn main(){
    let input = include_str!("../../inputs/day_2.txt");
    solve_p1(input);
    solve_p2(input);
}
