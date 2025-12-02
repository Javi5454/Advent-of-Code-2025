use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day01.txt").expect("failed to read input");
    println!("Part 1: Password (times dial points at 0): {}", part1(&input));
    println!("Part 2: Password (times dial points at 0 during and at end): {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut position = 50; // Initial position
    let mut zero_count = 0;

    for line in input.lines() {
        if line.is_empty(){
            continue;
        }
        let dir = line.chars().next().unwrap();
        let dist: i32 = line[1..].parse().expect("expected number after direction");

        position = match dir {
            'L' => (position - dist).rem_euclid(100),
            'R' => (position + dist).rem_euclid(100),
            _ => panic!("unexpected direcion"),
        };

        if position == 0{
            zero_count += 1;
        }
    }

    zero_count
}

fn part2(input: &str) -> i32 {
    let mut position = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let dir = line.chars().next().unwrap();
        let dist: i32 = line[1..].parse().expect("expected number after direction");

        let step = match dir {
            'L' => -1,
            'R' => 1,
            _ => panic!("unexpected direction"),
        };

        for _ in 0..dist {
            position = ((position + step) % 100 + 100) % 100;

            if position == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2_example(){
        assert_eq!(part2(EXAMPLE), 6);
    }
}