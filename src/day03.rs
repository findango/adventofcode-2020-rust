#![allow(dead_code)]

use std::fs::read_to_string;

const EXAMPLE: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

type Map = Vec<Vec<char>>;

fn parse(input: &str) -> Map {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn load_input(filename: &str) -> Map {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

fn count_trees(map: &Map, dx: usize, dy: usize) -> usize {
    let width = map[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < map.len() {
        if map[y][x % width] == '#' {
            count += 1;
        }
        x += dx;
        y += dy;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let map = parse(EXAMPLE);
        let total = count_trees(&map, 3, 1);
        assert_eq!(7, total);
    }

    #[test]
    fn example_2() {
        let map = parse(EXAMPLE);
        let trees = [
            count_trees(&map, 1, 1),
            count_trees(&map, 3, 1),
            count_trees(&map, 5, 1),
            count_trees(&map, 7, 1),
            count_trees(&map, 1, 2),
        ];
        let total = trees.iter().fold(1, |a, b| a * b);
        assert_eq!(336, total);
    }

    #[test]
    fn part_1() {
        let map = load_input("input/input03.txt");
        let total = count_trees(&map, 3, 1);
        assert_eq!(274, total);
    }

    #[test]
    fn part_2() {
        let map = load_input("input/input03.txt");
        let trees = [
            count_trees(&map, 1, 1),
            count_trees(&map, 3, 1),
            count_trees(&map, 5, 1),
            count_trees(&map, 7, 1),
            count_trees(&map, 1, 2),
        ];
        let total = trees.iter().fold(1, |a, b| a * b);
        assert_eq!(6050183040, total);
    }
}
