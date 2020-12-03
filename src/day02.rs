use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    let mut count: i32 = 0;
    for item in input.lines() {
        let v = item.split(':').collect::<Vec<&str>>();
        let policy = v[0 as usize];
        let password = v[1 as usize];
        let v2 = policy.split(' ').collect::<Vec<&str>>();
        let min_max = v2[0];
        let v3 = min_max.split('-').collect::<Vec<&str>>();
        let letter = v2[1 as usize];
        let min: u32 = v3[0 as usize].parse().unwrap();
        let max: u32 = v3[1 as usize].parse().unwrap();

        let occurrences: u32 = password.matches(letter).count() as u32;
        if min <= occurrences && occurrences <= max {
            count += 1;
        }
    }

    count
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    let mut count: i32 = 0;
    for item in input.lines() {
        let mut v = item.rsplit(':').collect::<Vec<&str>>();
        v.reverse();
        let policy = v[0 as usize];
        let password = v[1 as usize];
        let mut v2 = policy.rsplit(' ').collect::<Vec<&str>>();
        v2.reverse();
        let min_max = v2[0];
        let v3 = min_max.split('-').collect::<Vec<&str>>();
        let letter: char = v2[1 as usize].chars().nth(0).unwrap();
        let pri: usize = v3[0 as usize].parse().unwrap();
        let sec: usize = v3[1 as usize].parse().unwrap();

        let test_pri: bool = password.chars().nth(pri).unwrap() == letter;
        let test_sec: bool = password.chars().nth(sec).unwrap() == letter;

        if (test_pri && !test_sec) || (!test_pri && test_sec) {
            count += 1;
        }
    }

    count
}
