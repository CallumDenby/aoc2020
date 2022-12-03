use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: String) {
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let alphabet: HashMap<char, i32> = format!("{}{}", alpha, alpha.to_uppercase())
        .char_indices()
        .map(|(first, second)| (second, (first + 1) as i32))
        .collect();

    let prepared_input = input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    part1(&prepared_input, &alphabet);
    part2(&prepared_input, &alphabet);
}

fn part1(input: &Vec<Vec<char>>, alphabet: &HashMap<char, i32>) {
    let total_rucksacks: i32 = input
        .iter()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(first, second)| {
            first
                .iter()
                .filter(|c| second.contains(*c))
                .unique()
                .map(|c| char_to_priority(&c, alphabet))
                .sum::<i32>()
        })
        .sum();

    println!("P1 {:?}", total_rucksacks);
}

fn part2(input: &Vec<Vec<char>>, alphabet: &HashMap<char, i32>) {
    let total_rucksacks: i32 = input
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let first: &Vec<char> = chunk.take_first().unwrap();
            let second: &Vec<char> = chunk.take_first().unwrap();
            let third: &Vec<char> = chunk.take_first().unwrap();
            first
                .iter()
                .filter(|c| second.contains(*c) && third.contains(*c))
                .unique()
                .map(|c| char_to_priority(&c, alphabet))
                .sum::<i32>()
        })
        .sum();

    println!("P2 {:?}", total_rucksacks);
}

fn char_to_priority(c: &char, alphabet: &HashMap<char, i32>) -> i32 {
    alphabet
        .get(c)
        .expect("Unable to convert key to priority")
        .clone()
}
