fn main() {
    let input = include_str!("../input.txt");
    dbg!(solve(input));
}

#[derive(Debug, Clone)]
struct KeyOrLock {
    key: bool,
    heights: Vec<usize>,
}

impl KeyOrLock {
    fn fits(&self, other: &Self) -> bool {
        self.heights.iter().zip(other.heights.iter()).all(|(h1, h2)| {
            h1 + h2 < 6
        })
    }
}

fn convert_input(input: &str) -> Vec<KeyOrLock> {
    let width = 5;
    input.split("\n\n").filter(|&s| s != "").map(|block| {
        let char_element = block.chars().next().unwrap();
        let key = char_element == '.';
        let block_vec: Vec<bool> = block.chars().filter(|&c| c != '\n').map(|c| {
            c == char_element
        }).collect();
        let heights = (0..width).map(|w| {
            let count_elt = (w..block_vec.len()).step_by(width).map(|index| block_vec[index]).filter(|b| *b).count();
            if key {
                6 - count_elt
            } else {
                count_elt - 1
            }
        }).collect();
        KeyOrLock {key, heights}
    }).collect()
}

fn solve(input: &str) -> usize {
    let vec_blocks = convert_input(input);
    let keys: Vec<KeyOrLock> = vec_blocks.iter().filter_map(|block| if block.key {Some(block.clone())} else {None}).collect();
    let locks: Vec<KeyOrLock> = vec_blocks.iter().filter_map(|block| if !block.key {Some(block.clone())} else {None}).collect();
    keys.into_iter().map(|key| {
        locks.iter().filter(|lock| key.fits(lock)).count()
    }).sum()
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input_test.txt");
        let result = solve(input);
        assert_eq!(result, 3);
    }
}
