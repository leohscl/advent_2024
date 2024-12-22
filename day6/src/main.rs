use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

#[derive(Clone, Copy)]
enum Element {
    Empty,
    Occupied,
}

fn convert_grid(input: &str) -> (Vec<Element>, usize, usize, usize) {
    let width = input.split('\n').next().unwrap().len();
    let height = input.chars().filter(|&c| c == '\n').count();
    let mut position = 0;
    let grid = input.chars().filter(|&c| c != '\n').enumerate().map(|(i, c)| 
        match c {
            '.' => Element::Empty,
            '^' => {
                position = i as usize;
                Element::Empty
            }
            '#' => Element::Occupied,
            _ => panic!("unexpected char"),
        }
    ).collect();
    (grid, width, height, position)
}

fn solve(input: &str) -> usize {
    let (grid, width, height, position_start) = convert_grid(input);
    let mut pos_set = HashSet::new();
    pos_set.insert(position_start);
    let mut current_direction = Direction::Up;
    let max = grid.len();
    let mut last_pos = position_start;
    let mut next_pos_opt = current_direction.compute_next_pos(last_pos, width, max);
    while let Some(position) = next_pos_opt {
        match grid[position] {
            Element::Occupied => {
                // we turn
                current_direction = current_direction.turn();
                // use old postition
                next_pos_opt = current_direction.compute_next_pos(last_pos, width, max);
                // last position is the same
            },
            Element::Empty => {
                // we move
                last_pos = position;
                pos_set.insert(last_pos);
                next_pos_opt = current_direction.compute_next_pos(position, width, max);
            },
        }
    }
    pos_set.iter().count()
}

fn check_loop(grid: Vec<Element>, width: usize, position_start: usize) -> bool {
    let mut state_set = HashSet::new();
    let mut current_direction = Direction::Up;
    let max = grid.len();
    let mut last_pos = position_start;
    let mut next_pos_opt = current_direction.compute_next_pos(last_pos, width, max);
    while let Some(position) = next_pos_opt {
        if state_set.contains(&(current_direction, position)) {
            // We are looping
            return true
        }
        state_set.insert((current_direction, last_pos));
        match grid[position] {
            Element::Occupied => {
                // we turn
                current_direction = current_direction.turn();
                // use old postition
                next_pos_opt = current_direction.compute_next_pos(last_pos, width, max);
                // last position is the same
            },
            Element::Empty => {
                // we move
                last_pos = position;
                next_pos_opt = current_direction.compute_next_pos(position, width, max);
            },
        }
    }
    false
}

fn solve_2(input: &str) -> usize {
    let (grid, width, _height, position_start) = convert_grid(input);
    (0..grid.len()).filter(|&i| {
        let mut grid_clone = grid.clone();
        grid_clone[i] = Element::Occupied;
        check_loop(grid_clone, width, position_start)
    }).count()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    fn turn(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
    fn compute_next_pos(&self, old_pos: usize, width: usize, max: usize) -> Option<usize> {
        use Direction::*;
        match self {
            Up => old_pos.checked_sub(width),
            Down => check_add(max, old_pos, width),
            Left => if old_pos % width == 0 {None} else {old_pos.checked_sub(1)},
            Right => if (old_pos + 1) % width == 0 {None} else {Some(old_pos + 1)},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 41);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 6);
    }
}
