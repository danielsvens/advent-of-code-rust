use advent_of_code::helpers::vec_of_strings;
use itertools::max;


fn parse(num: &str) -> u32 {
    match num.parse::<u32>() {
        Ok(n) => n,
        Err(_) => 0,
      }
}

fn calc(input: &str) -> Vec<u32> {
    let strings = vec_of_strings(input);
    let mut nums: Vec<u32> = Vec::new();
    let mut current_num: u32 = 0;
    let mut temp;

    for n in strings {
        temp = parse(&n);

        if temp == 0 {
            nums.push(current_num);
            current_num = 0;
        }

        current_num += temp
    }

    nums.push(current_num);
    nums
}

pub fn part_one(input: &str) -> Option<u32> {
    max(calc(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nums = calc(input);
    nums.sort_by(|a, b| b.cmp(a));

    let result = &nums[0..3];
    let total = result.to_vec()
        .iter()
        .sum();
    
    Some(total)
}

 fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
