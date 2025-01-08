use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
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
    let (grid, start, end, width, height) = convert_grid(input);
    let mut hash_visited = HashMap::new();
    fill_distances(vec![], start, end, &mut hash_visited, &grid);
    dbg!(&hash_visited.get(&end));
    print_grid(&grid, width, height);
    let cheats = enumerate_cheats(&hash_visited, width, height);
    cheats.into_iter().map(|(k, v)| {
        if k >= 100 {
            v
        } else {
            0
        }
    }).sum()
}


type Path = Vec<Direction>;

fn enumerate_cheats_2(visited_score: &HashMap<(i64, i64), usize>, width: i64, height: i64) -> HashMap<usize, usize> {
    let mut cheats = HashMap::new();
    let mut offsets = HashSet::new();
    for h in -20..=20i64 {
        for w in -20..=20i64 {
            if h.abs() + w.abs() <= 20 {
                let offset = (w, h);
                offsets.insert(offset);
            }
        }
    }

    for h in 0..height {
        for w in 0..width {
            let tile = (w, h);
            let tile_score = if let Some(score) = visited_score.get(&tile) {
                score
            } else {
                continue;
            };
            for offset in &offsets {
                let tile_cheated = (tile.0 + offset.0, tile.1 + offset.1);
                let tile_cheated_score = if let Some(score) = visited_score.get(&tile_cheated) {
                    score.checked_sub(offset.0.abs() as usize + offset.1.abs() as usize).unwrap_or(0)
                } else {
                    continue;
                };
                if tile_cheated_score > *tile_score {
                    let cheat_value = tile_cheated_score - tile_score;
                    cheats.entry(cheat_value).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
    }
    cheats
}

fn enumerate_cheats(visited_score: &HashMap<(i64, i64), usize>, width: i64, height: i64) -> HashMap<usize, usize> {
    let mut cheats = HashMap::new();
    let offsets = [(-2, 0), (2, 0), (0, 2), (0, -2)];

    for h in 0..height {
        for w in 0..width {
            let tile = (w, h);
            let tile_score = if let Some(score) = visited_score.get(&tile) {
                score
            } else {
                continue;
            };
            for offset in offsets {
                let tile_cheated = (tile.0 + offset.0, tile.1 + offset.1);
                let tile_cheated_score = if let Some(score) = visited_score.get(&tile_cheated) {
                    score
                } else {
                    continue;
                };
                if *tile_cheated_score > tile_score + 2 {
                    let cheat_value = tile_cheated_score - tile_score - 2;
                    cheats.entry(cheat_value).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
    }
    cheats
}

fn fill_distances(current_path: Path, start: (i64, i64), end: (i64, i64), visited_score: &mut HashMap<(i64, i64), usize>, grid: &HashMap<(i64, i64), Element>) {
    let score = current_path.len();
    let hash_id = (start.0, start.1);
    let opt_min_score = visited_score.get_mut(&hash_id);
    if let Some(min) = opt_min_score {
        if *min <= score {
            return;
        } else {
            *min = score
        }
    } else {
        visited_score.insert(hash_id, score);
    }
    if start == end {
        return;
    }
    DIRECTIONS.iter().for_each(|direction| {
        let next_pos = direction.compute_next_pos(start);
        match grid.get(&next_pos) {
            Some(Element::Empty) => {
                let mut new_path = current_path.clone();
                new_path.push(*direction);
                fill_distances(new_path, next_pos, end, visited_score, grid);
            }
            _ => (),
        }
    })
}



fn solve_2(input: &str) -> usize {
    let (grid, start, end, width, height) = convert_grid(input);
    let mut hash_visited = HashMap::new();
    fill_distances(vec![], start, end, &mut hash_visited, &grid);
    dbg!(&hash_visited.get(&end));
    print_grid(&grid, width, height);
    let cheats = enumerate_cheats_2(&hash_visited, width, height);
    cheats.into_iter().map(|(k, v)| {
        if k >= 100 {
            v
        } else {
            0
        }
    }).sum()
    //let cheats_filtered: HashMap<_, _> = cheats.into_iter().filter(|&(k, _v)| k >= 100).collect();
    //dbg!(cheats_filtered);
}


fn print_grid(grid: &HashMap<(i64, i64), Element>, width: i64, height: i64) {
    use Element::*;
    (0..height).for_each(|h| {
        (0..width).for_each(|w| {
            let c = match grid.get(&(w, h)).unwrap() {
                Wall => '#',
                Empty => '.',
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
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 45);
    }

}
