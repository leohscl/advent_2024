use std::collections::HashSet;


fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

fn convert_grid(input: &str) -> (Vec<char>, usize, usize) {
    let width = input.split('\n').next().unwrap().len();
    let height = input.chars().filter(|&c| c == '\n').count();
    let grid = input.chars().filter(|&c| c != '\n').collect();
    (grid, width, height)
}

fn solve(input: &str) -> usize {
    let (grid, width, _height) = convert_grid(input);
    let mut indices_mapped = HashSet::new();
    (0..grid.len()).filter_map(|i_grid| {
        if indices_mapped.contains(&i_grid) {
            return None
        }
        let mut region_boundaries = 0;
        let crop = grid[i_grid];
        let mut new_region: Vec<usize> = vec![i_grid];
        let mut active_nodes = vec![i_grid];
        indices_mapped.insert(i_grid);


        while active_nodes.len() != 0 {
            let mut new_nodes = vec![];
            active_nodes.into_iter().for_each(|index_new_node| {
                // search for neighbors
                region_boundaries += if index_new_node >= width {
                    let new_index = index_new_node - width;
                    try_add_candidate(new_index, &grid, &mut indices_mapped, &mut new_nodes, crop)
                } else {
                    1
                };
                region_boundaries += if index_new_node + width < grid.len() {
                    let new_index = index_new_node + width;
                    try_add_candidate(new_index, &grid, &mut indices_mapped, &mut new_nodes, crop)
                } else {
                    1
                };
                region_boundaries += if index_new_node % width != 0 {
                    let new_index = index_new_node - 1;
                    try_add_candidate(new_index, &grid, &mut indices_mapped, &mut new_nodes, crop)
                } else {
                    1
                };
                region_boundaries += if index_new_node % width != width - 1 {
                    let new_index = index_new_node + 1;
                    try_add_candidate(new_index, &grid, &mut indices_mapped, &mut new_nodes, crop)
                } else {
                    1
                };
            });
            active_nodes = new_nodes.clone();
            new_region.append(&mut new_nodes);
        }
        Some(new_region.len() * region_boundaries)
    }).sum()
}

fn try_add_candidate_has_borders(new_index: usize, grid: &[char], indices_mapped: &mut HashSet<usize>, new_nodes: &mut Vec<usize>, crop: char) -> bool {
    if grid[new_index] != crop {
        return true
    }
    if !indices_mapped.contains(&new_index) {
        indices_mapped.insert(new_index);
        new_nodes.push(new_index);
    }
    return false;
}

fn try_add_candidate(new_index: usize, grid: &[char], indices_mapped: &mut HashSet<usize>, new_nodes: &mut Vec<usize>, crop: char) -> usize {
    if grid[new_index] != crop {
        return 1
    }
    if !indices_mapped.contains(&new_index) {
        indices_mapped.insert(new_index);
        new_nodes.push(new_index);
    }
    return 0;
}

fn solve_2(input: &str) -> usize {
    let (grid, width, height) = convert_grid(input);
    let mut indices_mapped = HashSet::new();
    (0..grid.len()).filter_map(|i_grid| {
        if indices_mapped.contains(&i_grid) {
            return None
        }
        let mut up_borders = Vec::new();
        let mut down_borders = Vec::new();
        let mut left_borders = Vec::new();
        let mut right_borders = Vec::new();
        let crop = grid[i_grid];
        let mut new_region: Vec<usize> = vec![i_grid];
        let mut active_nodes = vec![i_grid];
        indices_mapped.insert(i_grid);


        while active_nodes.len() != 0 {
            let mut new_nodes = vec![];
            active_nodes.into_iter().for_each(|index_new_node| {
                // search for neighbors
                // UP
                if index_new_node >= width {
                    let new_index = index_new_node - width;
                    if try_add_candidate_has_borders(new_index, &grid, &mut indices_mapped, &mut new_nodes, crop) {
                        up_borders.push(index_new_node);
                    }
                } else {
                    up_borders.push(index_new_node);
                }
                // DOWN
                if index_new_node + width < grid.len() {
                    let new_index = index_new_node + width;
                    if try_add_candidate_has_borders(new_index, &grid, &mut indices_mapped, &mut new_nodes, crop) {
                        down_borders.push(index_new_node + width);
                    }
                } else {
                    down_borders.push(index_new_node + width);
                };
                // LEFT
                // reindex grid, to go vertically
                let (w, h) = (index_new_node % width, index_new_node / width);
                let left_border = h + w * height;
                //let left_border = (index_new_node / width) * super_grid_width + index_new_node % width;
                if index_new_node % width != 0 {
                    let new_index = index_new_node - 1;
                    if try_add_candidate_has_borders(new_index, &grid, &mut indices_mapped, &mut new_nodes, crop) {
                        left_borders.push(left_border);
                    }
                } else {
                    left_borders.push(left_border);
                };
                // RIGHT
                let right_border = h + (w + 1) * height;
                if index_new_node % width != width - 1 {
                    let new_index = index_new_node + 1;
                    if try_add_candidate_has_borders(new_index, &grid, &mut indices_mapped, &mut new_nodes, crop) {
                        right_borders.push(right_border);
                    }
                } else {
                    right_borders.push(right_border);
                };
            });
            active_nodes = new_nodes.clone();
            new_region.append(&mut new_nodes);
        }
        let cost_up = score_boudaries(up_borders, width, height + 1);
        let cost_down = score_boudaries(down_borders, width, height + 1);
        let cost_left = score_boudaries(left_borders, width, height + 1);
        let cost_right = score_boudaries(right_borders, width, height + 1);
        dbg!((cost_up, cost_down, cost_left, cost_right));
        Some(new_region.len() * (cost_up + cost_down + cost_left + cost_right))
    }).sum()
}

fn score_boudaries(mut boundaries: Vec<usize>, width: usize, height: usize) -> usize {
    boundaries.sort();
    let mut price = 0;
    let mut i_bound = 0;
    boundaries.iter().for_each(|bound| {
        if bound % width == 0 {
            price += 1
        } else {
            if !boundaries.contains(&(bound - 1)) {
                price += 1
            }
        }
        i_bound += 1;
    });
    price
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
    fn part_2_first() {
        let input = include_str!("../input_test_0.txt");
        let result = solve_2(input);
        assert_eq!(result, 80);
    }

    #[test]
    fn part_2_other() {
        let input = include_str!("../input_test_1.txt");
        let result = solve_2(input);
        assert_eq!(result, 436);
    }

    #[test]
    fn part_2_2() {
        let input = include_str!("../input_test_2.txt");
        let result = solve_2(input);
        assert_eq!(result, 236);
    }

    #[test]
    fn part_2_3() {
        let input = include_str!("../input_test_3.txt");
        let result = solve_2(input);
        assert_eq!(result, 368);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 1206);
    }
}
