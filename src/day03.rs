use aoc_runner_derive::{aoc, aoc_generator};
use std::string::ParseError;

struct Row {
    r: Vec<bool>,
}

struct Hill {
    rows: Vec<Row>,
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Result<Hill, ParseError> {
    let mut hill = Hill { rows: Vec::new() };

    for line in input.lines() {
        let mut row = Row { r: Vec::new() };

        for c in line.chars() {
            let test: bool = c == '#';
            row.r.push(test);
        }

        hill.rows.push(row);
    }
    Ok(hill)
}

fn logic(hill: &Hill, down: i8, right: i8) -> u32 {
    let mut trees: u32 = 0;

    let mut x: usize = 0;
    let mut y: usize = 0;
    let row_width: usize = hill.rows[0].r.len();
    let hill_height: usize = hill.rows.len();
    while y < hill_height {
        if let true = hill.rows[y].r[x] {
            trees += 1;
        }

        x += right as usize; // move across the hill
        if x >= row_width {
            // check if we need to expand (i.e. wrap-around) the columns
            x = x % row_width;
        }
        y += down as usize; // move down the hill
    }

    trees
}

#[aoc(day3, part1)]
fn part1(hill: &Hill) -> u32 {
    let trees: u32 = logic(&hill, 1, 3);
    trees
}

#[aoc(day3, part2)]
fn part2(hill: &Hill) -> u32 {
    let trees1 = logic(&hill, 1, 1);
    let trees2 = logic(&hill, 1, 3);
    let trees3 = logic(&hill, 1, 5);
    let trees4 = logic(&hill, 1, 7);
    let trees5 = logic(&hill, 2, 1);

    trees1 * trees2 * trees3 * trees4 * trees5
}
