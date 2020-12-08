use aoc_runner_derive::{aoc, aoc_generator};
use std::mem;

#[derive(Clone)]
struct Instruction {
    cmd: String,
    num: i32,
    exec: bool,
}

struct Result {
    acc: i32,
    line: i32,
    exec: bool,
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(' ').collect();
        let cmd = line_parts[0];
        let num = line_parts[1];
        instructions.push(Instruction {
            cmd: String::from(cmd),
            num: num.parse::<i32>().unwrap(),
            exec: false,
        });
    }

    instructions
}

#[aoc(day8, part1)]
fn day8_part1(data: &[Instruction]) -> i32 {
    let mut acc: i32 = 0;
    let mut line: i32 = 0;

    let mut copied_data = data.to_vec();

    loop {
        let instruction = copied_data.get(line as usize).unwrap();
        if instruction.exec {
            return acc;
        } else {
            let result = process_inst(&instruction, acc, line);

            let new_instruction = Instruction {
                cmd: instruction.cmd.clone(),
                num: instruction.num,
                exec: result.exec,
            };
            let _ = mem::replace(&mut copied_data[line as usize], new_instruction);
            line = result.line;
            acc = result.acc;
        }
    }
}

/*#[aoc(day8, part2)]
fn day8_part2(data: &[Instruction]) -> i32 {
    0
}*/

fn process_inst(instruction: &Instruction, acc: i32, line: i32) -> Result {
    let mut result = Result {
        acc: acc,
        line: line,
        exec: true,
    };
    match instruction.cmd.as_str() {
        "acc" => {
            result.acc += instruction.num;
            result.line += 1;
        }
        "jmp" => {
            result.line += instruction.num;
        }
        "nop" => {
            result.line += 1;
        }
        _ => (),
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let data = generator(&input);
        assert_eq!(day8_part1(&data), 5);
    }
}
