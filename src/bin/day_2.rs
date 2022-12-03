use std::collections::HashMap;



fn solve_round(player_1: &str, player_2: &str) -> usize {
    let states = ["Rock", "Paper", "Scissors"];
    let mut iter = states.iter().cycle();

    for x in iter.by_ref() {
        if *x == player_1 {
            break;
        }
    }

    (iter.take_while(|x| **x != player_2).count() + 1) % 3
    
}

fn translate_char(c: char) -> &'static str {
    match c {
        'A' | 'X' => "Rock",
        'B' | 'Y' => "Paper",
        'C' | 'Z' => "Scissors",
        _ => unreachable!("Malformed input")
    }
}
fn translate_line(line: &str) -> (&str, &str){
    // translate A -> Rock, X -> Rock
    // translate B -> Paper, Y -> Paper
    // translate C -> Scissors, Z -> Scissors
    let first = line.as_bytes()[0] as char;
    let second = line.as_bytes()[2] as char;

    (translate_char(first), translate_char(second))
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input.lines()
        .map(|line| {
            translate_line(line)
        }).collect::<Vec<_>>()
}

fn solve_p1(input: &str, select_points: &HashMap<&str, i32>, round_points: &HashMap<usize, i32>) {
    let rounds = parse_input(input);
    let mut sum = 0;
    for round in rounds {
        sum += select_points.get(&round.1).unwrap();
        let result = solve_round(round.1, round.0);
        sum += round_points.get(&result).unwrap();
    }

    dbg!(sum);
}

fn solve_p2(input: &str, select_points: &HashMap<&str, i32>, round_points: &HashMap<usize, i32>) {
    let states = ["Rock", "Paper", "Scissors"];
    let rounds = parse_input(input);

    let mut total = 0;
    for round in rounds {
        let val = match round.1 {
            "Rock" => -1,
            "Paper" => 0,
            "Scissors" => 1,
            _ => unreachable!("unexpected value")
        };
        let losing_state = states[(states.iter().position(|&x| x == round.0).unwrap() + 2 ) % 3];
        let winning_state = states[(states.iter().position(|&x| x == round.0).unwrap() + 1 ) % 3];
        let score = match val {
            -1 => select_points.get(losing_state).unwrap() + 0,
            0 => select_points.get(round.0).unwrap() + 3,
            1 => select_points.get(winning_state).unwrap() + 6,
            _ => unreachable!("dun goofed")
        };
        total += score;
    }
    dbg!(total);


}

fn main(){

    let select_points: HashMap<&str, i32> = HashMap::from([("Rock", 1), ("Paper", 2), ("Scissors", 3)]);
    let round_points: HashMap<usize, i32> = HashMap::from([(0, 3), (1, 0), (2, 6)]);

    let input = include_str!("../../inputs/day_2.txt");
    solve_p1(input, &select_points, &round_points);
    solve_p2(input, &select_points, &round_points);
}
