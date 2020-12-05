use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    let up: i32 = input.matches('(').count() as i32;
    let down: i32 = input.matches(')').count() as i32;

    up - down
}

#[aoc(day1, part2)]
fn part2(input: &str) -> i32 {
    let mut floor: i32 = 0;
    let mut pos: i32 = 1;

    for c in input.chars() {
        match c {
            '(' => floor +=1,
            ')' => floor -=1,
            _=> ()
        }

        if floor == -1 {
            break;
        } else {
            pos +=1;
        }
    }

    pos
}