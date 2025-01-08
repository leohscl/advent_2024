use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
    dbg!(solve_2(input));
}


fn solve(input: &str) -> i64 {
    let secrets: Vec<i64> = input.split('\n').filter(|&s| s != "").map(|num_s| {
        num_s.parse().unwrap()
    }).collect();
    secrets.into_iter().map(|mut secret| {
        for _ in 0..2000 {
            secret = generate_new_secret(secret);
        }
        secret
    }).sum()
}

fn mix(secret: i64, op: i64) -> i64 {
    secret ^ op
}

fn prune(secret: i64) -> i64 {
    secret.rem_euclid(16777216)
}

fn generate_new_secret(secret: i64) -> i64 {
    let mult_1 = secret * 64;
    let mixed_1 = mix(secret, mult_1);
    let secret_1 = prune(mixed_1);
    let div_2 = secret_1 / 32;
    let mixed_2 = mix(secret_1, div_2);
    let secret_2 = prune(mixed_2);
    let mult_3 = secret_2 * 2048;
    let mixed_3 = mix(secret_2, mult_3);
    let secret_3 = prune(mixed_3);
    secret_3
}

fn solve_2(input: &str) -> i64 {
    let secrets: Vec<i64> = input.split('\n').filter(|&s| s != "").map(|num_s| {
        num_s.parse().unwrap()
    }).collect();
    //dbg!(&secrets);

    let all_hashes: Vec<HashMap<_, _>> = secrets.into_iter().map(|secret| {
        let vec_new_secrets = (0..2000).fold(vec![secret], |mut results, _| {
            let old_secret = *results.iter().last().unwrap();
            let new_secret = generate_new_secret(old_secret);
            results.push(new_secret);
            results
        });
        let changes: Vec<i64> = vec_new_secrets.windows(2).map(|window_secret| {
            let digits_newer = count_digits(window_secret[1]);
            let digits_older = count_digits(window_secret[0]);
            digits_newer - digits_older
        }).collect();

        let hash_sequences = changes.windows(4).enumerate().fold(HashMap::new(), |mut hash, (i_window, window_4_diff)| {
            let hash_id = (window_4_diff[0], window_4_diff[1], window_4_diff[2], window_4_diff[3]);
            hash.entry(hash_id).or_insert(count_digits(vec_new_secrets[i_window + 4]));
            hash
        });
        hash_sequences
    }).collect();

    let keys: HashSet<(i64, i64, i64, i64)> = all_hashes.iter().map(|hash| hash.keys()).flatten().cloned().collect();

    
    //let key_test = (-2,1,-1,3);
    //all_hashes.iter().for_each(|hash| {dbg!(hash.get(&key_test).unwrap_or(&0i64));});

    keys.into_iter().map(|key| {
        all_hashes.iter().map(|hash| hash.get(&key).unwrap_or(&0i64)).sum()
    }).max().unwrap()
}

fn count_digits(input: i64) -> i64 {
    input.to_string().chars().last().unwrap() as i64 - '0' as i64
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }
    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 37327623);
    }

    //#[test]
    //fn part_2_simple() {
    //    //let input = include_str!("../input_test.txt");
    //    let result = solve_2("123");
    //    assert_eq!(result, 6);
    //}

    //#[test]
    //fn part_2_subtest() {
    //    let result = solve_2("2");
    //    assert_eq!(result, 8);
    //}

    #[test]
    fn part_2() {
        let input = include_str!("../input_test_2.txt");
        let result = solve_2(input);
        assert_eq!(result, 23);
    }

}
