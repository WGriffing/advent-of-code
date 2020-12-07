use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let mut groups: Vec<HashMap<char, bool>> = Vec::new();

    let mut hash: HashMap<char, bool> = HashMap::new();
    for line in input.lines() {
        if line == "" {
            groups.push(hash);
            hash = HashMap::new();
        } else {
            for c in line.chars() {
                hash.insert(c, true);
            }
        }
    }
    groups.push(hash);
    let mut output: i32 = 0;

    for group in groups.iter() {
        output += group.len() as i32;
    }

    output
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i32 {
    let mut groups: Vec<HashMap<char, i32>> = Vec::new();
    let mut members: Vec<i32> = Vec::new();

    let mut hash: HashMap<char, i32> = HashMap::new();
    let mut member: i32 = 0;
    for line in input.lines() {
        if line == "" {
            members.push(member);
            groups.push(hash);
            hash = HashMap::new();
            member = 0;
        } else {
            for c in line.chars() {
                if hash.contains_key(&c) {
                    let mut count: i32 = hash.get(&c).unwrap().clone();
                    count += 1;
                    hash.insert(c, count);
                } else {
                    hash.insert(c, 1);
                }
            }
            member += 1;
        }
    }
    groups.push(hash);
    members.push(member);

    let mut output: i32 = 0;
    for (i, group) in groups.iter().enumerate() {
        for (_k, v) in group.iter() {
            if v.clone() == members[i] {
                output += 1;
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(part1(&input), 11);
        assert_eq!(part2(&input), 6);
    }
}
