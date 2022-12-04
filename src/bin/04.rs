use advent_of_code::helpers::{parse_str_to_num, vec_of_strings};
use std::ops::RangeInclusive;

fn get_range(str_to_split: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let (left, right) = str_to_split.split_once(",").unwrap();

    let (lower_left, higher_left) = left.split_once("-").unwrap();
    let (lower_right, higher_right) = right.split_once("-").unwrap();

    let lower_left_num = parse_str_to_num(lower_left);
    let higher_left_num = parse_str_to_num(higher_left);

    let lower_right_num = parse_str_to_num(lower_right);
    let higher_right_num = parse_str_to_num(higher_right);

    if lower_left_num < lower_right_num {
        return (
            lower_left_num..=higher_left_num,
            lower_right_num..=higher_right_num,
        );
    }

    (
        lower_right_num..=higher_right_num,
        lower_left_num..=higher_left_num,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let strings = vec_of_strings(input);
    let mut result = 0;

    for s in strings {
        let (left, right) = s.split_once(",").unwrap();

        let (lower_left, higher_left) = left.split_once("-").unwrap();
        let (lower_right, higher_right) = right.split_once("-").unwrap();

        let left_start = parse_str_to_num(lower_left);
        let left_end = parse_str_to_num(higher_left);

        let right_start = parse_str_to_num(lower_right);
        let right_end = parse_str_to_num(higher_right);

        if (left_start <= right_start && left_end >= right_end) || (right_start <= left_start && right_end >= left_end) {
            result += 1
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let strings = vec_of_strings(input);
    let mut result = 0;

    for s in strings {
        let range = get_range(s);

        if range.1.into_iter().any(|e| range.0.contains(&e)) {
            result += 1
        }
        
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
