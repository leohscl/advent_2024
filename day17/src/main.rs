use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    //let input = include_str!("../input_test_2.txt");
    //dbg!(solve(input));
    dbg!(solve_2(input));
}

#[derive(Clone)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    output: Vec<u8>,
}

impl Computer {
    fn parse_from_str(input: &str) -> Computer {
        let abc = input.split('\n').filter(|&c| c != "").map(|line| {
            line.split(": ").skip(1).next().unwrap().parse().unwrap()
        }).collect_vec();
        let (a, b, c) = (abc[0], abc[1], abc[2]);
        Computer {a, b, c, output: vec![]}
    }

    fn to_str(&self) -> String {
        format!("a: {}, b: {}, c: {}", self.a, self.b, self.c)
    }

    fn print_out(&self) -> String {
        self.output.iter().join(",")
    }

    fn run_instruction(&mut self, ip: &mut u8, instruction: u8, operand: u8) {
        let real_operand = if instruction == 1 || instruction == 3 {
            operand as usize
        } else {
            match operand {
                0..=3 => operand as usize,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => panic!(),
            }
        };

        match instruction {
            0 => {
                self.a = (self.a as f64 / (2usize.pow(real_operand as u32) as f64)).floor() as usize;
                *ip += 2;
            },
            1 => {
                self.b = self.b ^ real_operand;
                *ip += 2;
            },
            2 => {
                self.b = real_operand % 8;
                *ip += 2;
            }
            3 => {
                if self.a != 0 {
                    *ip = real_operand as u8;
                } else {
                    *ip += 2;
                }
            },
            4 => {
                self.b = self.b ^ self.c;
                *ip += 2;
            },
            5 => {
                self.output.push((real_operand % 8) as u8);
                *ip += 2;
            }
            6 => {
                self.b = (self.a as f64 / (2usize.pow(real_operand as u32) as f64)).floor() as usize;
                *ip += 2;
            },
            7 => {
                self.c = (self.a as f64 / (2usize.pow(real_operand as u32) as f64)).floor() as usize;
                *ip += 2;
            },
            _ => panic!(),
        }

    }

}

fn solve(input: &str) -> String {
    let mut parts_iter = input.split("\n\n").filter(|&s| s != "");
    let initialization = parts_iter.next().unwrap();
    let mut computer = Computer::parse_from_str(initialization);
    let program = parts_iter.next().unwrap();
    let program_vec = program.split([' ', ',', '\n']).filter(|&c| c != "").skip(1).map(|s| {s.parse::<u8>().unwrap()}).collect_vec();
    run_program(&mut computer, &program_vec)
}

fn run_program(computer: &mut Computer, program_vec: &Vec<u8>) -> String {
    let mut ip = 0;
    let mut ip_usize = ip as usize;
    while ip_usize < program_vec.len() {
        computer.run_instruction(&mut ip, program_vec[ip_usize], program_vec[ip_usize+1]);
        ip_usize = ip as usize;
    }
    computer.print_out()
}


fn find_min_solution(current_a: usize, values_left: &[u8]) -> Option<usize> {
    if values_left.len() == 0 {
        return Some(current_a);
    }
    let start = current_a * 8;
    (start..start + 8).find_map(|candidate_a: usize| {
        let mut b = (candidate_a % 8) ^ 5;
        let c = candidate_a / (2usize.pow(b as u32));
        b = b ^ 6;
        let remain = (b ^ c) % 8;
        if remain == values_left[0] as usize {
            find_min_solution(candidate_a, &values_left[1..])
        } else {
            None
        }
    })
}

fn solve_2(input: &str) -> usize {
    let mut parts_iter = input.split("\n\n").filter(|&s| s != "");
    let initialization = parts_iter.next().unwrap();
    let mut computer = Computer::parse_from_str(initialization);
    let program = parts_iter.next().unwrap();
    let program_vec = program.split([' ', ',', '\n']).filter(|&c| c != "").skip(1).map(|s| {s.parse::<u8>().unwrap()}).collect_vec();
    //dbg!(find_min_solution(0, &vec![0, 3]));
    let reversed_program = program_vec.iter().cloned().rev().collect_vec();
    let solution = find_min_solution(0, &reversed_program).unwrap();
    computer.a = solution;
    assert_eq!(run_program(&mut computer, &program_vec), program_vec.iter().join(","));
    solution
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test_2.txt");
        let result = solve_2(input);
        assert_eq!(result, 117440);
    }
}
