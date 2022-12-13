use advent_of_code::helpers::{vec_of_strings,parse_string_to_usize};
use itertools::Itertools;

fn get_score_top(grid: &Vec<Vec<String>>, row: &usize, column: &usize, current_tree_height: &usize) -> usize {
    let mut result: usize = 0;
    

    for i in (0..*row).rev() {
        let n = parse_string_to_usize(&grid[i][*column]);
        if n < *current_tree_height {
            result += 1;
        } else {
            break;
        }
    }

    if result == (0..*row).len() { 
        return result; 
    } 

    result + 1
}

fn get_score_left(grid: &Vec<Vec<String>>, row: &usize, column: &usize, current_tree_height: &usize) -> usize {
    let tree = parse_string_to_usize(&grid[*row][*column]);
    if tree == 0 {
        return 0;
    }

    let mut result: usize = 0;

    for i in (0..*column).rev() {
        let n = parse_string_to_usize(&grid[*row][i]);
         
        if n < *current_tree_height {
            result += 1;
        } else {
            break;
        }
    }

    if result == *column { return result; } else { return result + 1; }
}

fn get_score_right(grid: &Vec<Vec<String>>, row: &usize, column: &usize, current_tree_height: &usize, max_col: &usize) -> usize {
    let mut result: usize = 0;

    for i in *column..*max_col {
        let n = parse_string_to_usize(&grid[*row][i]);
         
        if n < *current_tree_height {
            result += 1;
        } else {
            break;
        }
    }

    if result == max_col - column { return result; } else { return result + 1; }
}

fn get_score_bottom(grid: &Vec<Vec<String>>, row: &usize, column: &usize, current_tree_height: &usize, max_row: &usize) -> usize {
    let mut result: usize = 0;

    for i in *row..*max_row {
        let n = parse_string_to_usize(&grid[i][*column]);
         
        if n < *current_tree_height {
            result += 1;
        } else {
            break;
        }
    }

    if result == max_row - row { return result; } else { return result + 1; }
}


fn get_grid(strings: Vec<&str>) -> Vec<Vec<String>> {
    strings.iter().map(|e| e.chars().map(|f| String::from(f)).collect_vec()).collect()
}

fn is_visible_top(grid: &Vec<Vec<String>>, row: &usize, column: &usize, current_tree_height: &usize) -> bool {
    (0..*row).rev().into_iter().map(|i| parse_string_to_usize(&grid[i][*column])).all(|e| e < *current_tree_height)
}

fn is_visible_bottom(grid: &Vec<Vec<String>>, row: &usize, column: &usize, current_tree_height: &usize, max_row: &usize) -> bool {
    (*row..*max_row).into_iter().map(|i| parse_string_to_usize(&grid[i][*column])).all(|e| e < *current_tree_height)
}

fn is_visible_left(grid: &Vec<Vec<String>>, row: &usize, column: &usize, current_tree_height: &usize) -> bool {
    (0..*column).rev().into_iter().map(|i| parse_string_to_usize(&grid[*row][i])).all(|e| e < *current_tree_height)
}

fn is_visible_right(grid: &Vec<Vec<String>>, row: &usize, column: &usize, current_tree_height: &usize, max_col: &usize) -> bool {
    (*column..*max_col).into_iter().map(|i| parse_string_to_usize(&grid[*row][i])).all(|e| e < *current_tree_height)
}

pub fn part_one(input: &str) -> Option<u32> {
    let strings = vec_of_strings(input);
    let grid: Vec<Vec<String>> = get_grid(strings);
    let mut visible_trees = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for (idx, current_tree) in row.iter().enumerate() {
            let tree = parse_string_to_usize(current_tree);
            
            if row_idx == 0 || idx == 0 {
                visible_trees += 1;
                continue;
            }
            
            let inc_idx = idx + 1;
            let inc_row_idx = row_idx + 1;

            let score_top = is_visible_top(&grid, &row_idx, &idx, &tree);
            let score_left = is_visible_left(&grid, &row_idx, &idx, &tree);
            let score_right = is_visible_right(&grid, &row_idx, &inc_idx, &tree, &row.len());
            let score_bottom = is_visible_bottom(&grid, &inc_row_idx, &idx, &tree, &grid.len());

            if vec![score_top, score_bottom, score_left, score_right].iter().any(|e| *e == true) {
                visible_trees += 1;
            }

        }
    }    
    
    Some(visible_trees)
}

pub fn part_two(input: &str) -> Option<usize> {
    let strings = vec_of_strings(input);
    let grid: Vec<Vec<String>> = get_grid(strings);
    let mut result_scenic_score: Vec<usize> = vec![];

    for (row_idx, row) in grid.iter().enumerate() {
        for (idx, current_tree) in row.iter().enumerate() {            
            let tree = parse_string_to_usize(current_tree);
            
            if row_idx == 0 || idx == 0 {
                continue;
            }

            let inc_idx = idx + 1;
            let inc_row_idx = row_idx + 1;

            let score_top: usize = get_score_top(&grid, &row_idx, &idx, &tree);
            let score_left: usize = get_score_left(&grid, &row_idx, &idx, &tree);
            let score_right: usize = get_score_right(&grid, &row_idx, &inc_idx, &tree, &row.len());
            let score_bottom: usize = get_score_bottom(&grid, &inc_row_idx, &idx, &tree, &grid.len());

            let result = vec![score_top, score_left, score_right, score_bottom];
            let scenic_score = result.into_iter().reduce(|a, b| a * b).unwrap();

            result_scenic_score.push(scenic_score);
        }
    }
    
    Some(*result_scenic_score.iter().max().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}

// [1, 2, 1, 4, 1, 1, 2, 2, 1, 8, 3, 24, 2, 1, 12, 0]
// [1, 4, 1, 0, 6, 1, 2, 0, 1, 8, 3, 0, 0, 0, 0, 0]