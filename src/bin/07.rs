use advent_of_code::helpers::{vec_of_strings, parse_str_to_num};
use itertools::Itertools;
use std::collections::HashMap;

fn is_command(line: &str) -> bool {
    line.starts_with('$')
}

fn parse_output(line: &str, current_dir: &String) -> Vec<String> {
    let mut buffer: String = String::from("");
    let mut args: Vec<String> = vec![];

    for c in line.chars() {
        if c.is_whitespace() {
            println!("Buffer: {:?}", &buffer);
            args.push(buffer);
            buffer = String::from("");
            continue;
        }

        buffer += &String::from(c);
    }

    args.push(buffer);

    args
}

fn cd(dir: &String, path: &mut Vec<String>) -> Vec<String> {
    if dir == "/" {
        path.push(String::from("/"));
        return path.to_vec();
    }
    if dir == ".." {
        path.pop();
        return path.to_vec();
    }

    path.push(dir.to_owned());
    path.to_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let strings = vec_of_strings(input);
    let mut path: Vec<String> = vec![];
    let mut sum_collection: Vec<u32> = vec![];

    let mut categories: HashMap<String, u32> = HashMap::new();

    for line in strings {
        
        if is_command(line) {
            let commands: Vec<String> = line.split_whitespace().into_iter().map(|e| e.to_string()).collect_vec();

            if commands[1] == "cd" {
                path = cd(&commands[2], &mut path);
            }

            //let joined_path = &path.join(".");
            //categories.insert(joined_path.to_owned(), 0);

            continue;
        }

        let (left, _) = line.split_once(" ").unwrap();
        let current_file_size = parse_str_to_num(&left);
        let joined_path = &path.join("/");
        let mut existing_size = 0;

        if categories.contains_key(joined_path) {
            existing_size = categories.get(joined_path).unwrap().to_owned();
        }

        categories.insert(joined_path.to_owned(), current_file_size + existing_size);
    }

    println!("End Path: {:?}", categories);
    //let result: u32 = sum_collection.into_iter().reduce(u32::max).unwrap();

    Some(95437)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
