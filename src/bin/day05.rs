use std::{fs};

fn main() {
    let input = fs::read_to_string("inputs/day05.txt").expect("failed to read input");
    println!("Part 1: Total fresh ingredients output: {}", part1(&input));
    println!("Part 2: Total fresh ingredients output: {}", part2(&input));
}

fn parse_ranges(s: &str) -> Option<(usize,usize)> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() == 2 {
        let start = parts[0].parse::<usize>().ok()?;
        let end = parts[1].parse::<usize>().ok()?;
        Some((start,end))
    }
    else{
        None
    }
}

fn part1(input: &str) -> i64 {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut parsing_ranges = true;

    for line in input.lines(){
        let trimmed = line.trim();
        if trimmed.is_empty() {
            parsing_ranges = false; // Now start parsings IDs
            continue;
        }

        if parsing_ranges {
            if let Some((start, end)) = parse_ranges(trimmed){
                ranges.push((start,end));
            }
        }
        else{
            if let Ok(id) = trimmed.parse::<usize>() {
                ids.push(id);
            }
        }
    }

    let mut fresh: i64 = 0;
    for id in ids{
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}

fn part2(input: &str) -> i64 {
    let mut ranges = Vec::new();
    let mut parsing_ranges = true;

    for line in input.lines(){
        let trimmed = line.trim();
        if trimmed.is_empty() {
            parsing_ranges = false; // Now start parsings IDs
            continue;
        }

        if parsing_ranges {
            if let Some((start, end)) = parse_ranges(trimmed){
                ranges.push((start,end));
            }
        }
        else{
            break ;
        }
    }

    // Order intervarls by start
    ranges.sort_by_key(|(s, _)| *s);

    // Merge interval
    let mut merged: Vec<(usize, usize)> = Vec::new();

    for (start, end) in ranges{
        if let Some(last) = merged.last_mut() {
            if last.1 + 1 >= start {
                last.1 = last.1.max(end);
            }
            else{
                merged.push((start, end));
            }
        } else {
            merged.push((start,end));
        }
    }
    
    // Sum intervals len
    let mut total = 0;
    for (start, end) in merged {
        total += end - start + 1;
    }

    total as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 14);
    }
}
