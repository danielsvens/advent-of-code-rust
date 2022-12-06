use std::collections::HashSet;

use advent_of_code::helpers::vec_of_strings;

fn find_marker(signal: String, seq_num: usize) -> usize {
    let mut start: usize = 0;
    let mut end: usize = seq_num;
    let mut chars: HashSet<String> = HashSet::new();
    let mut result = 0;

    for i in 0..signal.len() {
        for c in signal[start..end].chars() {
            chars.insert(c.to_string());
        }

        if chars.len() == seq_num {
            result = i + seq_num;
            break;
        }
        
        chars.clear();
        
        start += 1;
        end += 1;
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let strings = vec_of_strings(input);
    let signal = String::from(strings[0]);
    let result = find_marker(signal, 4);
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let strings = vec_of_strings(input);
    let signal = String::from(strings[0]);
    let result = find_marker(signal, 14);
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
