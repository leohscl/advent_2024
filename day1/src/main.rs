use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

fn solve(input: &str) -> i32 {
    let (mut left_list, mut right_list) = input.split('\n').filter(|&l| l != "").fold((vec![], vec![]), |mut acc, l| {
        dbg!(&l);
        let mut iter_elt = l.split_whitespace().map(|e| e.parse::<i32>().unwrap());
        acc.0.push(iter_elt.next().unwrap());
        acc.1.push(iter_elt.next().unwrap());
        acc
    });
    left_list.sort();
    right_list.sort();
    left_list.iter().zip(right_list.iter()).map(|(left, right)| (left - right).abs()).sum()
}

fn solve_2(input: &str) -> i32 {
    let (mut left_list, mut right_list) = input.split('\n').filter(|&l| l != "").fold((vec![], vec![]), |mut acc, l| {
        dbg!(&l);
        let mut iter_elt = l.split_whitespace().map(|e| e.parse::<i32>().unwrap());
        acc.0.push(iter_elt.next().unwrap());
        acc.1.push(iter_elt.next().unwrap());
        acc
    });
    let mut hash_right_list = HashMap::new();
    right_list.iter().for_each(|elt| {
        hash_right_list.entry(elt).and_modify(|count| *count += 1).or_insert(1);
    });
    left_list.iter().map(|elt| {
        elt * hash_right_list.get(&elt).unwrap_or(&0)
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 31);
    }
}
