use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}
type Rule = (u32, u32);
type Rules = HashMap<u32, Vec<u32>>;

fn solve(input: &str) -> u32 {
    let mut iter_parts = input.split("\n\n");
    let str_rules = iter_parts.next().unwrap();
    let str_updates = iter_parts.next().unwrap();
    let vec_rules = str_rules.split('\n').filter(|&s| s != "").map(|s| {
        let mut rules_num = s.split('|').map(|str_n| {
            str_n.parse::<u32>().unwrap()
        });
        (rules_num.next().unwrap(), rules_num.next().unwrap())
    }).collect();
    let rules = create_rules_hash(vec_rules);
    str_updates.split('\n').filter(|&s| s != "").filter_map(|s| {
        let update: Vec<u32> = s.split(',').map(|str_n| {
            str_n.parse::<u32>().unwrap()
        }).collect();
        if check_update(&update, &rules) {
            Some(get_middle_update(update))
        } else {
            None
        }
    }).sum()
}

fn solve_2(input: &str) -> u32 {
    let mut iter_parts = input.split("\n\n");
    let str_rules = iter_parts.next().unwrap();
    let str_updates = iter_parts.next().unwrap();
    let vec_rules = str_rules.split('\n').filter(|&s| s != "").map(|s| {
        let mut rules_num = s.split('|').map(|str_n| {
            str_n.parse::<u32>().unwrap()
        });
        (rules_num.next().unwrap(), rules_num.next().unwrap())
    }).collect();
    let rules = create_rules_hash(vec_rules);
    str_updates.split('\n').filter(|&s| s != "").filter_map(|s| {
        let mut update: Vec<u32> = s.split(',').map(|str_n| {
            str_n.parse::<u32>().unwrap()
        }).collect();
        if check_update(&update, &rules) {
            None
        } else {
            while !check_update(&update, &rules) {
                sort_1_step(&mut update, &rules);
            }
            Some(get_middle_update(update))
        }
    }).sum()
}

fn sort_1_step(update: &mut Vec<u32>, rules: &Rules) {
    (0..update.len() - 1).for_each(|i_index| {
        let cmp_first = update[i_index];
        match rules.get(&update[i_index]) {
            None => (),
            Some(forbidden_values) => {
                let vec_values_after = &update[i_index+1..];
                let opt_pos = vec_values_after.iter().position(|value_to_check| forbidden_values.contains(value_to_check));
                if let Some(pos) = opt_pos {
                    update[i_index] = vec_values_after[pos];
                    update[i_index + 1 + pos] = cmp_first;
                }
            }
        }
    })
}

fn check_update(update: &[u32], rules: &Rules) -> bool {
    (0..update.len() - 1).all(|i_index| {
        match rules.get(&update[i_index]) {
            None => true,
            Some(forbidden_values) => {
                let vec_values_after = &update[i_index+1..];
                vec_values_after.iter().all(|value_to_check| !forbidden_values.contains(value_to_check))
            }
        }
    })
}

fn get_middle_update(update: Vec<u32>) -> u32 {
    update[update.len() / 2]
}


fn create_rules_hash(vec_rules: Vec<Rule>) -> Rules {
    let mut hash_rules = HashMap::new();
    vec_rules.iter().for_each(|rule| {
        hash_rules.entry(rule.1).or_insert(Vec::new()).push(rule.0);
    });
    hash_rules
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 143);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 123);
    }
}
