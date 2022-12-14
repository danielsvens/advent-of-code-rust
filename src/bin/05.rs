use advent_of_code::helpers::{vec_of_strings,parse_str_to_usize};
use regex::Regex;

fn get_instuctions(strings: &Vec<&str>) -> Vec<String> {
    strings.iter().filter(|e| e.starts_with("move")).map(|line| line.to_string()).collect()
}

fn take(mut containers: Vec<String>, times: usize, from: usize, to: usize) -> Vec<String> {
    for _ in 0..times {
        let _item = containers[from - 1].chars().last().unwrap();
        let _last = containers[from - 1].len();
        
        containers[from - 1].remove(_last - 1);
        containers[to - 1].push(_item);
    };

    containers
}

fn take_multiple(mut containers: Vec<String>, times: usize, from: usize, to: usize) -> Vec<String> {
    let mut cache = String::from("");

    for _ in 0..times {
        let _item = containers[from - 1].chars().last().unwrap();
        let _last = containers[from - 1].len();
        
        containers[from - 1].remove(_last - 1);
        cache.push(_item);
    };

    for c in cache.chars().rev() {
        containers[to - 1].push(c);
    }

    containers
}

fn setup_input() -> Vec<String> {
    vec![
        String::from("HCR"), 
        String::from("BJHLSF"), 
        String::from("RMDHJTQ"), 
        String::from("SGRHZBJ"), 
        String::from("RPFZTDCB"),
        String::from("THCG"),
        String::from("SNVZBPWL"),
        String::from("RJQGC"),
        String::from("LDTRHPFS")
    ]
}

fn get_args(separator: &Regex, instruction: &String) -> Vec<usize> {
    separator.find_iter(instruction)
            .map(|m| m.as_str().to_string())
            .map(|e| parse_str_to_usize(&e))
            .collect()
}

fn get_result(stacks: &Vec<String>) -> Vec<String> {
    stacks.iter()
        .map(|e| e.chars().last().unwrap())
        .map(|e| e.to_string())
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let strings = vec_of_strings(input);
    let mut stacks: Vec<String> = setup_input();

    let separator = Regex::new(r"\d+").unwrap();
    let instructions = get_instuctions(&strings);

    for instruction in instructions {
        let args = get_args(&separator, &instruction);
        stacks = take(stacks, args[0], args[1], args[2]);
    };

    let result: Vec<String> = get_result(&stacks);

    Some(result.join(""))
}

pub fn part_two(input: &str) -> Option<String> {
    let strings = vec_of_strings(input);
    let mut stacks: Vec<String> = setup_input();

    let separator = Regex::new(r"\d+").unwrap();
    let instructions = get_instuctions(&strings);

    for instruction in instructions {
        let args = get_args(&separator, &instruction);
        stacks = take_multiple(stacks, args[0], args[1], args[2]);
    };

    let result: Vec<String> = get_result(&stacks);

    Some(result.join(""))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("SLCJBGLCS")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("LSFJBGLCS")));
    }
}
