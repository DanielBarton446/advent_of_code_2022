fn read_input(input: &str) -> Vec<Vec<(i32, i32)>> {
    input.lines()
        .map(|line| {
            line.split(',').collect::<Vec<_>>()
                .into_iter()
                .map(|elf|{
                    let mut pairs = elf.split('-');
                    (pairs.next().unwrap().parse::<i32>().unwrap(), pairs.next().unwrap().parse::<i32>().unwrap())
                }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
}

fn count_total_overlaps(rounds: Vec<Vec<(i32, i32)>>) -> i32 {
    let mut sum = 0;
    for round in rounds{
        let diff = round[0].0 - round[1].0;
        if diff == 0 { sum += 1;}
        else if diff < 0 {
            if round[0].1 - round[1].1 >= 0 { sum += 1;}
        }
        else if diff >0 && round[0].1 - round[1].1 <= 0 { sum += 1;}
    }
    sum
}
fn count_any_overlap(rounds: Vec<Vec<(i32, i32)>>) -> i32 {
    let mut sum = 0;
    for round in rounds{
        let diff = round[0].0 - round[1].0;
        if diff == 0 { sum += 1;}
        else if diff < 0 {
            if round[0].1 - round[1].0 >= 0 { sum += 1;}
        }
        else if diff > 0 && round[0].0 - round[1].1 <= 0 { sum += 1;}
    }
    sum
}


fn main() {
    let input = include_str!("../../inputs/day_04/input.txt");
    p1::solve(input);
    p2::solve(input);
}

mod p1 {
    use super::*;
    pub fn solve(input: &str) {
        let parsed = read_input(input);
        dbg!(count_total_overlaps(parsed));
    }
}

mod p2 {
    use super::*;

    pub fn solve(input: &str) {
        let parsed = read_input(input);
        dbg!(count_any_overlap(parsed));
    }
}

#[cfg(test)]
mod day04_tests {
    use super::*;
    
    const DAY_INPUT: &str = include_str!("../../inputs/day_04/input.txt");
    const SAMPLE: &str = include_str!("../../inputs/day_04/sample.txt");
    
// 2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8

    #[test]
    fn p1_sample() {
        let parsed = read_input(SAMPLE);
        let real = vec![
            vec![(2,4),(6,8)],
            vec![(2,3),(4,5)],
            vec![(5,7),(7,9)],
            vec![(2,8),(3,7)],
            vec![(6,6),(4,6)],
            vec![(2,6),(4,8)],
        ];
        assert_eq!(parsed, real);
    }

    #[test]
    fn p1_sample_violators() {
        let parsed = read_input(SAMPLE);
        assert_eq!(count_total_overlaps(parsed), 2);
    }


    #[test]
    fn p2_sample() {
        let parsed = read_input(SAMPLE);
        assert_eq!(count_any_overlap(parsed), 4);
    }

}

