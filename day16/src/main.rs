use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

enum Element {
    Empty,
    Wall,
}

fn convert_grid(input: &str) -> (HashMap<(i64, i64), Element>, (i64, i64), (i64, i64), i64, i64) {
    let width = input.split('\n').next().unwrap().len() as i64;
    let height = input.chars().filter(|&c| c == '\n').count() as i64;
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = input.chars().filter(|&c| c != '\n').enumerate().fold(HashMap::new(), |mut hash, (i, c)| {
        let (x, y) = ((i as i64 % width), (i as i64 / width));
        let element = match c {
            '.' => Element::Empty,
            '#' => Element::Wall,
            'S' => {
                start = (x, y);
                Element::Empty
            },
            'E' => {
                end = (x, y);
                Element::Empty
            },
            _ => panic!()
        };
        hash.insert((x,y), element);
        hash
    });
    (grid, start, end, width, height)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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

fn solve(input: &str) -> usize {
    let (grid, start, end, _width, _height) = convert_grid(input);
    let mut hash_visited = HashMap::new();
    let all_paths = build_path(vec![Direction::Right], start, end, &mut hash_visited, &grid);
    all_paths.into_iter().map(|path| evaluate_path(&path)).min().unwrap()
}

fn evaluate_path(path: &Path) -> usize {
    let steps = path.len() - 1;
    let turns_count = path.windows(2).filter(|v| v[0] != v[1]).count();
    steps + turns_count * 1000
}

type Path = Vec<Direction>;


fn build_path(current_path: Path, start: (i64, i64), end: (i64, i64), visited_score: &mut HashMap<(i64, i64, Direction), usize>, grid: &HashMap<(i64, i64), Element>) -> Vec<Path> {
    let score = evaluate_path(&current_path);
    let min_scores = DIRECTIONS.iter().filter_map(|&dir| {
        let hash_id = (start.0, start.1, dir);
        visited_score.get(&hash_id)
    }).min().unwrap_or(&(usize::MAX - 1000));
    if min_scores + 1000 < score {
        return vec![];
    }
    let previous_direction = current_path[current_path.len() - 1];
    let hash_id = (start.0, start.1, previous_direction);
    match visited_score.get(&hash_id) {
        None => {
            visited_score.entry(hash_id).or_insert(score);
        },
        Some(previous_score) => {
            if score <= *previous_score {
                visited_score.entry(hash_id).and_modify(|e| *e = score);
            } else {
                return vec![];
            }
        }
    }
    if start == end {
        return vec![current_path];
    }
    DIRECTIONS.iter().fold(vec![], |mut all_paths, direction| {
        let next_pos = direction.compute_next_pos(start);
        match grid.get(&next_pos) {
            Some(Element::Empty) => {
                let mut new_path = current_path.clone();
                new_path.push(*direction);
                let new_paths = build_path(new_path, next_pos, end, visited_score, grid);
                all_paths.extend(new_paths);
                all_paths
            }
            _ => all_paths,
        }
    })
}



fn solve_2(input: &str) -> usize {
    let (grid, start, end, width, height) = convert_grid(input);
    let mut hash_visited = HashMap::new();
    let all_paths = build_path(vec![Direction::Right], start, end, &mut hash_visited, &grid);
    let min = all_paths.iter().map(|path| evaluate_path(&path)).min().unwrap();
    let mut winning_tiles = HashSet::new();
    winning_tiles.insert(start);
    all_paths.into_iter().filter(|path| evaluate_path(path) == min).for_each(|path| {
        let mut current_tile = start;
        path.into_iter().skip(1).for_each(|dir| {
            current_tile = dir.compute_next_pos(current_tile);
            winning_tiles.insert(current_tile);
        })
    });
    print_winning(&winning_tiles, &grid, width, height);
    winning_tiles.into_iter().count()
}


fn print_winning(winning: &HashSet<(i64, i64)>, grid: &HashMap<(i64, i64), Element>, width: i64, height: i64) {
    use Element::*;
    (0..height).for_each(|h| {
        (0..width).for_each(|w| {
            let c = if winning.contains(&(w, h)) {
                'O'
            } else {
                match grid.get(&(w, h)).unwrap() {
                    Wall => '#',
                    Empty => '.',
                }
            };
            print!("{}", c);
        });
        println!()
    });
    println!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 7036);
    }

    #[test]
    fn part_1_2() {
        let input = include_str!("../input_test_2.txt");
        let result = solve(input);
        assert_eq!(result, 11048);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 45);
    }

    #[test]
    fn part_2_2() {
        let input = include_str!("../input_test_2.txt");
        let result = solve_2(input);
        assert_eq!(result, 64);
    }
}
