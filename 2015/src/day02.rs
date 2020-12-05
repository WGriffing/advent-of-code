use aoc_runner_derive::aoc;
use std::cmp;

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    let mut total: i32 = 0;

    for line in input.lines() {
        let dims: Vec<i32> = line.split('x').map(|c| c.parse::<i32>().unwrap()).collect();
        total += calc_area(&dims[0], &dims[1], &dims[2]);
    }

    total
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    let mut total: i32 = 0;

    for line in input.lines() {
        let dims: Vec<i32> = line.split('x').map(|c| c.parse::<i32>().unwrap()).collect();
        total += calc_volume(&dims[0], &dims[1], &dims[2]);
        total += calc_smallest_perimeter(&dims[0], &dims[1], &dims[2]);
    }
    total
}

fn calc_area(l: &i32, w: &i32, h: &i32) -> i32 {
    let slack = cmp::min(cmp::min(l * w, l * h), w * h);

    let area = 2 * l * w + 2 * w * h + 2 * h * l + slack;

    //println!("l={} w={} h={} slack={} area={}", l, w, h, slack, area);

    area
}

fn calc_volume(l: &i32, w: &i32, h: &i32) -> i32 {
    l * w * h
}

fn calc_smallest_perimeter(l: &i32, w: &i32, h: &i32) -> i32 {
    cmp::min(2 * l + 2 * w, cmp::min(2 * l + 2 * h, 2 * w + 2 * h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&"2x3x4"), 58);
        assert_eq!(part1(&"1x1x10"), 43);
    }
}
