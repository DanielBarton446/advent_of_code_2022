use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solve_p2(input: &str) {
    let lines = parse_input(input);

    let sum = lines
        .chunks(3)
        .map(|group| {
            let set = group
                .iter()
                .map(|rucksack| rucksack.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<_>>())
                .unwrap();

            get_key_code(set.into_iter().next().unwrap())
        })
        .sum::<u32>();

    dbg!(sum);
}

fn get_key_code(c: char) -> u32 {
    let mut keycode = 0;
    if c.is_lowercase() {
        keycode += c as u32 - 'a' as u32 + 1;
    } else {
        keycode += c as u32 - 'A' as u32 + 27;
    }
    keycode
}

fn solve_p1(input: &str) {
    let lines = parse_input(input);

    let mut sum = 0;
    for line in lines {
        let mut hash = HashSet::new();
        let (rucksack1, rucksack2) = line.split_at(line.len() / 2);
        for c in rucksack1.chars() {
            hash.insert(c);
        }

        for c in rucksack2.chars() {
            if hash.contains(&c) {
                sum += get_key_code(c);
                break;
            }
        }
    }
    dbg!(sum);
}

fn main() {
    let input = include_str!("../../inputs/day_3.txt");
    solve_p1(input);
    solve_p2(input);
}
