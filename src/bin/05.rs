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

pub fn part_one(input: &str) -> Option<String> {
    let strings = vec_of_strings(input);
    let mut stacks: Vec<String> = vec![
        String::from("HCR"), 
        String::from("BJHLSF"), 
        String::from("RMDHJTQ"), 
        String::from("SGRHZBJ"), 
        String::from("RPFZTDCB"),
        String::from("THCG"),
        String::from("SNVZBPWL"),
        String::from("RJQGC"),
        String::from("LDTRHPFS")
    ];

    let separator = Regex::new(r"\d+").unwrap();
    let instructions = get_instuctions(&strings);

    for instruction in instructions {
        let parsed: Vec<usize> = separator.find_iter(&instruction)
            .map(|m| m.as_str().to_string())
            .map(|e| parse_str_to_usize(&e))
            .collect();

        stacks = take(stacks, parsed[0], parsed[1], parsed[2]);
    };

    let result: Vec<String> = stacks.iter()
        .map(|e| e.chars().last().unwrap())
        .map(|e| e.to_string())
        .collect();

    Some(result.join(""))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
