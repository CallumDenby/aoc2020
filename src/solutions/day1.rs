use rayon::prelude::*;

pub fn solve(input: String) {
    let mut elves = input
        .trim()
        .split("\n\n")
        .par_bridge()
        .map(|elf| {
            elf.par_lines()
                .map(|calorie| calorie.parse::<i32>().expect("Calorie not a number"))
                .reduce(|| 0, |acc, x| acc + x)
        })
        .collect::<Vec<i32>>();

    elves.sort_unstable();

    part1(&elves);
    part2(&elves);
}

fn part1(elves: &Vec<i32>) {
    let elf = elves.last().expect("No elves found :(");

    println!("Highest Calories for Individual: {}", elf)
}

fn part2(elves: &Vec<i32>) {
    let elf: i32 = elves.clone().iter().rev().take(3).sum();

    println!(
        "Top 3 {:?}",
        elves.clone().iter().rev().take(5).collect::<Vec<&i32>>()
    );
    println!("Total Calories for Highest 3 Elves: {}", elf)
}
