use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
    dbg!(solve_2(input));
}


    
fn get_all_connections(input: &str) -> HashMap<String, Vec<String>> {
    input.split('\n').filter(|&s| s != "").fold(HashMap::new(), |mut hash, line| {
        let vec_result: Vec<String> = line.split('-').map(|s| s.to_string()).collect();
        let first = vec_result[0].clone();
        let second = vec_result[1].clone();
        hash.entry(first.clone()).or_insert(vec![]).push(second.clone());
        hash.entry(second).or_insert(vec![]).push(first);
        hash
    })
}

fn solve(input: &str) -> usize {
    let connections = get_all_connections(input);
    let sets_3: HashSet<Vec<String>> = connections.keys().fold(HashSet::new(), |mut sets, first_computer| {
        let neighbors_1 = connections.get(first_computer).unwrap();
        for second_computer in neighbors_1.iter() {
            let neighbors_2 = connections.get(second_computer).unwrap();
            for third_computer in neighbors_2 {
                if neighbors_1.contains(third_computer) {
                    let mut vec_index = vec![first_computer, second_computer, third_computer];
                    vec_index.sort();
                    sets.insert(vec_index.into_iter().cloned().collect());
                }
            }
        }
        sets
    });
    sets_3.iter().filter(|&tuple| {
        tuple.iter().any(|s| s.starts_with("t"))
    }).count()
}



fn solve_2(input: &str) -> String {
    let connections = get_all_connections(input);
    //dbg!(bron_kerbosch(HashSet::new(), connections.keys().cloned().collect(), HashSet::new(), &connections));
    //let connex_part = connections.keys().fold(vec![], |mut connex_parts, node| {
    //    dbg!(node);
    //    let mut hash_start = Vec::new();
    //    hash_start.push(node.to_string());
    //    let full_connected = find_biggest_containg(hash_start, &connections);
    //    connex_parts.push(full_connected);
    //    connex_parts
    //}).into_iter().max_by(|connex_p1, connex_p2| connex_p1.len().cmp(&connex_p2.len())).unwrap();
    let vec_cliques = bron_kerbosch(HashSet::new(), connections.keys().cloned().collect(), HashSet::new(), &connections);
    let biggest_clique = vec_cliques.into_iter().max_by(|h1, h2| h1.len().cmp(&h2.len())).unwrap();
    let mut result_vec: Vec<_> = biggest_clique.into_iter().collect();
    result_vec.sort();
    result_vec.join(",")
}

fn bron_kerbosch(r: HashSet<String>, p: HashSet<String>, mut x: HashSet<String>, hash: &HashMap<String, Vec<String>>) -> Vec<HashSet<String>> {
    if x.is_empty() && p.is_empty() {
        return vec![r];
    }
    let mut p_clone = p.clone();
    p.iter().map(|vertex_p| {
        let mut r_new = r.clone();
        r_new.insert(vertex_p.to_string());
        let neighbors: HashSet<String> = hash.get(vertex_p).unwrap().iter().cloned().collect();
        let res = bron_kerbosch(r_new, p_clone.intersection(&neighbors).cloned().collect(), x.intersection(&neighbors).cloned().collect(), hash);
        p_clone.remove(vertex_p);
        x.remove(vertex_p);
        res
    }).flatten().collect()
}

fn find_biggest_containg(nodes: Vec<String>, hash: &HashMap<String, Vec<String>>) -> Vec<String> {
    let random_node = nodes.iter().next().unwrap();
    let result = hash.get(random_node).unwrap().into_iter().filter_map(|vertex| {
        if nodes.contains(vertex) {
            return None
        }
        let neighbors = hash.get(vertex).unwrap();
        if nodes.iter().all(|connected| neighbors.contains(connected)) {
            let mut nodes_clone = nodes.clone();
            nodes_clone.push(vertex.to_string());
            Some(find_biggest_containg(nodes_clone, hash))
        } else {
            None
        }
    }).max_by(|hash_1, hash_2| hash_1.len().cmp(&hash_2.len())).unwrap_or(nodes.clone());
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, "co,de,ka,ta");
    }
}
