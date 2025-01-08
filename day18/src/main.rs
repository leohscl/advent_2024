use std::collections::HashMap;

enum Element {
    Empty,
    Byte,
}

type Coord = (i64, i64);
type Grid = HashMap<Coord, Element>;
type Path = Vec<Direction>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

impl Direction {
    fn compute_next_pos(&self, pos: (i64, i64)) -> (i64, i64) {
        use Direction::*;
        match self {
            Up => (pos.0, pos.1 - 1),
            Down => (pos.0, pos.1 + 1),
            Left => (pos.0 - 1, pos.1),
            Right => (pos.0 + 1, pos.1),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
    dbg!(solve_2(input));
}


fn solve(input: &str) -> usize {
    let width = 71;
    let height = 71;
    let parsed = parse_input(input);
    let grid = make_grid(input, width, height, &parsed[0..1024]);
    let best_path = find_path(Vec::new(), &grid, width, height, (0,0), (70, 70), &mut HashMap::new()).unwrap();
    best_path.len()
}

fn parse_input(input: &str) -> Vec<Coord> {
    input.split('\n').filter(|&s| s != "").map(|line| {
        let mut coord_iter = line.split(',').map(|num_s| num_s.parse().unwrap());
        let coords = (coord_iter.next().unwrap(), coord_iter.next().unwrap());
        coords
    }).collect()
}

fn make_grid(input: &str, width: i64, height: i64, bytes: &[Coord]) -> Grid {
    let mut grid = HashMap::new();
    (0..width).for_each(|h| {
        (0..height).for_each(|w| {
            grid.insert((h, w), Element::Empty);
        });
    });
    bytes.iter().for_each(|coord_byte| {
        grid.insert(*coord_byte, Element::Byte);
    });
    grid
}

fn find_path(current_path: Path, grid: &Grid, width: i64, height: i64, start: Coord, end: Coord, visited_score: &mut HashMap<(i64, i64), usize>) -> Option<Path> {
    let score = current_path.len();
    let hash_id = (start.0, start.1);
    match visited_score.get(&hash_id) {
        None => {
            visited_score.entry(hash_id).or_insert(score);
        },
        Some(previous_score) => {
            if score < *previous_score {
                visited_score.entry(hash_id).and_modify(|e| *e = score);
            } else {
                return None;
            }
        }
    }
    if start == end {
        return Some(current_path);
    }
    let paths = DIRECTIONS.iter().fold(vec![], |mut all_paths, direction| {
        let next_pos = direction.compute_next_pos(start);
        match grid.get(&next_pos) {
            Some(Element::Empty) => {
                let mut new_path = current_path.clone();
                new_path.push(*direction);
                if let Some(new_path) = find_path(new_path, grid, width, height, next_pos, end, visited_score) {
                    all_paths.push(new_path);
                };
                all_paths
            }
            _ => all_paths,
        }
    });
    paths.into_iter().min_by(|p1, p2| p1.len().cmp(&p2.len())) 
}


fn print_grid_with_distances(visited_score: &HashMap<(i64, i64), usize>, grid: &Grid, width: i64, height: i64) {
    use Element::*;
    (0..height).for_each(|h| {
        (0..width).for_each(|w| {
            let c = match (visited_score.get(&(w, h)), grid.get(&(w, h)).unwrap()) {
                (Some(i), _) => i.to_string().chars().next().unwrap(),
                (_, Byte) => '#',
                (_, Empty) => '.',
            };
            print!("{}", c);
        });
        println!()
    });
    println!()
}

fn solve_2(input: &str) -> Coord {
    let width = 71;
    let height = 71;
    let parsed = parse_input(input);
    let coord_i = (0..).find(|&fallen_bytes| {
        dbg!(fallen_bytes);
        let grid = make_grid(input, width, height, &parsed[0..=fallen_bytes]);
        find_path(Vec::new(), &grid, width, height, (0,0), (70, 70), &mut HashMap::new()).is_none()
    }).unwrap();
    parsed[coord_i]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let width = 7;
        let height = 7;
        let parsed = parse_input(input);
        let grid = make_grid(input, width, height, &parsed[0..12]);
        let mut visited_score = HashMap::new();
        let best_path = find_path(Vec::new(), &grid, width, height, (0,0), (6, 6), &mut visited_score).unwrap();
        print_grid_with_distances(&visited_score, &grid, width, height);
        assert_eq!(best_path.len(), 22);
        
    }

    //#[test]
    //fn part_2_1() {
    //    let input = include_str!("../input_test.txt");
    //    let width = 7;
    //    let height = 7;
    //    let parsed = parse_input(input);
    //    let mut visited_score = HashMap::new();
    //    let grid = make_grid(input, width, height, &parsed[0..21]);
    //    find_path(Vec::new(), &grid, width, height, (0,0), (6, 6), &mut visited_score);
    //    print_grid_with_distances(&visited_score, &grid, width, height);
    //    panic!()
    //    //assert_eq!(parsed[coord_i], (6, 1));
    //}

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let width = 7;
        let height = 7;
        let parsed = parse_input(input);
        let coord_i = (0..).find(|&fallen_bytes| {
            dbg!(fallen_bytes);
            let mut visited_score = HashMap::new();
            let grid = make_grid(input, width, height, &parsed[0..=fallen_bytes]);
            let best_path = find_path(Vec::new(), &grid, width, height, (0,0), (6, 6), &mut visited_score);
            best_path.is_none()
        }).unwrap();
        assert_eq!(parsed[coord_i], (6, 1));
    }

}
