use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct Instruction {
    cmd: String,
    num: i32,
    exec: bool,
}

#[derive(Clone)]
struct Results {
    acc: i32,
    line: i32,
}

#[derive(Clone)]
struct Stack {
    i: Instruction,
    r: Results,
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
fn day8_part1(data: &Vec<Instruction>) -> i32 {
    let mut owned_data = &mut data.clone();
    let (_finite, acc, _stack) = infinite_loop(&mut owned_data);

    acc
}

fn infinite_loop(data: &mut Vec<Instruction>) -> (bool, i32, Vec<Stack>) {
    let acc: &mut i32 = &mut 0;
    let line: &mut i32 = &mut 0;
    let program: &mut Vec<Instruction> = data;

    let mut stack: Vec<Stack> = Vec::new();

    let mut i: i32 = 0;
    loop {
        let prog_len = program.len() as i32;
        if *line >= prog_len {
            break;
        } else if program.get(*line as usize).unwrap().exec {
            return (false, *acc, stack);
        } else if i as usize > program.len() - 1 {
            break;
        } else {
            let s: Stack = process_inst(program, acc, line);
            stack.push(s.to_owned());
        }
        i += 1;
    }
    return (true, *acc, stack);
}

fn process_inst(program: &mut Vec<Instruction>, acc: &mut i32, l: &mut i32) -> Stack {
    let cl = *l;
    match program.get(cl as usize).unwrap().cmd.as_str() {
        "acc" => {
            *acc += program.get(cl as usize).unwrap().num;
            *l += 1;
        }
        "jmp" => {
            *l += program.get(cl as usize).unwrap().num;
        }
        "nop" => {
            *l += 1;
        }
        _ => (),
    };
    program[cl as usize].exec = true;

    Stack {
        r: Results {
            acc: *acc,
            line: *l,
        },
        i: program[cl as usize].clone(),
    }
}

#[aoc(day8, part2)]
fn day8_part2(data: &Vec<Instruction>) -> i32 {
    let mut program = &mut data.clone();
    let mut original_program = &mut data.clone();

    let mut tried: Vec<usize> = Vec::new();

    loop {
        let (finite, _acc, stack) = infinite_loop(&mut program);
        let mut s_copy = stack.clone();
        let mut idx: i32 = s_copy.len() as i32 - 1;

        if finite {
            return _acc;
        } else {
            let mut found: bool = false;
            while idx >= 0 && !found {
                let el = s_copy.pop().unwrap();
                let line = el.r.line.clone() as usize;
                if (el.i.cmd == String::from("nop") || el.i.cmd == String::from("jmp"))
                    && !tried.contains(&line)
                {
                    tried.push(el.r.line as usize);
                    found = true;
                    invert_command(&mut program[line]);
                    reset_program(&mut program, &mut original_program, &line);
                }

                idx -= 1;
            }
        }
    }
}

fn invert_command(inst: &mut Instruction) -> () {
    if inst.cmd == String::from("nop") {
        inst.cmd = String::from("jmp");
    } else {
        inst.cmd = String::from("nop");
    }
}

fn reset_program(
    program: &mut Vec<Instruction>,
    original: &mut Vec<Instruction>,
    line: &usize,
) -> () {
    for i in 0..program.len() {
        program[i].exec = false;

        if &i != line {
            program[i] = original[i].clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests1() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let data: Vec<Instruction> = generator(&input);
        assert_eq!(day8_part1(&data), 5);
        assert_eq!(day8_part2(&data), 8);
    }
}
