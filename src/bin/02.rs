use advent_of_code::helpers::vec_of_strings;

/*
 * 0 = draw,
 * 1 = win
 * 2 = loss
 * 
 * table structure: [[rock], [paper], [scissor]]
 * 
*/
static WIN_TABLE: [[u32; 3]; 3] = [[0, 2, 1], [1, 0, 2], [2, 1, 0]];

fn map_move(a_move: &str) -> usize {
    match a_move {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => 0
    }
}

fn check_win(opponent: &str, you: &str) -> u32 {
    let o = map_move(opponent);
    let y = map_move(you);
    let game_result = WIN_TABLE[y][o];
    let extra_score = 1 + u32::try_from(y).unwrap();

    match game_result {
        0 => 3 + extra_score,
        1 => 6 + extra_score,
        2 => 0 + extra_score,
        _ => 0
    }
}

fn select_move(opponent: &str, outcome: &str) -> String {
    let loss_choice = match opponent {
        "A" => "Z",
        "B" => "X",
        "C" => "Y",
        _ => ""
    };

    let win_choice = match opponent {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        _ => ""
    };

    match outcome {
        "Y" => String::from(opponent),
        "X" => String::from(loss_choice),
        "Z" => String::from(win_choice),
        _ => String::from("")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let strings = vec_of_strings(input);
    let mut result = 0;

    for s in strings {
        let (left, right) = s.split_once(" ").unwrap();
        let score = check_win(left, right);
        result += score
    }
    
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let strings = vec_of_strings(input);

    let mut result = 0;

    for s in strings {
        let (left, right) = s.split_once(" ").unwrap();
        let decided_move = select_move(left, right);
        let score = check_win(left, &decided_move);
        result += score
    }
    
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
