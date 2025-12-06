use std::{fs};

fn main() {
    let input = fs::read_to_string("inputs/day06.txt").expect("failed to read input");
    println!("Part 1: Total grand: {}", part1(&input));
    println!("Part 2: Total grand: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let rows = lines.len();

    let mut numbers: Vec<Vec<i64>> = Vec::new();

    // Get the numbers
    for i in 0..rows {
        let curr_numbers: Vec<i64> = lines[i].split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok()).collect();
        numbers.push(curr_numbers);
    }
    let operators: Vec<char> = lines[rows - 1].chars().filter(|c| !c.is_whitespace()).collect();

    // Let's do the operations
    let total_ops = operators.len();
    let mut result: i64 = 0;

    for i in 0..total_ops {
        let curr_op: char = operators[i];

        match curr_op {
            '+' => {
                let mut local_result = 0;

                for j in 0..rows-1{
                    local_result += numbers[j][i];
                }

                result += local_result;
            },
            '*' => {
                let mut local_result = 1;

                for j in 0..rows-1{
                    local_result *= numbers[j][i];
                }

                result += local_result;
            },
            _ => panic!("Error procesing operation {}", curr_op)
        };
    }

    result
}

fn part2(input: &str) -> i64 {
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    
    // Normalize the width
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    for line in &mut lines {
        if line.len() < width {
            line.push_str(&" ".repeat(width - line.len()));
        }
    }
    
    // Converting into a grid of chars
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let last_row_idx = rows - 1;

    let mut is_separator = vec![false;width];

    for c in 0..width {
        let mut all_space = true;
        for r in 0..rows {
            if grid[r][c] != ' '{
                all_space = false;
                break;
            }
        }
        is_separator[c] = all_space;
    }

    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut c = 0;
    while c < width {
        if is_separator[c] {
            c +=1;
            continue;
        }

        let start = c;
        while c < width && !is_separator[c] {
            c += 1
        }
        let end = c - 1;
        blocks.push((start, end));
    }

    let mut grand_total: i64 = 0;

    for (start, end) in blocks {
        let mut op = '+';
        for col in start..=end {
            let ch = grid[last_row_idx][col];
            if ch != ' ' {
                op = ch;
                break;
            }
        }

        let mut result = if op == '+' {
            0
        }
        else{
            1
        };

        for col in (start..=end).rev() {
            let mut s = String::new();
            for r in 0..last_row_idx {
                let ch = grid[r][col];
                if ch.is_ascii_digit(){
                    s.push(ch);
                }
            }

            let n = if s.is_empty() {
                0
            }
            else{
                s.parse::<i64>().unwrap_or(0)
            };

            match op {
                '+' => result += n,
                '*' => result *= n,
                _ => panic!("Failed to process operator: {}", op)
            }
        }

        grand_total += result;
    }

    grand_total
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";
    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 3263827);
    }
}
