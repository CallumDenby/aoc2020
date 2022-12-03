use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: String) {
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let alphabet: HashMap<char, i32> = format!("{}{}", alpha, alpha.to_uppercase())
        .char_indices()
        .map(|(first, second)| (second, (first + 1) as i32))
        .collect();

    part1(&input, &alphabet);
    part2(&input, &alphabet);
}

fn part1(input: &String, alphabet: &HashMap<char, i32>) {
    let total_rucksacks: i32 = input
        .trim()
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(first, second)| {
            first
                .chars()
                .filter(|c| second.contains(*c))
                .unique()
                .collect_vec()
        })
        .map(|dupes| duplicate_list_to_priority(dupes.to_vec(), alphabet))
        .sum();

    println!("P1 {:?}", total_rucksacks);
}

fn part2(input: &String, alphabet: &HashMap<char, i32>) {
    let total_rucksacks: i32 = input
        .trim()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let first: &str = chunk.next().unwrap();
            let second: &str = chunk.next().unwrap();
            let third: &str = chunk.next().unwrap();
            first
                .chars()
                .filter(|c| second.contains(*c))
                .filter(|c| third.contains(*c))
                .unique()
                .map(|c| char_to_priority(&c, &alphabet))
                .sum::<i32>()
        })
        .sum();

    println!("P2 {:?}", total_rucksacks);
}

fn duplicate_list_to_priority(list: Vec<char>, alphabet: &HashMap<char, i32>) -> i32 {
    list.iter().map(|c| char_to_priority(c, alphabet)).sum()
}

fn char_to_priority(c: &char, alphabet: &HashMap<char, i32>) -> i32 {
    alphabet.get(c).expect("").clone()
}
