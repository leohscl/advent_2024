use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

type Coords = (i32, i32);

fn convert_grid(input: &str) -> (HashMap<char, Vec<Coords>>, i32, i32) {
    let width = input.split('\n').next().unwrap().len();
    let height = input.chars().filter(|&c| c == '\n').count();
    let mut hash_coords = HashMap::new();
    input.chars().filter(|&c| c != '\n').enumerate().for_each(|(i, c)| 
        match c {
            '.' => (),
            _ => {
                let h_i = (i / width) as i32;
                let w_i = (i % width) as i32;
                hash_coords.entry(c).or_insert(vec![]).push((h_i, w_i));
            }
        }
    );
    (hash_coords, width as i32, height as i32)
}

fn compute_candidates(start_coord: Coords, h_diff: i32, w_diff: i32) -> Vec<Coords> {
    let w_diff_cand_1 = -2 * w_diff;
    let w_diff_cand_2 = w_diff;
    let h_diff_cand_1 = -2 * h_diff;
    let h_diff_cand_2 = h_diff;
    vec![(start_coord.0 + h_diff_cand_1, start_coord.1 + w_diff_cand_1), (start_coord.0 + h_diff_cand_2, start_coord.1 + w_diff_cand_2)]
}

fn on_grid(coords: &Coords, width: i32, height: i32) -> bool {
    let h = coords.0;
    let w = coords.1;
    0 <= h && h < height && 0 <= w && w < width
}

fn compute_antinodes(antenna_list: &[Coords], antinodes: &mut HashSet<Coords>, width: i32, height: i32) {
    for i in 0..antenna_list.len() {
        for j in i+1..antenna_list.len() {
            let c_i = antenna_list[i];
            let c_j = antenna_list[j];
            let h_diff = c_j.0 - c_i.0;
            let w_diff = c_j.1 - c_i.1;
            let elements = compute_candidates(c_j, h_diff, w_diff);
            elements.into_iter().filter(|e| on_grid(e, width, height)).for_each(|e| {dbg!(e);antinodes.insert(e);});
        }
    }
}

fn solve(input: &str) -> usize {
    let (hash_coords, width, height) = convert_grid(input);
    // stores all the antinodes
    let mut antinodes = HashSet::new();
    hash_coords.values().for_each(|antennas| compute_antinodes(&antennas, &mut antinodes, width, height));
    antinodes.iter().count()
}

fn compute_antinodes_with_resonance(antenna_list: &[Coords], antinodes: &mut HashSet<Coords>, width: i32, height: i32) {
    for i in 0..antenna_list.len() {
        for j in i+1..antenna_list.len() {
            let c_i = antenna_list[i];
            let c_j = antenna_list[j];
            let h_diff = c_j.0 - c_i.0;
            let w_diff = c_j.1 - c_i.1;
            (0..).map(|i_test| (c_i.0 + i_test * h_diff, c_i.1 + i_test * w_diff)).take_while(|e| on_grid(e, width, height)).for_each(|e| {dbg!(e);antinodes.insert(e);});
            (0..).map(|i_test| (c_i.0 - i_test * h_diff, c_i.1 - i_test * w_diff)).take_while(|e| on_grid(e, width, height)).for_each(|e| {dbg!(e);antinodes.insert(e);});
        }
    }
}

fn solve_2(input: &str) -> usize {
    let (hash_coords, width, height) = convert_grid(input);
    // stores all the antinodes
    let mut antinodes = HashSet::new();
    hash_coords.values().for_each(|antennas| compute_antinodes_with_resonance(&antennas, &mut antinodes, width, height));
    antinodes.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_small() {
        let input = include_str!("../input_test_0.txt");
        let result = solve(input);
        assert_eq!(result, 12);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 14);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 6);
    }
}
