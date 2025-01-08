use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
    dbg!(solve_2(input));
}

#[derive(Clone, Copy, PartialEq)]
enum Element {
    Empty,
    Box,
    Wall,
}

#[derive(Clone, Copy, PartialEq)]
enum Element2 {
    Empty,
    Box(bool),
    Wall,
}

fn convert_grid(grid_input: &str) -> (HashMap<(i64, i64), Element>, i64, i64, (i64, i64)) {
    let width = grid_input.split('\n').next().unwrap().len() as i64;
    let height = grid_input.chars().filter(|&c| c == '\n').count() as i64;
    let (mut robot_x, mut robot_y) = (0, 0);
    let grid = grid_input.chars().filter(|&c| c != '\n').enumerate().fold(HashMap::new(), |mut hash, (i, c)| {
        let (x, y) = ((i as i64 % width), (i as i64 / width));
        let element = match c {
            '.' => Element::Empty,
            '@' => {
                (robot_x, robot_y) = (x, y);
                Element::Empty
            },
            '#' => Element::Wall,
            'O' => Element::Box,
            _ => panic!()
        };
        hash.insert((x,y), element);
        hash
    });
    (grid, width, height, (robot_x, robot_y))
}


fn solve(input: &str) -> i64 {
    let mut input_parts = input.split("\n\n");
    let (grid_input, instructions) = (input_parts.next().unwrap(), input_parts.next().unwrap());
    let (mut grid, _width, _height, position_start) = convert_grid(grid_input);
    run_instructions(instructions, position_start, &mut grid);
    grid.into_iter().filter_map(|(key, value)| {
        if value == Element::Box {
            Some(key.0 + key.1 * 100)
        } else {
            None
        }
    }).sum()
}

fn run_instructions(instructions: &str, mut robot_pos: (i64, i64), grid: &mut HashMap<(i64, i64), Element>) {
    use Element::*;
    instructions.chars().filter(|&c| c != '\n').for_each(|c| {
        let direction = match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!(),
        };
        let target_position = direction.compute_next_pos(robot_pos);
        match grid.get(&target_position).unwrap() {
            Wall => (),
            Empty => robot_pos = target_position,
            Box => {
                let box_to_move_position = target_position;
                let mut next_position = direction.compute_next_pos(target_position);
                loop {
                    match grid.get(&next_position).unwrap() {
                        Box => {
                            next_position = direction.compute_next_pos(next_position);
                        },
                        Empty => {
                            robot_pos = box_to_move_position;
                            grid.entry(box_to_move_position).and_modify(|e| *e = Empty);
                            grid.entry(next_position).and_modify(|e| *e = Box);
                            break;
                        },
                        Wall => break
                    }
                }
            }
        }
    })
}

fn run_instructions_2(instructions: &str, mut robot_pos: (i64, i64), grid: &mut HashMap<(i64, i64), Element2>, width: i64, height: i64) {
    use Element2::*;
    instructions.chars().filter(|&c| c != '\n').for_each(|c| {
        //print_grid_2(&grid, width, height);
        let direction = match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!(),
        };
        let target_position = direction.compute_next_pos(robot_pos);
        match grid.get(&target_position).unwrap() {
            Wall => (),
            Empty => robot_pos = target_position,
            Box(is_left) => {
                let mut new_pos = vec![target_position];
                let other_pos = if *is_left {
                    Direction::Right.compute_next_pos(target_position)
                } else {
                    Direction::Left.compute_next_pos(target_position)
                };
                new_pos.push(other_pos);
                let mut all_pos_to_move = Vec::new();
                all_pos_to_move.extend(new_pos.clone().into_iter());
                let mut wall = false;
                while !new_pos.is_empty() {
                    let mut new_pos_update = vec![];
                    new_pos.clone().into_iter().for_each(|old_box_pos| {
                        let new_pos_to_check = direction.compute_next_pos(old_box_pos);
                        match grid.get(&new_pos_to_check).unwrap() {
                            Empty => (),
                            Wall => wall = true,
                            Box(is_left) => {
                                if !new_pos_update.contains(&new_pos_to_check) && !all_pos_to_move.contains(&new_pos_to_check) {
                                    new_pos_update.push(new_pos_to_check);
                                }
                                let other_pos = if *is_left {
                                    Direction::Right.compute_next_pos(new_pos_to_check)
                                } else {
                                    Direction::Left.compute_next_pos(new_pos_to_check)
                                };
                                if !new_pos_update.contains(&other_pos) && !all_pos_to_move.contains(&new_pos_to_check) {
                                    new_pos_update.push(other_pos);
                                }
                            },
                        };
                    });
                    all_pos_to_move.extend(&new_pos_update.clone());
                    new_pos = new_pos_update;
                    if wall {
                        break;
                    }
                };
                if !wall {
                    dbg!(&all_pos_to_move);
                    robot_pos = target_position;
                    all_pos_to_move.into_iter().rev().for_each(|pos| {
                        let destination = direction.compute_next_pos(pos);
                        let elt = grid.get(&pos).unwrap().clone();
                        grid.entry(destination).and_modify(|e| *e = elt);
                        grid.entry(pos).and_modify(|e| *e = Empty);
                    })
                }
            }
        }
    });
    //print_grid_2(&grid, width, height);
}


fn convert_grid_2(grid_input: &str) -> (HashMap<(i64, i64), Element2>, i64, i64, (i64, i64)) {
    let width = grid_input.split('\n').next().unwrap().len() as i64;
    let height = grid_input.chars().filter(|&c| c == '\n').count() as i64;
    let (mut robot_x, mut robot_y) = (0, 0);
    let grid = grid_input.chars().filter(|&c| c != '\n').enumerate().fold(HashMap::new(), |mut hash, (i, c)| {
        let (x, y) = ((i as i64 % width), (i as i64 / width));
        match c {
            '.' => {
                hash.insert((2*x,y), Element2::Empty);
                hash.insert((2*x + 1,y), Element2::Empty);
            },
            '@' => {
                hash.insert((2*x,y), Element2::Empty);
                hash.insert((2*x + 1,y), Element2::Empty);
                (robot_x, robot_y) = (2*x, y);
            },
            '#' => {
                hash.insert((2*x,y), Element2::Wall);
                hash.insert((2*x + 1,y), Element2::Wall);
            },
            'O' => {
                hash.insert((2*x,y), Element2::Box(true));
                hash.insert((2*x + 1,y), Element2::Box(false));
            },
            _ => panic!()
        };
        hash
    });
    (grid, width * 2, height + 1, (robot_x, robot_y))
}

fn print_grid_2(grid: &HashMap<(i64, i64), Element2>, width: i64, height: i64) {
    use Element2::*;
    (0..height).for_each(|h| {
        (0..width).for_each(|w| {
            let c = match grid.get(&(w, h)).unwrap() {
                Wall => '#',
                Empty => '.',
                Box(l) => {
                    if *l {
                        '['
                    } else {
                        ']'
                    }
                },
            };
            print!("{}", c);
        });
        println!()
    });
    println!()
}

fn solve_2(input: &str) -> i64 {
    let mut input_parts = input.split("\n\n");
    let (grid_input, instructions) = (input_parts.next().unwrap(), input_parts.next().unwrap());
    let (mut grid, width, height, position_start) = convert_grid_2(grid_input);
    run_instructions_2(instructions, position_start, &mut grid, width, height);
    grid.into_iter().filter_map(|(key, value)| {
        if value == Element2::Box(true) {
            Some(key.0 + key.1 * 100)
        } else {
            None
        }
    }).sum()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn compute_next_pos(&self, robot_pos: (i64, i64)) -> (i64, i64) {
        use Direction::*;
        match self {
            Up => (robot_pos.0, robot_pos.1 - 1),
            Down => (robot_pos.0, robot_pos.1 + 1),
            Left => (robot_pos.0 - 1, robot_pos.1),
            Right => (robot_pos.0 + 1, robot_pos.1),
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
        assert_eq!(result, 10092);
    }

    //#[test]
    //fn part_2_1() {
    //    let input = include_str!("../input_test_2.txt");
    //    let result = solve_2(input);
    //    assert_eq!(result, 9021);
    //}

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 9021);
    }
}
