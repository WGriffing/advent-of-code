use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            numbers.push(line.parse::<i64>().expect(line));
        }
    }
    numbers
}

#[aoc(day9, part1)]
fn part1(numbers: &Vec<i64>) -> i64 {
    process_numbers(&numbers, 25)
}

fn process_numbers(numbers: &Vec<i64>, preamble: usize) -> i64 {
    let mut queue: VecDeque<i64> = VecDeque::with_capacity(preamble);

    for i in 0..numbers.len() {
        if i < preamble {
            queue.push_back(numbers[i]);
        } else {
            let is_valid = check_for_sum(&queue, &numbers[i]);
            if !is_valid {
                return numbers[i];
            } else {
                queue.pop_front();
                queue.push_back(numbers[i]);
            }
        }
    }

    0
}

fn check_for_sum(q: &VecDeque<i64>, desired: &i64) -> bool {
    let mut l: usize = 0;
    let mut r: usize = q.len() - 1;
    let qq = &mut q.clone();

    qq.make_contiguous().sort();

    /* Now look for the two candidates in
    the sorted array*/
    println!("q = {:?}", qq);
    println!("desired = {:?}", desired);

    while l < r {
        if qq[l] + qq[r] == desired.clone() {
            return true;
        } else if qq[l] + qq[r] < desired.clone() {
            l += 1;
        } else {
            // A[i] + A[j] > sum
            r -= 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let numbers = generator(&input);
        assert_eq!(process_numbers(&numbers, 5), 127);
    }
}
