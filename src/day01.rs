use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn generator(input: &str) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for line in input.lines() {
        output.push(line.parse::<i32>().unwrap());
    }
    output
}

fn logic(desired: &i32, input_arr: &[i32]) -> i32 {
    for value in input_arr.iter() {
        let remainder = desired - value;

        if input_arr.contains(&remainder) {
            return remainder * value;
        }
    }
    return -1;
}

#[aoc(day1, part1)]
fn part1(input_arr: &[i32]) -> i32 {
    let desired: i32 = 2020;
    logic(&desired, &input_arr)
}

#[aoc(day1, part2)]
fn part2(input_arr: &[i32]) -> i32 {
    let desired: i32 = 2020;
    for value in input_arr.iter() {
        let remainder = desired - value;

        let partial_product = logic(&remainder, &input_arr);
        if partial_product > 0 {
            return partial_product * value;
        }
    }

    return -1;
}
