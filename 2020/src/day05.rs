use aoc_runner_derive::aoc;
use std::cmp;
use std::ops::Range;

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    let mut max_seat_id: i32 = -1;

    for line in input.lines() {
        let (row_input, col_input) = line.split_at(7);
        let row = find_row(&row_input);
        let col = find_col(&col_input);
        let seat_id = calc_seat_id(&row, &col);

        max_seat_id = cmp::max(seat_id, max_seat_id);
    }
    max_seat_id
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    let mut seat_ids: Vec<i32> = Vec::new();

    for line in input.lines() {
        let (row_input, col_input) = line.split_at(7);
        let row = find_row(&row_input);
        let col = find_col(&col_input);
        let seat_id = calc_seat_id(&row, &col);

        seat_ids.push(seat_id);
    }
    seat_ids.sort();

    for (i, id) in seat_ids.iter().enumerate() {
        if i > 0 {
            if seat_ids[i - 1] + 1 != seat_ids[i] {
                return seat_ids[i - 1] + 1;
            }
        }
    }

    return -1;
}

fn calc_seat_id(r: &i32, c: &i32) -> i32 {
    r * 8 + c
}

fn find(min: i32, max: i32, cntl: Vec<bool>) -> i32 {
    let range = Range {
        start: 0,
        end: cntl.len(),
    };

    let mut high = max;
    let mut low = min;
    for i in range {
        let diff = (high - low) / 2;
        if cntl[i] {
            high -= diff;
        } else {
            low += diff;
        }
    }
    if Some(&true) == cntl.last() {
        return low;
    } else {
        return high - 1;
    }
}

fn find_row(input: &str) -> i32 {
    let v: Vec<bool> = input.chars().map(|c| c == 'F').collect();
    find(0, 128, v)
}

fn find_col(input: &str) -> i32 {
    let v: Vec<bool> = input.chars().map(|c| c == 'L').collect();
    find(0, 8, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input1: &str = "FBFBBFFRLR"; // row 44, column 5, seat ID 357
        let input2: &str = "BFFFBBFRRR"; // row 70, column 7, seat ID 567.
        let input3: &str = "FFFBBBFRRR"; // row 14, column 7, seat ID 119.
        let input4: &str = "BBFFBBFRLL"; // row 102, column 4, seat ID 820.

        assert_eq!(calc_seat_id(&44, &5), 357);
        assert_eq!(find_row(&"FBFBBFF"), 44);
        assert_eq!(find_row(&"BFFFBBF"), 70);
        assert_eq!(find_row(&"FFFBBBF"), 14);
        assert_eq!(find_row(&"BBFFBBF"), 102);

        assert_eq!(find_col(&"RLR"), 5);
        assert_eq!(find_col(&"RRR"), 7);
        assert_eq!(find_col(&"RLL"), 4);

        assert_eq!(part1(&input1), 357);
        assert_eq!(part1(&input2), 567);
        assert_eq!(part1(&input3), 119);
        assert_eq!(part1(&input4), 820);
    }
}
