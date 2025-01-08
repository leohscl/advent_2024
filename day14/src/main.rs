use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

#[cfg(test)]
const WIDTH: i32 = 11;
#[cfg(test)]
const HEIGHT: i32 = 7;
#[cfg(not(test))]
const WIDTH: i32 = 101;
#[cfg(not(test))]
const HEIGHT: i32 = 103;

struct Robot {
    start_x: i32,
    start_y: i32,
    v_x: i32,
    v_y: i32,
}

impl Robot {
    fn position_after(&self, time: i32) -> (i32, i32) {
        let pos_x = (self.start_x + time * self.v_x).rem_euclid(WIDTH);
        let pos_y = (self.start_y + time * self.v_y).rem_euclid(HEIGHT);
        (pos_x, pos_y)
}
fn cycle_len(&self) -> usize {
        (1..).position(|time| self.position_after(time) == (self.start_x, self.start_y)).unwrap()
    }
}

fn make_robot(line: &str) -> Robot {
    let mut iter_parts = line.split([',', '=', ' ']);
    iter_parts.next();
    let start_x = iter_parts.next().unwrap().parse().unwrap();
    let start_y = iter_parts.next().unwrap().parse().unwrap();
    iter_parts.next();
    let v_x = iter_parts.next().unwrap().parse().unwrap();
    let v_y = iter_parts.next().unwrap().parse().unwrap();
    Robot {start_x, start_y, v_x, v_y}
}

fn solve(input: &str) -> usize {
    let robots = input.split('\n').filter(|&s| s != "").map(|line| {
        make_robot(line)
    }).collect_vec();
    let robots_position: Vec<(i32, i32)> = robots.iter().map(|robot| robot.position_after(100)).collect();
    let quadrant_width = WIDTH / 2;
    let quadrant_height = HEIGHT / 2;
    (0..4).map(|i_quadrant| {
        let quadrant_w_min = (quadrant_width + 1) * (i_quadrant % 2);
        let quadrant_w_max = quadrant_w_min + quadrant_width - 1;
        let quadrant_h_min = (quadrant_height + 1) * (i_quadrant / 2);
        let quadrant_h_max = quadrant_h_min + quadrant_height - 1;
        dbg!((quadrant_h_min, quadrant_h_max, quadrant_w_min, quadrant_w_max));
        robots_position.iter().filter(|pos| quadrant_w_min <= pos.0 && pos.0 <= quadrant_w_max && quadrant_h_min <= pos.1 && pos.1 <= quadrant_h_max).count()
    }).fold(1, |acc, robot_count| acc * robot_count)
}



fn solve_2(input: &str) -> usize {
    let robots = input.split('\n').filter(|&s| s != "").map(|line| {
        let robot = make_robot(line);
        let cycle_len = robot.cycle_len();
        dbg!(cycle_len);
        robot
    }).collect_vec();
    let avg_robots_per_line = robots.len() as f64 / HEIGHT as f64;
    let tree_time = (0..10402).position(|time_test| {
        dbg!(time_test);
        let lines_count: HashMap<i32, usize> = robots.iter().fold(HashMap::new(), |mut hash_lines, robot| {
            let position = robot.position_after(time_test);
            hash_lines.entry(position.1).and_modify(|e| *e += 1).or_insert(1);
            hash_lines
        });
        if lines_count.values().any(|count| *count as f64 >= 4f64 * avg_robots_per_line) {
            print_arrangement(&robots, time_test as i32);
        }
        false
    }).unwrap();
    print_arrangement(&robots, tree_time as i32);
    tree_time
}

fn print_arrangement(robots: &Vec<Robot>, time: i32) {
    let robots_position: HashSet<(i32, i32)> = robots.iter().map(|robot| robot.position_after(time)).collect();
    (0..HEIGHT).for_each(|h| {
        (0..WIDTH).for_each(|l| {
            if robots_position.contains(&(h, l)) {
                print!("x")
            } else {
                print!(" ")
            }
        });
        println!();
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 12);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 1206);
    }
}
