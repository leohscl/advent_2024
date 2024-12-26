use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

fn convert_grid(input: &str) -> (Vec<i32>, usize, usize) {
    let width = input.split('\n').next().unwrap().len();
    let height = input.chars().filter(|&c| c == '\n').count();
    let grid = input.chars().filter(|&c| c != '\n').map(|c| c.to_digit(10).unwrap_or(1) as i32).collect();
    (grid, width, height)
}

fn solve(input: &str) -> usize {
    let (grid, width, height) = convert_grid(input);
    (0..grid.len()).map(|i_grid| {
        let mut hash_nines = HashSet::new();
        if grid[i_grid] == 0 {
            find_paths(i_grid, &grid, width, height, &mut hash_nines)
        }
        let count = hash_nines.iter().count();
        count
    }).sum()
}

fn solve_2(input: &str) -> usize {
    let (grid, width, height) = convert_grid(input);
    (0..grid.len()).map(|i_grid| {
        if grid[i_grid] == 0 {
            find_paths_rating(i_grid, &grid, width, height)
        } else {
            0
        }
    }).sum()
}

fn find_paths(current_position: usize, grid: &[i32], width: usize, height: usize, hash_nines: &mut HashSet<usize>) {
    let current_number = grid[current_position];
    if current_number == 9 {
        hash_nines.insert(current_position);
        return
    }
    // if not on first line
    if current_position >= width {
        let new_position = current_position - width;
        if grid[new_position] == current_number + 1 {
            find_paths(new_position, grid, width, height, hash_nines)
        }
    }
    // if not on last line
    if current_position + width < grid.len() {
        let new_position = current_position + width;
        if grid[new_position] == current_number + 1 {
            find_paths(new_position, grid, width, height, hash_nines)
        }
    }
    // if not on first col
    if current_position % width != 0 {
        let new_position = current_position - 1;
        if grid[new_position] == current_number + 1 {
            find_paths(new_position, grid, width, height, hash_nines)
        }
    }
    // if not on last col
    if current_position % width != width - 1 {
        let new_position = current_position + 1;
        if grid[new_position] == current_number + 1 {
            find_paths(new_position, grid, width, height, hash_nines)
        }
    }
}

fn find_paths_rating(current_position: usize, grid: &[i32], width: usize, height: usize) -> usize {
    let mut num_path = 0;
    let current_number = grid[current_position];
    if current_number == 9 {
        return 1
    }
    // if not on first line
    if current_position >= width {
        let new_position = current_position - width;
        if grid[new_position] == current_number + 1 {
            num_path += find_paths_rating(new_position, grid, width, height)
        }
    }
    // if not on last line
    if current_position + width < grid.len() {
        let new_position = current_position + width;
        if grid[new_position] == current_number + 1 {
            num_path += find_paths_rating(new_position, grid, width, height)
        }
    }
    // if not on first col
    if current_position % width != 0 {
        let new_position = current_position - 1;
        if grid[new_position] == current_number + 1 {
            num_path += find_paths_rating(new_position, grid, width, height)
        }
    }
    // if not on last col
    if current_position % width != width - 1 {
        let new_position = current_position + 1;
        if grid[new_position] == current_number + 1 {
            num_path += find_paths_rating(new_position, grid, width, height)
        }
    }
    num_path
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 36);
    }
    #[test]
    fn part_1_other() {
        let input = include_str!("../input_test_0.txt");
        let result = solve(input);
        assert_eq!(result, 2);
    }
    #[test]
    fn part_1_other_other() {
        let input = include_str!("../input_test_1.txt");
        let result = solve(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 81);
    }
}
