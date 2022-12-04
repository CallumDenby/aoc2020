use std::ops::Range;

use itertools::Itertools;

pub fn solve(input: String) {
    let elves = input
        .lines()
        .filter_map(|line| {
            line.split(",")
                .map(|p| {
                    let (start, end) = p
                        .split("-")
                        .filter_map(|x| x.parse::<i32>().ok())
                        .collect_tuple()
                        .unwrap();
                    Range { start, end }
                })
                .collect_tuple::<(Range<i32>, Range<i32>)>()
        })
        .collect_vec();

    part1(&elves);
    part2(&elves);
}

fn part1(elves: &Vec<(Range<i32>, Range<i32>)>) {
    let res = elves
        .iter()
        .filter(|(a, b)| {
            (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
        })
        .count();

    println!("Part2: {:?}", res)
}

fn part2(elves: &Vec<(Range<i32>, Range<i32>)>) {
    let res = elves
        .iter()
        .filter(|(a, b)| {
            (a.start <= b.start && b.start <= a.end) || (b.start <= a.start && a.start <= b.end)
        })
        .count();

    println!("Part2: {:?}", res)
}
