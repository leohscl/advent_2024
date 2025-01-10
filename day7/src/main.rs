fn main() {
    let input = include_str!("../input.txt");
    // dbg!(solve(input));
    dbg!(solve_2(input));
}

fn solve(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let mut part_iter = line.split(':');
            let target_num = part_iter.next().unwrap().parse::<i64>().unwrap();
            let all_numbers: Vec<_> = part_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect();
            if try_remaining_part_1(0, &all_numbers, target_num) {
                Some(target_num)
            } else {
                None
            }
        })
        .sum()
}

fn try_remaining_part_1(current_total: i64, numbers_left: &[i64], target_num: i64) -> bool {
    if numbers_left.len() == 0 {
        return current_total == target_num;
    }
    if current_total > target_num {
        return false;
    }
    let new_remaining = &numbers_left[1..];
    let first = numbers_left[0];
    try_remaining_part_1(current_total + first, new_remaining, target_num)
        || try_remaining_part_1(current_total * first, new_remaining, target_num)
}

fn solve_2(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let mut part_iter = line.split(':');
            let target_num = part_iter.next().unwrap().parse::<i64>().unwrap();
            let all_numbers: Vec<_> = part_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect();
            if try_remaining_part_2(0, &all_numbers, target_num) {
                Some(target_num)
            } else {
                None
            }
        })
        .sum()
}

fn try_remaining_part_2(current_total: i64, numbers_left: &[i64], target_num: i64) -> bool {
    if numbers_left.len() == 0 {
        return current_total == target_num;
    }
    if current_total > target_num {
        return false;
    }
    let new_remaining = &numbers_left[1..];
    let first = numbers_left[0];
    try_remaining_part_2(current_total + first, new_remaining, target_num)
        || try_remaining_part_2(current_total * first, new_remaining, target_num)
        || try_remaining_part_2(concat_num(current_total, first), new_remaining, target_num)
}

fn concat_num(first: i64, second: i64) -> i64 {
    (first.to_string() + &second.to_string())
        .parse::<i64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_part_1() {
        let input = include_str!("../input_test.txt");
        assert_eq!(solve(input), 3749);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../input_test.txt");
        assert_eq!(solve_2(input), 11387);
    }
}
