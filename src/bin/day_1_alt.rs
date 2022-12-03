

fn to_iter(input: &str) -> impl Iterator<Item = usize> + '_ // important to consume lifetime
{
    input.split("\n\n").map(|elf|{
        elf.split_whitespace()
        .map(|cals| cals.parse::<usize>().unwrap())
        .sum()
    })
}

fn solve_p1(input: &str) -> usize {
    to_iter(input).max().expect("Empty input!")
}

fn solve_p2(input: &str) -> usize {
    let mut list = to_iter(input).collect::<Vec<usize>>();
    list.sort();
    list.pop().unwrap() + list.pop().unwrap() + list.pop().unwrap()

}

fn main()
{

    let input = include_str!("../../inputs/day_1.txt");
    let val = solve_p1(input);
    println!("{}", val);
    let val = solve_p2(input);
    println!("{}", val);


}

