use nom::{bytes::complete::tag, character::complete::{char, digit1}, combinator::map_res, IResult};



fn main() {
    let input = include_str!("../input.txt");
    // dbg!(solve(input));
    dbg!(solve_2(input));
}

fn mul(input: &str) -> IResult<&str, u32> {
    // dbg!(input);
    let start = tag("mul(");
    let comma = char(',');
    let end_par = char(')');
    let mut parse_num = map_res(
        digit1,
        convert_u32
    );
    let (input, _) = start(input)?;
    let (input, num1) = parse_num(input)?;
    // dbg!(num1);
    let (input, _) = comma(input)?;
    let (input, num2) = parse_num(input)?;
    // dbg!(num2);
    let (input, _) = end_par(input)?;
    Ok((input, num1 * num2))
}

fn convert_u32(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 10)
}

fn solve(input: &str) -> u32 {
    (0..input.len()).map(|i_start| {
        mul(&input[i_start..]).map(|(_s, i)| i).unwrap_or(0)
    }).sum()
}

fn solve_2(input: &str) -> u32 {
    let mut enabled = true;
    (0..input.len()).map(|i_start| {
        let new_input = &input[i_start..];
        if enabled {
            if let Ok(_) = tag::<_,_,()>("don't()")(new_input) {
                enabled = false;
            }
        } else {
            if let Ok(_) = tag::<_,_,()>("do()")(new_input) {
                enabled = true;
            }
        }
        if enabled {
            mul(new_input).map(|(_s, i)| i).unwrap_or(0)
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
        assert_eq!(result, 161);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input_test_2.txt");
        let result = solve_2(input);
        assert_eq!(result, 48);
    }
}
