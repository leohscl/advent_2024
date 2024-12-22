use itertools::Itertools;


fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

fn solve(input: &str) -> usize {
    use Direction::*;
    let directions = [Up, Down, Left, Right, UpRight, UpLeft, DownRight, DownLeft];
    let width = input.split('\n').next().unwrap().len();
    let height = input.chars().filter(|&c| c == '\n').count();
    let grid = input.chars().filter(|&c| c != '\n').collect_vec();
    assert_eq!(grid.len(), width * height);
    (0..width).cartesian_product(0..height).map(|(i_start, j_start)| {
        let start_index = i_start * width + j_start;
        if grid[start_index] != 'X' {
            return 0
        }
        directions.into_iter().filter(|&d| check_path(&grid, d, start_index, width)).count()
    }).sum()
}

fn solve_2(input: &str) -> usize {
    use Direction::*;
    let width = input.split('\n').next().unwrap().len();
    let height = input.chars().filter(|&c| c == '\n').count();
    let grid = input.chars().filter(|&c| c != '\n').collect_vec();
    assert_eq!(grid.len(), width * height);
    (0..width).cartesian_product(0..height).filter(|&(i_start, j_start)| {
        let start_index = i_start * width + j_start;
        if grid[start_index] != 'A' {
            return false
        }
        let max = grid.len();
        // diagonal
        let opt_char_1 = DownLeft.compute_next_pos(start_index, width, max).and_then(|index| Some(grid[index])).filter(|&c| c == 'M' || c == 'S');
        let opt_char_2 = UpRight.compute_next_pos(start_index, width, max).and_then(|index| Some(grid[index])).filter(|&c| c == 'M' || c == 'S');
        let opt_char_3 = DownRight.compute_next_pos(start_index, width, max).and_then(|index| Some(grid[index])).filter(|&c| c == 'M' || c == 'S');
        let opt_char_4 = UpLeft.compute_next_pos(start_index, width, max).and_then(|index| Some(grid[index])).filter(|&c| c == 'M' || c == 'S');
        opt_char_1.is_some() && opt_char_2.is_some() && opt_char_3.is_some() && opt_char_4.is_some() && opt_char_1 != opt_char_2 && opt_char_3 != opt_char_4
    }).count()
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

fn check_add(max: usize, left: usize, right: usize) -> Option<usize> {
    let results = left + right;
    if results >= max {
        None
    } else {
        Some(results)
    }
}

impl Direction {
    fn compute_next_pos(&self, old_pos: usize, width: usize, max: usize) -> Option<usize> {
        use Direction::*;
        match self {
            Up => old_pos.checked_sub(width),
            Down => check_add(max, old_pos, width),
            Left => if old_pos % width == 0 {None} else {old_pos.checked_sub(1)},
            Right => if (old_pos + 1) % width == 0 {None} else {Some(old_pos + 1)},
            UpRight => Right.compute_next_pos(old_pos, width, max).and_then(|pos| Up.compute_next_pos(pos, width, max)),
            UpLeft => Left.compute_next_pos(old_pos, width, max).and_then(|pos| Up.compute_next_pos(pos, width, max)),
            DownLeft => Left.compute_next_pos(old_pos, width, max).and_then(|pos| Down.compute_next_pos(pos, width, max)),
            DownRight => Right.compute_next_pos(old_pos, width, max).and_then(|pos| Down.compute_next_pos(pos, width, max)),
        }
    }
}

fn check_path(grid: &[char], direction: Direction, start_position: usize, width: usize) -> bool {
    let word_left = ['M', 'A', 'S'];
    let mut next_position = direction.compute_next_pos(start_position, width, grid.len());
    word_left.into_iter().fold(true, |acc, c| {
        //dbg!(next_position);
        if let Some(index) = next_position {
            next_position = direction.compute_next_pos(index, width, grid.len());
            acc && (grid[index] == c)
        } else {
            false
        }
    })
}
//
//fn solve_2(input: &str) -> u32 {
//}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 9);
    }
}
