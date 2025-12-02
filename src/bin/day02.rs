use core::panic;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day02.txt").expect("failed to read input");
    println!("Part 1: Total invalid IDs: {}", part1(&input));
    println!("Part 2: Total invalid IDs: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut result: i64 = 0;

    for range in ranges {
        let bounds: Vec<&str> = range.split('-').collect();

        // Check two limits
        if bounds.len() != 2 {
            panic!("Expected exactly two numbers as id ranges");
        }

        let lower_bound: i64 = bounds[0].parse().expect("Failed to parse lower bound");
        let upper_bound: i64 = bounds[1].parse().expect("Failed to parse upper bound");

        let mut current: i64 = lower_bound;

        while current <= upper_bound {
            let s_num: String = current.to_string();
            let len: usize = s_num.len();

            if len % 2 == 0{
                let mid: usize = len / 2;
                let (first, second) = s_num.split_at(mid);

                if first == second {
                    result += current;
                }
            }

            current += 1;
        }
    }

    result

}

fn part2(input: &str) -> i64 {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut result: i64 = 0;

    for range in ranges {
        let bounds: Vec<&str> = range.split('-').collect();

        // Check two limits
        if bounds.len() != 2 {
            panic!("Expected exactly two numbers as id ranges");
        }

        let lower_bound: i64 = bounds[0].parse().expect("Failed to parse lower bound");
        let upper_bound: i64 = bounds[1].parse().expect("Failed to parse upper bound");

        let mut current: i64 = lower_bound;

        // Using KMP or similiar would be overkill for AoC
        while current <= upper_bound {
            let s_num: String = current.to_string();
            let len: usize = s_num.len();

            // For each k
            for k in 1..=len/2 {
                if len % k != 0 {
                    continue;
                }

                let mut valid_pattern: bool = true;
                let pattern: &str = &s_num[0..k];

                for i in (k..len).step_by(k){
                    if &s_num[i..i+k] != pattern{
                        valid_pattern = false;
                        break;
                    }
                }

                if valid_pattern {
                    result += current;
                    break;
                }
            }

            current += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}