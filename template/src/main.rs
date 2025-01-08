fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}


fn solve(input: &str) -> usize {
    todo!()
}



fn solve_2(input: &str) -> usize {
    todo!()
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
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 1206);
    }
}
