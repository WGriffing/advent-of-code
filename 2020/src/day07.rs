use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

pub struct Bag {
    // if the bag contains a shiny
    pub shiny: bool,

    // a hash of the sub bags and quantity
    pub sub_bags: HashMap<String, i64>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Bag> {
    let mut result = HashMap::new();

    // new line seperates passports
    for line in input.split("\n") {
        let split: Vec<&str> = line.split(" bags contain ").collect();

        // this is the string that its indexed by
        let name = split[0];
        let contains = split[1]
            .replace(".", "")
            .replace("bags", "")
            .replace("bag", "");
        let sub_bag_raw: Vec<&str> = contains.trim().split(" , ").collect();

        let mut sub_bags = HashMap::new();

        let mut contains_shiny = false;
        for sub_bag in sub_bag_raw {
            if sub_bag == "no other" {
                continue;
            }
            let partition = sub_bag.find(" ").unwrap();
            let number = sub_bag[..partition].parse::<i64>().unwrap();
            let sub_bag_name = &sub_bag[(partition + 1)..];

            if sub_bag_name == "shiny gold" {
                contains_shiny = true;
            }

            sub_bags.insert(sub_bag_name.to_string(), number);
        }
        result.insert(
            name.to_string(),
            Bag {
                shiny: contains_shiny,
                sub_bags: sub_bags,
            },
        );
    }

    result
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<String, Bag>) -> i64 {
    let mut result = HashMap::new();

    for (name, details) in input.iter() {
        // has to be within a bag
        if name == "shiny gold" {
            continue;
        }

        let bag = Bag {
            shiny: true,
            sub_bags: details.sub_bags.to_owned(),
        };

        // no need to check, already contains it
        if details.shiny {
            result.insert(name.to_owned(), bag);
            continue;
        }

        if traverse_tree(name, &input).0 {
            result.insert(name.to_owned(), bag);
        }
    }

    result.len() as i64
}

fn traverse_tree(name: &String, input: &HashMap<String, Bag>) -> (bool, i64) {
    let mut contains_shiny = false;
    let mut bags = 0;

    // bag we are searchiung
    let bag = input.get(name).unwrap();
    let sub_bags = &bag.sub_bags;

    for (sub_name, quantity) in sub_bags.iter() {
        let sub_bag = input.get(sub_name).unwrap();

        // check any under it
        let (result_contains_shiny, result_bags) = traverse_tree(sub_name, input);
        if sub_bag.shiny || result_contains_shiny {
            contains_shiny = true;
        }

        bags += quantity + (quantity * result_bags)
    }

    (contains_shiny, bags)
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<String, Bag>) -> i64 {
    let output: i64 = traverse_tree(&"shiny gold".to_string(), &input).1;

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let transformed_input = input_generator(&input);
        assert_eq!(part1(&transformed_input), 4);
        assert_eq!(part2(&transformed_input), 32);

        let input_2 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let transformed_2 = input_generator(&input_2);

        assert_eq!(part2(&transformed_2), 126);
    }
}
