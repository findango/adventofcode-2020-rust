// Advent of Code 2020 - Day 1: Report Repair
// see https://adventofcode.com/2020/day/1

#![allow(dead_code)]

use itertools::Itertools;

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

// TODO: make this generic so we can find any tuple based on the predicate
fn find_pair<P>(list: &[i32], predicate: P) -> (&i32, &i32)
where
    P: Fn(&(&i32, &i32)) -> bool,
{
    list.iter()
        .tuple_combinations()
        .filter(predicate)
        .next()
        .unwrap()
}

fn find_triplet(target: i32, list: &[i32]) -> (&i32, &i32, &i32) {
    list.iter()
        .tuple_combinations()
        .filter(|&(a, b, c)| a + b + c == target)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::utils::*;
    use std::fs::read_to_string;

    #[test]
    fn example_1() {
        let list = [1721, 979, 366, 299, 675, 1456];
        let pair = find_pair(&list, |&(a, b)| a + b == 2020);
        assert_eq!(pair, (&1721, &299));
        assert_eq!(pair.0 * pair.1, 514579);
    }

    #[test]
    fn example_2() {
        let list = [1721, 979, 366, 299, 675, 1456];
        let triplet = find_triplet(2020, &list);
        assert_eq!(triplet, (&979, &366, &675));
        assert_eq!(triplet.0 * triplet.1 * triplet.2, 241861950);
    }

    #[test]
    fn part_1() {
        let input = read_to_string("input/input01.txt").unwrap();
        let list = parse(&input);
        let pair = find_pair(&list, |&(a, b)| a + b == 2020);
        assert_eq!(pair.0 * pair.1, 1015476);
    }
    #[test]
    fn part_2() {
        let input = read_to_string("input/input01.txt").unwrap();
        let list = parse(&input);
        let triplet = find_triplet(2020, &list);
        assert_eq!(triplet.0 * triplet.1 * triplet.2, 200878544);
    }
}
