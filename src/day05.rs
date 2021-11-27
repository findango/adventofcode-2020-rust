#![allow(dead_code)]

use std::fs::read_to_string;

type Seat = (usize, usize, usize);

fn load_input(filename: &str) -> Vec<Seat> {
    let input = read_to_string(&filename).unwrap();
    input.lines().into_iter().map(parse_seat).collect()
}

fn from_binary(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn parse_seat(seat: &str) -> Seat {
    let (row_str, col_str) = seat.split_at(7);
    let row = from_binary(row_str.replace("F", "0").replace("B", "1").as_str());
    let col = from_binary(col_str.replace("L", "0").replace("R", "1").as_str());
    (row, col, row * 8 + col)
}

#[cfg(test)]
mod tests {
    use super::*;
    use array_tool::vec::Uniq;
    use itertools::{max, min};

    #[test]
    fn example_1() {
        assert_eq!((44, 5, 357), parse_seat("FBFBBFFRLR"));
        assert_eq!((70, 7, 567), parse_seat("BFFFBBFRRR"));
        assert_eq!((14, 7, 119), parse_seat("FFFBBBFRRR"));
        assert_eq!((102, 4, 820), parse_seat("BBFFBBFRLL"));
    }

    #[test]
    fn part_1() {
        let seats = load_input("input/input05.txt");
        let max_seat = seats.into_iter().map(|s| s.2).max().unwrap();
        assert_eq!(919, max_seat);
    }

    #[test]
    fn part_2() {
        let seat_ids: Vec<usize> = load_input("input/input05.txt")
            .into_iter()
            .map(|s| s.2)
            .collect();
        let (first, last) = (min(&seat_ids).unwrap(), max(&seat_ids).unwrap());
        let all_seats: Vec<usize> = (*first..*last).collect();
        let empty = all_seats.uniq(seat_ids);
        assert_eq!(642, empty[0]);
    }
}
