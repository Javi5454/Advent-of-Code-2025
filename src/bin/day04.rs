use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day04.txt").expect("failed to read input");
    println!("Part 1: Total accesibles output: {}", part1(&input).0);
    println!("Part 2: Total accesibles output: {}", part2(&input).0);
}

fn count_neighbors(grid: &[String], r: usize, c: usize, rows: usize, columns: usize) -> i32 {
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < columns as i32 {
                if grid[nr as usize].as_bytes()[nc as usize] as char == '@' {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part1(input: &str) -> (i64, Vec<String>) {
    let grid: Vec<String> = input.lines().map(|x| x.to_string()).collect();

    let rows = grid.len();
    let columns = grid[0].len();

    let mut accesible_count: i64 = 0;
    let mut result_grid = grid.clone();

    //Finding and process each @
    for r in 0..rows {
        for c in 0..columns {
            if result_grid[r].as_bytes()[c] as char == '@' {
                let neighbors = count_neighbors(&grid, r, c, rows, columns);

                if neighbors < 4 {
                    result_grid[r].replace_range(c..c + 1, "x");
                    accesible_count += 1;
                }
            }
        }
    }

    (accesible_count, result_grid)
}

fn part2(input: &str) -> (i64, Vec<String>) {
    let mut grid: Vec<String> = input.lines().map(|x| x.to_string()).collect();

    let rows = grid.len();
    let columns = grid[0].len();

    let mut accesible_count: i64 = 0;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for r in 0..rows {
            for c in 0..columns {
                if grid[r].as_bytes()[c] as char == '@'{
                    let neigbors = count_neighbors(&grid, r, c, rows, columns);

                    if neigbors < 4{
                        to_remove.push((r,c));
                    }
                }
            }
        }

        if to_remove.is_empty(){
            break;
        }

        for (r,c) in to_remove{
            grid[r].replace_range(c..c+1, "x");
            accesible_count += 1;
        }
    }

    (accesible_count, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE).0, 13);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE).0, 43);
    }
}
