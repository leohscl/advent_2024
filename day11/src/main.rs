use itertools::Itertools;
use std::collections::HashMap;


fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

fn solve(input: &str) -> usize {
    let starting_stones = input.split_whitespace().filter(|&c| c != "\n").map(|n_str| n_str.parse::<u64>().unwrap()).collect_vec();
    // iterate on each stone 25 times
    starting_stones.into_iter().map(|stone| {
        dbg!(stone);
        compute_number_stones(stone, 25)
    }).sum()
}

fn compute_number_stones_memo(stone: u64, remaining_steps: usize, memoisator: &mut HashMap<(u64, usize), usize>) -> usize {
    let key = (stone, remaining_steps);
    if let Some(result) = memoisator.get(&key) {
        return *result;
    }
    if remaining_steps == 0 {
        return 1
    }
    let new_remaining_steps = remaining_steps - 1;
    let result;
    if stone == 0 {
        result = compute_number_stones_memo(1, new_remaining_steps, memoisator);
    } else {
        let str_stone = stone.to_string();
        let digit_count = str_stone.chars().count();
        if  digit_count % 2 == 0 {
            let middle_point = digit_count/2;
            let stone_1 = str_stone[0..middle_point].parse().unwrap();
            let stone_2 = str_stone[middle_point..str_stone.len()].parse().unwrap();
            result = compute_number_stones_memo(stone_1, new_remaining_steps, memoisator) + compute_number_stones_memo(stone_2, new_remaining_steps, memoisator);
        } else {
            result = compute_number_stones_memo(stone * 2024, new_remaining_steps, memoisator);
        }
    }
    memoisator.insert(key, result);
    result
}

fn compute_number_stones(stone: u64, remaining_steps: usize) -> usize {
    if remaining_steps == 0 {
        return 1
    }
    let new_remaining_steps = remaining_steps - 1;
    if stone == 0 {
        return compute_number_stones(1, new_remaining_steps)
    }
    let str_stone = stone.to_string();
    let digit_count = str_stone.chars().count();
    if  digit_count % 2 == 0 {
        let middle_point = digit_count/2;
        let stone_1 = str_stone[0..middle_point].parse().unwrap();
        let stone_2 = str_stone[middle_point..str_stone.len()].parse().unwrap();
        compute_number_stones(stone_1, new_remaining_steps) + compute_number_stones(stone_2, new_remaining_steps)
    } else {
        compute_number_stones(stone * 2024, new_remaining_steps)
    }
}

fn compute_stone_list(stone: u64, remaining_steps: usize) -> Vec<usize> {
    todo!()
}

fn solve_2(input: &str) -> usize {
    let starting_stones = input.split_whitespace().filter(|&c| c != "\n").map(|n_str| n_str.parse::<u64>().unwrap()).collect_vec();
    starting_stones.into_iter().map(|stone| {
        compute_number_stones_memo(stone, 75, &mut HashMap::new())
    }).sum()
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 55312);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 81);
    }
}
