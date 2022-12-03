use advent_of_code::helpers::vec_of_strings;
use advent_of_code::helpers::get_alphabet;

use std::slice::Chunks;
use std::collections::HashSet;

fn find_common_characters(left: &str, right: &str) -> Vec<String> {
    left.chars()
        .filter(|e| right.contains(*e))
        .map(|c| c.to_string())
        .collect()
}

fn find_position(alphabet: &Vec<String>, character: &String) -> u32 {
    let index = alphabet.iter().position(|r| r == character).map(|e| u32::try_from(e).unwrap());
    index.unwrap() + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let strings = vec_of_strings(input);
    let alphabet = get_alphabet();
    let mut result: u32 = 0;

    for s in strings {
        let half = s.len() / 2;
        let left = &s[0..half];
        let right = &s[half..s.len()];

        let common_characters: Vec<String> = find_common_characters(left, right);
        let position = find_position(&alphabet, &common_characters[0]);

        result += position;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let strings = vec_of_strings(input);
    let chunks: Chunks<&str> = strings.chunks(3);

    for group in chunks.into_iter() {
        
        // do stuff

    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
