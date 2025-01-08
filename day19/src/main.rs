use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
    dbg!(solve_2(input));
}

type Towel = String;


fn parse_input(input: &str) -> (Vec<String>, Vec<Towel>) {
    let mut iter_parts = input.split("\n\n");
    let towels_str = iter_parts.next().unwrap();
    let patterns_str = iter_parts.next().unwrap();
    let mut towels: Vec<String> = towels_str.split(", ").map(|towel_str| towel_str.to_string()).collect();
    towels.sort_by(|t1, t2| t2.len().cmp(&t1.len()));
    let patterns = patterns_str.split('\n').filter(|&s| s != "").map(|line| line.to_string()).collect();
    (patterns, towels)
}

fn solve(input: &str) -> usize {
    let (patterns, towels) = parse_input(input);
    let mut visited_pattern = HashMap::new();
    patterns.into_iter().filter(|pattern| {
        solve_pattern(pattern, &towels, &mut visited_pattern)
    }).count()
}

fn solve_pattern_2(pattern: &str, towels: &Vec<Towel>, visited_pattern: &mut HashMap<String, usize>) -> usize {
    if let Some(ret) = visited_pattern.get(pattern) {
        return *ret;
    }
    let pattern_ways = if pattern == "" {
        1
    } else {
        towels.iter().map(|towel| {
            if pattern.starts_with(towel) {
                solve_pattern_2(&pattern[towel.len()..], towels, visited_pattern)
            } else {
                0
            }
        }).sum()
    };
    visited_pattern.insert(pattern.to_string(), pattern_ways);
    pattern_ways
}

fn solve_pattern(pattern: &str, towels: &Vec<Towel>, visited_pattern: &mut HashMap<String, bool>) -> bool {
    if let Some(ret) = visited_pattern.get(pattern) {
        return *ret;
    }
    let pattern_doable = if pattern == "" {
        true
    } else {
        towels.iter().any(|towel| {
            if pattern.starts_with(towel) {
                solve_pattern(&pattern[towel.len()..], towels, visited_pattern)
            } else {
                false
            }
        })
    };
    visited_pattern.insert(pattern.to_string(), pattern_doable);
    pattern_doable
}



fn solve_2(input: &str) -> usize {
    let (patterns, towels) = parse_input(input);
    let mut visited_pattern = HashMap::new();
    patterns.into_iter().map(|pattern| {
        solve_pattern_2(&pattern, &towels, &mut visited_pattern)
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 16);
    }
}
