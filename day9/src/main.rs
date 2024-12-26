use std::collections::LinkedList;

use itertools::Itertools;


fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

fn solve(input: &str) -> usize {
    let digits = input.chars().filter(|&c| c != '\n').map(|c| {c.to_digit(10).unwrap() as usize}).collect_vec();
    let mut result_vec = vec![];
    let mut index_end = digits.len() - 1;
    let mut remaining_end = digits[index_end];
    let mut index_vec = 0;
    let mut space_remaining = true;
    while space_remaining {
        let parsed = digits[index_vec];
        if index_vec % 2 == 0 {
            let id_file = index_vec / 2;
            if index_vec == index_end {
                result_vec.append(&mut vec![id_file; remaining_end]);
                space_remaining = false;
            } else {
                result_vec.append(&mut vec![id_file; parsed]);
            }
        } else {
            (index_end, remaining_end, space_remaining) = fill_missing(&mut result_vec, &digits, index_end, remaining_end, parsed, index_vec);
        }
        index_vec += 1;
    }
    dbg!(&result_vec);
    result_vec.into_iter().enumerate().fold(0, |acc, (i_num, num)| {
        acc + i_num * num
    })
}

fn fill_missing(result_vec: &mut Vec<usize>, digits: &[usize], index_end: usize, remaining_end: usize, amount: usize, index_vec: usize) -> (usize, usize, bool) {
    let id_file = index_end / 2;
    if remaining_end >= amount {
        result_vec.append(&mut vec![id_file; amount]);
        (index_end, remaining_end - amount, true)
    } else {
        result_vec.append(&mut vec![id_file; remaining_end]);
        let new_index_end = index_end - 2;
        if new_index_end < index_vec {
            return (0, 0, false)
        }
        fill_missing(result_vec, digits, new_index_end, digits[new_index_end], amount - remaining_end, index_vec)
    }
}

enum Element {
    Space(usize),
    File(usize, usize),
}

fn insert_at(l: &mut LinkedList<usize>, idx: usize, val: usize) {
    let mut tail = l.split_off(idx);
    l.push_back(val);
    l.append(&mut tail);
}

//fn main(){
//    let mut l = LinkedList::from([1, 2, 3]);
//
//    insert_at(&mut l, 2, 4);
//
//    let res: Vec<u8> = l.into_iter().collect();
//    assert_eq!(res, vec![1, 2, 4, 3]);
//}


fn solve_2(input: &str) -> usize {
    let digits = input.chars().filter(|&c| c != '\n').map(|c| {c.to_digit(10).unwrap() as usize}).collect_vec();
    let mut result_vec = vec![];
    let mut indexes_vec_spaces = vec![];
    let mut indexes_vec_files = vec![];
    for index_vec in 0..digits.len() {
        let parsed = digits[index_vec];
        if index_vec % 2 == 0 {
            indexes_vec_files.push((result_vec.len(), parsed));
            let id_file = index_vec / 2;
            result_vec.append(&mut vec![id_file; parsed]);
        } else {
            indexes_vec_spaces.push((result_vec.len(), parsed));
            result_vec.append(&mut vec![0; parsed]);
        }
    }
    for (index_file, size) in indexes_vec_files.iter().rev() {
        if let Some(pos) = indexes_vec_spaces.iter().position(|(_index_space, space)| {
            space >= size
        }) {
            let (index_space, space) = indexes_vec_spaces[pos];
            if index_space < *index_file {
                // move
                for i_size in 0..*size {
                    result_vec[index_space + i_size] = result_vec[index_file + i_size];
                    result_vec[index_file + i_size] = 0;
                }
                // update space
                let new_space = space - size;
                let new_index = index_space + size;
                indexes_vec_spaces[pos] = (new_index, new_space);
            }
        }
    }
    dbg!(&result_vec);
    result_vec.into_iter().enumerate().fold(0, |acc, (i_num, num)| {
        acc + i_num * num
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let input = include_str!("../input_test_0.txt");
        let result = solve(input);
        assert_eq!(result, 60);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 1928);
    }

    #[test]
    fn part_2_mini() {
        let input = include_str!("../input_test_1.txt");
        let result = solve_2(input);
        assert_eq!(result, 2858);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 2858);
    }
}
