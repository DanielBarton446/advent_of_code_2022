fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    input
        .split_once("\n\n")
        .map(|group| {
            // group.0 ==> stacks
            // group.1 ==> instructions
            (
                group.0.lines().collect::<Vec<&str>>(),
                group.1.lines().collect::<Vec<&str>>(),
            )
        })
        .unwrap()
}

fn parse_instructions(instructions: Vec<&str>) -> Vec<[usize; 3]> {
    instructions
        .into_iter()
        .map(|line| {
            let splits = line.split_whitespace().collect::<Vec<_>>();
            [
                splits[1].parse::<usize>().unwrap(),
                splits[3].parse::<usize>().unwrap(), 
                splits[5].parse::<usize>().unwrap(),
            ] 
        })
        .collect::<Vec<[usize; 3]>>()
}

fn create_stacks(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut last_idx = lines[0].len() - 1;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    while last_idx > 0 {
        // check if the number which tells us there is a stack exists
        if lines[lines.len() - 1]
            .chars()
            .nth(last_idx)
            .unwrap()
            .is_numeric()
        {
            stacks.push(Vec::new());
        }
        for line in lines.iter().rev() {
            let c = line.chars().nth(last_idx).unwrap();
            if c.is_alphabetic() {
                let stack_idx = stacks.len() - 1;
                stacks[stack_idx].push(c);
            }
        }
        last_idx -= 1;
    }
    stacks.reverse();
    stacks
}

fn main() {
    let input: &str = include_str!("../../inputs/day_05/input.txt");
    p1::solve(input);
    p2::solve(input);
}

mod p1 {
    use super::*;

    pub fn solve(input: &str) {
        let parsed = parse_input(input);
        let mut stacks = create_stacks(parsed.0);
        let instructions = parse_instructions(parsed.1);

        for action in instructions {
            let repetitions = action[0];
            let target_stack = action[1] - 1; // zero indexed
            let destination_stack = action[2] - 1; // zero indexed
            for _rep in 0..repetitions {
                let val = stacks[target_stack].pop().unwrap();
                stacks[destination_stack].push(val);
            }
        }

        let top_vals = stacks
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect::<String>();
        dbg!(top_vals);
    }
}

mod p2 {
    use super::*;

    pub fn solve(input: &str) {
        let parsed = parse_input(input);
        let mut stacks = create_stacks(parsed.0);
        let instructions = parse_instructions(parsed.1);

        for action in instructions {
            let repetitions = action[0];
            let target_stack = action[1] - 1; // zero indexed
            let destination_stack = action[2] - 1; // zero indexed
            let remove_index = stacks[target_stack].len() - repetitions;
            for _rep in 0..repetitions {
                let val = stacks[target_stack].remove(remove_index);
                stacks[destination_stack].push(val);
            }
        }

        let top_vals = stacks
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect::<String>();
        dbg!(top_vals);
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    // const DAY_INPUT: &str = include_str!("../../inputs/DAY_INPUT");
    const SAMPLE: &str = include_str!("../../inputs/day_05/sample.txt");

    #[test]
    fn p1_sample() {
        let input = parse_input(SAMPLE);
        let stacks = create_stacks(input.0);
        let expected_stack = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let instructions = parse_instructions(input.1);
        let expected_instructions = vec![[1, 2, 1], [3, 1, 3], [2, 2, 1], [1, 1, 2]];
        assert_eq!(stacks, expected_stack);
        assert_eq!(instructions, expected_instructions);
    }
}
