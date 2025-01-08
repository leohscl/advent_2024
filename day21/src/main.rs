fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
    dbg!(solve_2(input));
}

type Course = Vec<Action>;
type Coord = (i64, i64);

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
//impl Direction {
//    fn compute_next_pos(&self, pos: (i64, i64)) -> (i64, i64) {
//        use Direction::*;
//        match self {
//            Up => (pos.0, pos.1 - 1),
//            Down => (pos.0, pos.1 + 1),
//            Left => (pos.0 - 1, pos.1),
//            Right => (pos.0 + 1, pos.1),
//        }
//    }
//}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Action {
    Dir(Direction),
    Press
}


fn construct_paths(mut current_course: Course, quantity_1: usize, quantity_2: usize, dir_1: Direction, dir_2: Direction) -> Vec<Course> {
    if quantity_1 == 0 {
        current_course.extend(vec![Action::Dir(dir_2); quantity_2]);
        return vec![current_course];
    }
    if quantity_2 == 0 {
        current_course.extend(vec![Action::Dir(dir_1); quantity_1]);
        return vec![current_course];
    }
    let mut return_vec = construct_paths(current_course.clone(), quantity_1 - 1, quantity_2, dir_1, dir_2);
    return_vec.extend(construct_paths(current_course, quantity_1, quantity_2 - 1, dir_1, dir_2));
    return_vec
}

fn get_grid_paths(start: Coord, end: Coord, door: bool) -> Vec<Course> {
    let w_diff = end.0 - start.0;
    let h_diff = end.1 - start.1;
    let horizontal_movement = if w_diff < 0 {
        Direction::Left
    } else {
        Direction::Right
    };
    let vertical_movement = if h_diff < 0 {
        Direction::Up
    } else {
        Direction::Down
    };
    let mut paths = construct_paths(Vec::new(), w_diff.abs() as usize, h_diff.abs() as usize, horizontal_movement, vertical_movement);
    if !door {
        // eliminate paths going to (0, 0)
        if start.0 == 0 && end.1 == 0 {
            paths.retain(|path| &path[0..(h_diff.abs() as usize)] != &vec![Action::Dir(Direction::Up); h_diff.abs() as usize]);
        }
        if start.1 == 0 && end.0 == 0 {
            paths.retain(|path| &path[0..(w_diff.abs() as usize)] != &vec![Action::Dir(Direction::Left); h_diff.abs() as usize]);
        }
    } else {
        // eliminate paths going to (0, 3)
        if start.0 == 0 && end.1 == 3 {
            paths.retain(|path| &path[0..(h_diff.abs() as usize)] != &vec![Action::Dir(Direction::Down); h_diff.abs() as usize]);
        }
        if start.1 == 0 && end.0 == 0 {
            paths.retain(|path| &path[0..(w_diff.abs() as usize)] != &vec![Action::Dir(Direction::Left); h_diff.abs() as usize]);
        }
    }
    paths
}


fn convert_action_coords_door(c: char) -> Coord {
    match c {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("Unexpected char")
    }
}

fn convert_action_coords_keypad(action: Action) -> Coord {
    use Direction::*;
    match action {
        Action::Dir(dir) => match dir {
            Left => (0, 1),
            Right => (2, 1),
            Up => (1, 0),
            Down => (1, 1),
        },
        Action::Press => (2, 0),
    }
}

fn print_course(course: &Course) {
    let string_course: String = course.iter().map(|action| {
        match action {
            Action::Press => 'A',
            Action::Dir(dir) => {
                match dir {
                    Direction::Left => '<',
                    Direction::Right => '>',
                    Direction::Up => '^',
                    Direction::Down => 'v',
                }
            }
        }
    }).collect();
    println!("{}", string_course);
}

//fn get_shortest_from_coords(coord: Coord) -> Vec<Course> {
//    coord.windows(2).fold(vec![], |mut result, coord_vec| {
//        let shortest = get_grid_paths(coord_vec[0], coord_vec[1]);
//        result.extend(shortest);
//        result.push(Action::Press);
//        result
//    })
//}

fn create_coords(course: &Course) -> Vec<Coord> {
    let press = std::iter::once(convert_action_coords_keypad(Action::Press));
    press.chain(course.into_iter().map(|c| convert_action_coords_keypad(*c))).collect()
}

fn get_numeric_value(code: &str) -> usize {
    code[0..code.len() - 1].parse().unwrap()
}


fn solve(input: &str) -> usize {
    input.split('\n').filter(|&s| s != "").map(|code| {
        let code_value = get_numeric_value(code);
        let target_coords_door: Vec<Coord> = std::iter::once(convert_action_coords_door('A')).chain(code.chars().map(|c| convert_action_coords_door(c))).collect();
        let shortest_path_value = target_coords_door.windows(2).map(|window_coord_door| {
            dbg!(&window_coord_door);
            let min = get_grid_paths(window_coord_door[0], window_coord_door[1], true).into_iter().map(|course_door| {
                dbg!(&course_door);
                let course_door_coords = create_coords(&course_door);
                dbg!(&course_door_coords);
                course_door_coords.windows(2).map(|course_key_1_window| {
                    dbg!(course_key_1_window);
                    get_grid_paths(course_key_1_window[0], course_key_1_window[1], false).into_iter().map(|course_key_1| {
                        let course_key_1_coords = create_coords(&course_key_1);
                        //course_key_1_coords.windows(2).map(|course_key_2_window| {
                        //})
                        dbg!(course_key_1);
                        todo!()
                    })
                });
                todo!()
            }).min().unwrap();
            //min
            todo!();
            0
        }).sum::<usize>();
        //get_grid_paths(target_coords_door);
        //
        //let shortest_door = get_shortest_from_coords(target_coords_door);
        //let target_coords_pad_1: Vec<Coord> = create_coords(&shortest_door);
        //let shortest_pad_1 = get_shortest_from_coords(target_coords_pad_1);
        //let target_coords_pad_2: Vec<Coord> = create_coords(&shortest_pad_1);
        //let shortest_pad_2 = get_shortest_from_coords(target_coords_pad_2);
        //dbg!(shortest_pad_2.len());
        //dbg!(code_value);
        //print_course(&shortest_pad_2);
        shortest_path_value * code_value
    }).sum()
}



fn solve_2(input: &str) -> usize {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 1930);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 1206);
    }
}
