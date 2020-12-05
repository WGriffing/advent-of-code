use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Clone)]
struct Passport {
    byr: u32,    // birth year
    iyr: u32,    // issue year
    eyr: u32,    // exp. year
    hgt: String, // height
    hcl: String, // hair color
    ecl: String, // eye color
    pid: String, // passport id
    cid: u32,    // country id
}

impl Default for Passport {
    fn default() -> Passport {
        Passport {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: String::from(""),
            hcl: String::from(""),
            ecl: String::from(""),
            pid: String::from(""),
            cid: 0,
        }
    }
}

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<Passport> {
    let mut result: Vec<Passport> = Vec::new();

    let separator = Regex::new(r"(?P<key>[a-z]{3}):(?P<val>.*)").expect("Invalid regex");

    let mut passport: Passport = Passport::default();

    for line in input.lines() {
        if line == "" {
            result.push(passport.clone());
            passport = Passport::default();
        } else {
            for group in line.split(" ") {
                for caps in separator.captures_iter(group) {
                    let key = &caps["key"];
                    let val = &caps["val"];
                    //println!("{} {}", &caps["key"], &caps["val"]);
                    match key {
                        "byr" => passport.byr = val.parse::<u32>().unwrap(),
                        "iyr" => passport.iyr = val.parse::<u32>().unwrap(),
                        "eyr" => passport.eyr = val.parse::<u32>().unwrap(),
                        "hgt" => passport.hgt = val.parse::<String>().unwrap(),
                        "hcl" => passport.hcl = val.parse::<String>().unwrap(),
                        "ecl" => passport.ecl = val.parse::<String>().unwrap(),
                        "pid" => passport.pid = val.parse::<String>().unwrap(), // required because some pids contain non-numeric characters
                        "cid" => passport.cid = val.parse::<u32>().unwrap(),
                        _ => (),
                    }
                }
            }
        }
    }
    result.push(passport.clone());

    result
}

#[aoc(day4, part1)]
fn part1(data: &[Passport]) -> u32 {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid: u32 = 0;

    for passport in data {
        let required_passes = required.len();
        let mut passes: usize = 0;
        for key in &required {
            if let true = match key {
                &"byr" => passport.byr != 0,
                &"iyr" => passport.iyr != 0,
                &"eyr" => passport.eyr != 0,
                &"hgt" => passport.hgt != String::from(""),
                &"hcl" => passport.hcl != String::from(""),
                &"ecl" => passport.ecl != String::from(""),
                &"pid" => passport.pid != String::from(""),
                &"cid" => passport.cid != 0,
                &_ => false,
            } {
                passes += 1;
            }
        }
        if passes == required_passes {
            valid += 1
        }
    }

    valid
}

#[aoc(day4, part2)]
fn part2(data: &[Passport]) -> u32 {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid: u32 = 0;

    for passport in data {
        let required_passes = required.len();
        let mut passes: usize = 0;
        for key in &required {
            if let true = match key {
                &"byr" => 1920 <= passport.byr && passport.byr <= 2002,
                &"iyr" => 2010 <= passport.iyr && passport.iyr <= 2020,
                &"eyr" => 2020 <= passport.eyr && passport.eyr <= 2030,
                &"hgt" => {
                    let mut height = passport.hgt.clone();
                    if passport.hgt.len() >= 4 {
                        let units = height.split_off(passport.hgt.len() - 2);
                        match units.as_str() {
                            "cm" => {
                                let height_num = height.parse::<u32>().unwrap();
                                150 <= height_num && height_num <= 193
                            }
                            "in" => {
                                let height_num = height.parse::<u32>().unwrap();
                                59 <= height_num && height_num <= 76
                            }
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                &"hcl" => {
                    let re = Regex::new(r"^(?P<start>#)([a-f0-9]{6})").expect("Invalid regex");
                    re.is_match(passport.hcl.as_str()) && passport.hcl.len() == 7
                }
                &"ecl" => {
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                        .contains(&passport.ecl.as_str())
                        && passport.ecl.len() == 3
                }
                &"pid" => {
                    let re = Regex::new(r"^[0-9]{9}$").expect("Invalid regex");
                    re.is_match(passport.pid.as_str()) && passport.pid.len() == 9
                }
                &"cid" => passport.cid != 0,
                &_ => false,
            } {
                passes += 1;
            } else {
                //println!("failed {} check", key)
            }
        }
        //println!("passed {} out of {}", passes, required_passes);
        if passes == required_passes {
            valid += 1
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let data = generator(&input);

        assert_eq!(part1(&data), 2);
        //assert_eq!(part2(&data), 0);
    }

    #[test]
    fn test_part2() {
        let input1: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let data1 = generator(&input1);

        assert_eq!(part2(&data1), 0);
        let input2: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let data2 = generator(&input2);
        assert_eq!(part2(&data2), 4);
    }
}
