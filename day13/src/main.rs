use gcd::{euclid_u64, Gcd};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
    //dbg!(solve_2(input));
}

#[derive(Clone)]
struct Button {
    x_increase: i64,
    y_increase: i64,
}

#[derive(Clone)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: (i64, i64),
}

impl PartialEq for Machine {
    fn eq(&self, other: &Self) -> bool {
        self.prize == other.prize
    }
}


impl Machine {
    fn compute_state(&self, a_presses: i64, b_presses: i64) -> (i64, i64) {
        let state_x = self.prize.0 - self.button_a.x_increase * a_presses - self.button_b.x_increase * b_presses;
        let state_y = self.prize.1 - self.button_a.y_increase * a_presses - self.button_b.y_increase * b_presses;
        (state_x, state_y)
    }

    fn prize_reached(&self) -> bool {
        self.prize.0 == 0 && self.prize.1 == 0
    }
}

enum MachineState {
    Correct,
    Negative,
    Positive,
}

fn solve_machine(machine: &Machine, a_presses: i64, b_presses: i64) -> MachineState {
    use MachineState::*;
    let state = machine.compute_state(a_presses, b_presses);
    if state.0 < 0 || state.0 < 0 {
        Negative
    } else if state.0 == 0 && state.1 == 0 {
        Correct
    } else {
        Positive
    }
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input.split("\n\n").filter(|&s| s != "").map(|machine_s| {
        let mut iter_lines = machine_s.split("\n").filter(|&s| s != "");
        let button_a = parse_line_button(iter_lines.next().unwrap());
        let button_b = parse_line_button(iter_lines.next().unwrap());
        let prize = parse_line_x_y(iter_lines.next().unwrap());
        Machine {button_a, button_b, prize}
    }).collect()
}

fn parse_machines_2(input: &str) -> Vec<Machine> {
    input.split("\n\n").filter(|&s| s != "").map(|machine_s| {
        let mut iter_lines = machine_s.split("\n").filter(|&s| s != "");
        let button_a = parse_line_button(iter_lines.next().unwrap());
        let button_b = parse_line_button(iter_lines.next().unwrap());
        let mut prize = parse_line_x_y(iter_lines.next().unwrap());
        prize.0 += 10000000000000;
        prize.1 += 10000000000000;
        Machine {button_a, button_b, prize}
    }).collect()
}

fn parse_line_button(button_line: &str) -> Button {
    let (x_increase, y_increase) = parse_line_x_y(button_line);
    Button {x_increase, y_increase}
}

fn parse_line_x_y(line: &str) -> (i64, i64) {
    let mut iter_plus = line.split(['=', '+', ',']);
    iter_plus.next();
    let x_increase = iter_plus.next().unwrap().parse().unwrap();
    iter_plus.next();
    let y_increase = iter_plus.next().unwrap().parse().unwrap();
    (x_increase, y_increase)
}

fn solve(input: &str) -> i64 {
    use MachineState::*;
    let machines = parse_machines(input);
    machines.into_iter().filter_map(|machine| {
        let mut min_price: Option<i64> = None;
        for a_presses in 0..100 {
            for b_presses in 0..100 {
                match solve_machine(&machine, a_presses, b_presses) {
                    Negative => break,
                    Positive => (),
                    Correct => {
                        let current_price = a_presses * 3 + b_presses;
                        min_price = if let Some(previous_min) = min_price {
                            Some(previous_min.min(current_price))
                        } else {
                            Some(current_price)
                        };
                    },
                }
            }
        }
        dbg!(&min_price);
        min_price
    }).sum()
}




fn solve_2(input: &str) -> i64 {
    let machines = parse_machines_2(input);
    machines.into_iter().filter_map(|machine| {
        let mut min_price: Option<i64> = None;
        dbg!(&min_price);
        min_price
    }).sum()
}

fn get_opt_solution(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64)> {
    let (gcd, s, t) = mathematics::extended_euclidean_algorithm(a, b);
    if c % gcd != 0 {
        return None
    }
    // return 1 solution
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 480);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test.txt");
        let result = solve_2(input);
        assert_eq!(result, 1206);
    }
}
