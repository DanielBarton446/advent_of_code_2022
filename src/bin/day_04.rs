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
        let starting_diff = round[0].0 - round[1].0;
        let end_diff_elf_1_later = round[0].1 - round[1].1 >= 0;
        let end_diff_elf_2_later = round[0].1 - round[1].1 <= 0;
        match starting_diff{
            0 => sum += 1,
            i if i < 0 => if end_diff_elf_1_later { sum += 1;},
            i if i > 0 => if end_diff_elf_2_later { sum += 1;}
            _ => unreachable!("NAN")
        }
    }
    sum
}
fn count_any_overlap(rounds: Vec<Vec<(i32, i32)>>) -> i32 {
    let mut sum = 0;
    for round in rounds{
        let diff = round[0].0 - round[1].0;
        let elf_1_end_overlaps_elf_2_start = round[0].1 - round[1].0 >= 0;
        let elf_2_end_overlaps_elf_1_start = round[0].0 - round[1].1 <= 0;

        match diff {
            0 => sum += 1,
            i if i < 0 => if elf_1_end_overlaps_elf_2_start { sum += 1},
            i if i > 0 => if elf_2_end_overlaps_elf_1_start { sum += 1},
            _ => unreachable!("NAN")
        }
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

