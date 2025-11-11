use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum LightInstruction {
    On,
    Off,
    Toggle,
}

fn contains_digit(s: &str) -> bool {
    s.chars().any(|c| c.is_digit(10))
}

fn process_lighting_instruction(instruction: &str) -> (LightInstruction, Vec<u16>) {
    let light_instr: LightInstruction;

    match instruction {
        instruction if instruction.starts_with("turn on") => light_instr = LightInstruction::On,
        instruction if instruction.starts_with("turn off") => light_instr = LightInstruction::Off,
        instruction if instruction.starts_with("toggle") => light_instr = LightInstruction::Toggle,
        _ => {
            eprintln!("Lighting instruction not formatted properly.");
            std::process::exit(1);
        },
    }

    let stringed_numbers: Vec<&str> = instruction
        .split_whitespace()
        .filter(|s| contains_digit(s))
        .collect();

    let mut numbers: Vec<u16> = Vec::new();
    for string in stringed_numbers {
        let mut num: Vec<u16> = string
            .split(',')
            .filter_map(|s| s.parse::<u16>().ok())
            .collect();

        numbers.append(&mut num);
    }

    (light_instr, numbers)
}

fn main() {
    println!("Hello, world!");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all_lift_on() {
        let test: &str = "turn on 0,0 through 999,999";
        assert_eq!(process_lighting_instruction(test), (LightInstruction::On, vec![0, 0, 999, 999]));
    }
}
