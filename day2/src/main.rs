fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve_2(input));
}

fn solve(input: &str) -> i32 {
    input.split('\n').filter(|&l| l != "").map(|line| {
        let levels: Vec<_> = line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();
        let asc = levels[0] < levels[1];
        let valid = levels.windows(2).fold(true, |acc, pair| {
            let diff = (pair[0] - pair[1]).abs();
            acc && (asc ^ (pair[1] < pair[0])) && diff <=3 && diff != 0 
        });
        if valid {
            1
        } else {
            0
        }
    }).sum()
}

fn solve_2(input: &str) -> i32 {
    input.split('\n').filter(|&l| l != "").map(|line| {
        let levels: Vec<_> = line.split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();
        let mut levels_candidate: Vec<_> = levels.clone().into_iter().skip(1).collect();
        let valid = (0..levels.len()).fold(false, |acc_i, index_lev| {
            if index_lev >= 1 {
                levels_candidate[index_lev - 1] = levels[index_lev - 1];
            }
            // dbg!(index_lev);
            // dbg!(&levels_candidate);
            let asc = levels_candidate[0] < levels_candidate[1];
            let valid_j = levels_candidate.windows(2).fold(true, |acc_j, pair| {
                let diff = (pair[0] - pair[1]).abs();
                acc_j && (asc ^ (pair[1] < pair[0])) && diff <=3 && diff != 0 
            });
            dbg!(valid_j);
            acc_i || valid_j
        });
        if valid {
            1
        } else {
            0
        }
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 4);
    }
}
