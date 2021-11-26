// Advent of Code 2020 - Day 1: Report Repair
// see https://adventofcode.com/2020/day/1

#![allow(dead_code)]

use itertools::Itertools;
use std::fs::read_to_string;

fn load_input(filename: &str) -> Vec<i32> {
    let input = read_to_string(filename).unwrap();
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn find_combo<P>(length: usize, list: &[i32], predicate: P) -> Vec<&i32>
where
    P: Fn(&Vec<&i32>) -> bool,
{
    list.iter()
        .combinations(length)
        .filter(predicate)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let list = [1721, 979, 366, 299, 675, 1456];
        let pair = find_combo(2, &list, |xs| xs[0] + xs[1] == 2020);
        assert_eq!(pair[0] * pair[1], 514579);
    }

    #[test]
    fn example_2() {
        let list = [1721, 979, 366, 299, 675, 1456];
        let triplet = find_combo(3, &list, |xs| xs[0] + xs[1] + xs[2] == 2020);
        assert_eq!(triplet[0] * triplet[1] * triplet[2], 241861950);
    }

    #[test]
    fn part_1() {
        let list = load_input("input/input01.txt");
        let pair = find_combo(2, &list, |xs| xs[0] + xs[1] == 2020);
        assert_eq!(pair[0] * pair[1], 1015476);
    }

    #[test]
    fn part_2() {
        let list = load_input("input/input01.txt");
        let triplet = find_combo(3, &list, |xs| xs[0] + xs[1] + xs[2] == 2020);
        assert_eq!(triplet[0] * triplet[1] * triplet[2], 200878544);
    }
}
