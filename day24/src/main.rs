use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
    dbg!(solve_2(input));
}

#[derive(Clone)]
enum Operation {
    XOR,
    OR,
    AND,
}

impl Operation {
    fn compute(&self, v1: u8, v2: u8) -> u8 {
        match self {
            Self::XOR => v1 ^ v2,
            Self::AND => v1 & v2,
            Self::OR => v1 | v2,
        }
    }
    fn to_str(&self) -> &str {
        match self {
            Self::XOR => " XOR ",
            Self::AND => " AND ",
            Self::OR => " OR ",
        }
    }
}

#[derive(Clone)]
struct Gate {
    operation: Operation,
    i1: String,
    i2: String,
    out: String,
}

impl Gate {
    fn to_str(&self) -> String {
        self.i1.clone() + self.operation.to_str() + &self.i2 + " = " + &self.out
    }
}

fn solve(input: &str) -> usize {
    let mut iter_parts = input.split("\n\n");
    let init = iter_parts.next().unwrap();
    let circuit_str = iter_parts.next().unwrap();
    let mut hash_values = translate_values(init);
    let circuit = build_circuit(circuit_str);
    run_circuit(&circuit, &mut hash_values);
    compute_circuit_out(&hash_values)
}

fn build_circuit(circuit_str: &str) -> Vec<Gate> {
    circuit_str.split('\n').filter(|&s| s != "").fold(Vec::new(), |mut vec, line| {
        let mut iter_part_line = line.split(' ');
        let i1 = iter_part_line.next().unwrap().to_string();
        let op_str = iter_part_line.next().unwrap();
        let i2 = iter_part_line.next().unwrap().to_string();
        let out = iter_part_line.skip(1).next().unwrap().to_string();
        let operation = match op_str {
            "XOR" => Operation::XOR,
            "OR" => Operation::OR,
            "AND" => Operation::AND,
            _ => panic!()
        };
        let gate = Gate {operation, i1, i2, out};
        vec.push(gate);
        vec
    })
}

fn run_circuit(circuit: &[Gate], values: &mut HashMap<String, u8>) {
    let mut not_updated: Vec<_> = circuit.iter().cloned().collect();
    let mut should_continue = true;
    while should_continue {
        not_updated.iter().for_each(|gate| {
            match (values.get(&gate.i1), values.get(&gate.i2)) {
                (Some(v1), Some(v2)) => {
                    let out_value = gate.operation.compute(*v1, *v2);
                    values.insert(gate.out.clone(), out_value);
                },
                _ => ()
            }
        });
        not_updated.retain(|gate| values.get(&gate.out).is_none());
        should_continue = !not_updated.is_empty();
    }
}

fn compute_circuit_out(values: &HashMap<String, u8>) -> usize {
    (0..64).filter_map(|z_i| {
        let z_input = format!("z{:02}", z_i);
        values.get(&z_input).and_then(|&i| Some(i as usize *  (1usize << z_i)))
    }).sum()
}

fn translate_values(init: &str) -> HashMap<String, u8> {
    init.split('\n').filter(|&s| s != "").fold(HashMap::new(), |mut hash, line| {
        let mut iter_part_line = line.split(": ");
        let name = iter_part_line.next().unwrap().to_string();
        let value = iter_part_line.next().unwrap().parse().unwrap();
        hash.insert(name, value);
        hash
    })
}

fn solve_2(input: &str) -> String {
    let mut iter_parts = input.split("\n\n");
    let _init = iter_parts.next().unwrap();
    let circuit_str = iter_parts.next().unwrap();
    let mut circuit = build_circuit(circuit_str);
    let mut hash_values = HashMap::new();
    (0..64).for_each(|i| {
        let x_input = format!("x{:02}", i);
        hash_values.insert(x_input, 0);
        let y_input = format!("y{:02}", i);
        hash_values.insert(y_input, 1);
    });
    swap_out(&mut circuit, 84, 167);
    swap_out(&mut circuit, 13, 11);
    swap_out(&mut circuit, 152, 216);
    swap_out(&mut circuit, 182, 217);
    let indices = [84, 167, 13, 11, 152, 216, 182, 217];
    let mut outputs = circuit.iter().enumerate().filter_map(|(i, gate)| {
        if indices.contains(&i) {
            Some(gate.out.clone())
        } else {
            None
        }
    }).collect::<Vec<String>>();
    outputs.sort();
    outputs.join(",")
}

//fn test_addition_small(circuit: &[Gate]) -> bool {
//    let mut hash_values = HashMap::new();
//    (0..64).for_each(|i| {
//        let x_input = format!("x{:02}", i);
//        hash_values.insert(x_input, 0);
//        let y_input = format!("y{:02}", i);
//        hash_values.insert(y_input, 1);
//    });
//    run_circuit(&circuit, &mut hash_values);
//    let res = compute_circuit_out(&hash_values);
//    dbg!(res);
//    res % (1 << 10) == 2
//}
//
fn swap_out(circuit: &mut Vec<Gate>, i1: usize, i2: usize) {
    let out_1 = circuit[i1].out.clone();
    circuit[i1].out = circuit[i2].out.clone();
    circuit[i2].out = out_1;
}
//fn try_swap_indexes(circuit: &mut Vec<Gate>, indexes: &[usize]) {
//    for i in 0..indexes.len() {
//        for j in i+1..indexes.len() {
//            let mut circuit_clone = circuit.clone();
//            let out_1 = circuit_clone[indexes[i]].out.clone();
//            circuit_clone[indexes[i]].out = circuit_clone[indexes[j]].out.clone();
//            circuit_clone[indexes[j]].out = out_1;
//            if test_addition_small(&circuit_clone) {
//                dbg!(i, j);
//            }
//        }
//    }
//}

fn index_outputs(value: String, circuit: &[Gate]) -> Vec<usize> {
    let inputs = vec![value];
    let mut gates_index = vec![];
    circuit.iter().enumerate().for_each(|(i,gate)| {
        if inputs.contains(&gate.i1) || inputs.contains(&gate.i2) {
            gates_index.push(i);
        }
    });
    gates_index
}

fn index_dependencies(value: String, circuit: &[Gate]) -> Vec<usize> {
    let outputs = vec![value];
    let mut gates_index = vec![];
    circuit.iter().enumerate().for_each(|(i,gate)| {
        if outputs.contains(&gate.out) {
            gates_index.push(i);
        }
    });
    gates_index
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 2024);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(&result, "");
    }
}
