use std::{fs, vec};

fn main() {
    let mut grid: Vec<Vec<char>> = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;

    for _ in 0..4 {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                count += search_east_and_southeast_for_xmas((i, j), &grid);
            }
        }
        grid = rotate(grid);
    }
    print!("XMAS: {}", count);
}

fn search_east_and_southeast_for_xmas(pos: (usize, usize), grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    if grid[pos.0][pos.1] != 'X' {
        return 0;
    }

    // Just check for a match from left to right
    let mut discovered = "".to_owned();
    for i in 0..4 {
        let nx = pos.0 + i;
        if nx >= grid.len() {
            break;
        }
        discovered.push(grid[pos.0 + i][pos.1])
    }
    if discovered == "XMAS" {
        count += 1;
        println!("XMAS");
    }

    // Just check for SE direction match
    discovered = "".to_owned();
    for i in 0..4 {
        let nx = pos.0 + i;
        let ny = pos.1 + i;
        if nx >= grid.len() || ny >= grid[nx].len() {
            break;
        }
        discovered.push(grid[nx][ny]);
    }
    if discovered == "XMAS" {
        count += 1;
        println!("XMAS diag");
    }

    count
}

fn rotate(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![vec![' '; grid.len()]; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            new_grid[j][grid.len() - 1 - i] = grid[i][j];
        }
    }
    new_grid
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_east_and_southeast() {
        let grid = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['M', 'X', 'A', 'S'],
            vec!['A', 'A', 'X', 'S'],
            vec!['S', 'A', 'S', 'X'],
        ];
        assert_eq!(search_east_and_southeast_for_xmas((0, 0), &grid), 1);

        let grid = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['M', 'M', 'A', 'S'],
            vec!['A', 'A', 'A', 'S'],
            vec!['S', 'A', 'S', 'S'],
        ];
        assert_eq!(search_east_and_southeast_for_xmas((0, 0), &grid), 2);

        let grid = vec![
            vec!['M', 'M', 'A', 'S'],
            vec!['M', 'M', 'A', 'S'],
            vec!['A', 'A', 'A', 'S'],
            vec!['S', 'A', 'S', 'S'],
        ];
        assert_eq!(search_east_and_southeast_for_xmas((0, 0), &grid), 0);
    }

    #[test]
    fn test_rotate() {
        let grid = vec![
            vec!['1', '2', '3', '4'],
            vec!['5', '6', '7', '8'],
            vec!['9', 'A', 'B', 'C'],
        ];
        let rotated = vec![
            vec!['9', '5', '1'],
            vec!['A', '6', '2'],
            vec!['B', '7', '3'],
            vec!['C', '8', '4'],
        ];
        assert_eq!(rotate(&grid), rotated);
    }
}
