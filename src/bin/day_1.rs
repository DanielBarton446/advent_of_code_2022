use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut elves: Vec<i32> = Vec::new();

    let mut elf = 0;
    for line in reader.lines() {
        if line.as_ref().unwrap() != "" {
            elf += line.as_ref().unwrap().parse::<i32>().unwrap();
        } else {
            elves.push(elf);
            elf = 0;
        }
    }

    elves.sort();
    let mut top_three = 0;
    for i in 1..4 {
        let elf = elves.pop().unwrap();
        println!("Elf {i}: {}", elf);
        top_three += elf;
    }
    println!("Total of top 3: {}", top_three);

    // Part 1 answer: use the first elf popped as the answer
    // Part 2 answer: use the total of the top 3 printed out

    Ok(())
}
