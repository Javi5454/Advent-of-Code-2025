
use std::{fs};

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").expect("failed to read input");
    println!("Part 1: Total joltage output: {}", part1(&input));
    println!("Part 2: Total joltage output: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.lines() {
        let digits: Vec<i64> = line.chars().map(|c| c.to_digit(10).expect("non-digit character. {}") as i64).collect();

        let n: usize = digits.len();

        let mut max_from_right: Vec<i64> = vec![0i64; n];
        let mut current_max = -1;

        for i in (0..n).rev() {
            let d = digits[i];

            if d > current_max {
                current_max = d;
            }

            max_from_right[i] = current_max;
        }

        let mut curr_best: i64 = -1;

        for i in 0..n-1 {
            let d1 = digits[i];
            let best_rigth = max_from_right[i + 1];
            let val = d1*10 + best_rigth;

            if val > curr_best {
                curr_best = val;
            }
        }   

        result += curr_best;
    }

    result

}

fn part2(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.lines() {
        let digits: Vec<i64> = line.chars().map(|c| c.to_digit(10).expect("non-digit character. {}") as i64).collect();

        let n: usize = digits.len();

        let mut best_val: Vec<u8> = Vec::with_capacity(12);
        let mut last_used: usize = 0;

        for pos in 0..12{
            let mut best_digit: u8 = 0;
            let mut best_pos: usize = n;

            let start = last_used;
            let end = n.saturating_sub(12 - pos - 1) - 1;

            // Look for the best digit form last_used + 1 till n - pos - 1

            for i in start..=end {
                if digits[i] as u8 > best_digit {
                    best_digit = digits[i] as u8;
                    best_pos = i;
                }
            }


            best_val.push(best_digit);
            last_used = best_pos +1;
        }

        let mut number: u64 = 0;
        for &digit in &best_val {
            number = number * 10 + digit as u64;
        }

        result += number as i64;
    }

    result

}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
    }
}