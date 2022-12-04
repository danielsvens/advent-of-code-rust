use advent_of_code::helpers::get_alphabet;
use advent_of_code::helpers::vec_of_strings;

use std::slice::Chunks;

fn find_common_characters(left: &str, right: &str) -> Vec<String> {
    left.chars()
        .filter(|e| right.contains(*e))
        .map(|c| c.to_string())
        .collect()
}

fn find_position(alphabet: &Vec<String>, character: &String) -> u32 {
    let index = alphabet
        .iter()
        .position(|r| r == character)
        .map(|e| u32::try_from(e).unwrap());
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
    let alphabet = get_alphabet();
    let mut result: u32 = 0;

    for group in chunks.into_iter() {
        let (first, others) = group.split_at(1);

        let mut found_one: bool = false;
        let mut found_two: bool = false;
        let mut found_item: String = String::from("");

        for c in first[0].chars() {
            if others[0].contains(c) {
                found_one = true;
            }

            if others[1].contains(c) {
                found_two = true;
            }

            if found_one && found_two {
                found_item = c.to_string();
                break;
            }

            found_one = false;
            found_two = false;
        }

        result += find_position(&alphabet, &found_item);
    }

    Some(result)
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
        assert_eq!(part_two(&input), Some(70));
    }
}
