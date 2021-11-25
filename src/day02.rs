#![allow(dead_code)]

// use log::*;
use regex::Regex;

#[derive(Debug)]
struct Password {
    a: usize,
    b: usize,
    letter: char,
    pwd: String,
}

fn parse(input: &str) -> Vec<Password> {
    let matcher = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    input
        .lines()
        .map(|line: &str| {
            let m = matcher.captures(&line).unwrap();
            Password {
                a: m[1].parse().unwrap(),
                b: m[2].parse().unwrap(),
                letter: m[3].parse().unwrap(),
                pwd: m[4].parse().unwrap(),
            }
        })
        .collect()
}

fn nth(s: &String, n: usize) -> char {
    s.chars().nth(n).unwrap()
}

fn validate(p: &&Password) -> bool {
    let count = p.pwd.matches(p.letter).count().try_into().unwrap();
    p.a <= count && count <= p.b
}

fn validate2(p: &&Password) -> bool {
    let ca = nth(&p.pwd, p.a - 1);
    let cb = nth(&p.pwd, p.b - 1);
    (ca == p.letter || cb == p.letter) && ca != cb
}

const EXAMPLE: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn example_1() {
        let pwds = parse(EXAMPLE);
        let goods = pwds.iter().filter(validate);
        assert_eq!(2, goods.count());
    }

    #[test]
    fn example_2() {
        let pwds = parse(EXAMPLE);
        let goods = pwds.iter().filter(validate2);
        assert_eq!(1, goods.count());
    }

    #[test]
    fn part_1() {
        let input = read_to_string("input/input02.txt").unwrap();
        let pwds = parse(&input);
        let goods = pwds.iter().filter(validate);
        assert_eq!(536, goods.count());
    }

    #[test]
    fn part_2() {
        let input = read_to_string("input/input02.txt").unwrap();
        let pwds = parse(&input);
        let goods = pwds.iter().filter(validate2);
        assert_eq!(558, goods.count());
    }
}
