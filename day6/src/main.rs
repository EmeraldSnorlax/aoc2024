use parking_lot::Mutex;
use rayon::prelude::*;
use std::{collections::HashSet, fs, sync::Arc};

#[derive(Debug, PartialEq, Clone)]
enum Position {
    Empty,
    Blocked,
}

#[derive(Debug, Clone)]
struct Guard {
    facing: Direction,
    position: (usize, usize),
    visited: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
enum MoveOutcome {
    Moved,
    Blocked,
    Finished,
}

impl Guard {
    fn turn_right(&mut self) -> &Self {
        self.facing = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        self
    }

    fn visit(&mut self) -> &Self {
        self.visited.insert(self.position);
        self
    }

    fn move_forward(&mut self, grid: &Vec<Vec<Position>>) -> MoveOutcome {
        self.visit();
        match self.facing {
            Direction::North => {
                if self.position.0 == 0 {
                    return MoveOutcome::Finished;
                } else if grid[self.position.0 - 1][self.position.1] == Position::Blocked {
                    return MoveOutcome::Blocked;
                } else {
                    self.position.0 -= 1;
                    return MoveOutcome::Moved;
                }
            }
            Direction::East => {
                if self.position.1 == grid[0].len() - 1 {
                    return MoveOutcome::Finished;
                } else if grid[self.position.0][self.position.1 + 1] == Position::Blocked {
                    return MoveOutcome::Blocked;
                } else {
                    self.position.1 += 1;
                    return MoveOutcome::Moved;
                }
            }
            Direction::South => {
                if self.position.0 == grid.len() - 1 {
                    return MoveOutcome::Finished;
                } else if grid[self.position.0 + 1][self.position.1] == Position::Blocked {
                    return MoveOutcome::Blocked;
                } else {
                    self.position.0 += 1;
                    return MoveOutcome::Moved;
                }
            }
            Direction::West => {
                if self.position.1 == 0 {
                    return MoveOutcome::Finished;
                } else if grid[self.position.0][self.position.1 - 1] == Position::Blocked {
                    return MoveOutcome::Blocked;
                } else {
                    self.position.1 -= 1;
                    return MoveOutcome::Moved;
                }
            }
        }
    }
}

fn simulate<'a>(grid: &Vec<Vec<Position>>, guard: &'a mut Guard) -> Option<&'a Guard> {
    let mut iterations = 0;
    while guard.move_forward(&grid) != MoveOutcome::Finished {
        if guard.move_forward(&grid) == MoveOutcome::Blocked {
            guard.turn_right();
        }
        iterations += 1;
        if iterations > 1_000_000 {
            return None;
        }
    }
    return Some(guard);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut start_position: (usize, usize) = (0, 0);
    let mut guard = Guard {
        facing: Direction::North,
        position: (0, 0),
        visited: HashSet::new(),
    };

    let mut grid: Vec<Vec<Position>> = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Position::Empty),
                '#' => row.push(Position::Blocked),
                '^' => {
                    start_position = (i, j);
                    guard.position = start_position;
                    row.push(Position::Empty);
                }
                _ => panic!("Unknown character: {}", c),
            }
        }
        grid.push(row);
    }

    // Part 1
    simulate(&grid, &mut guard);
    println!("Visited locations: {:?}", guard.visited.len());

    // Part 2: just fucking bruteforce it i'm way too stupid for this
    let infinite_loops_found = Arc::new(Mutex::new(0));
    (0..grid.len()).into_par_iter().for_each(|i| {
        (0..grid[0].len()).into_par_iter().for_each(|j| {
            if grid[i][j] == Position::Empty {
                let mut grid = grid.clone();
                grid[i][j] = Position::Blocked;
                let mut guard = Guard {
                    facing: Direction::North,
                    position: start_position,
                    visited: HashSet::new(),
                };
                match simulate(&grid, &mut guard) {
                    Some(_guard) => {}
                    None => {
                        let mut infinite_loops_found = infinite_loops_found.lock();
                        *infinite_loops_found += 1;
                    }
                }
            }
        })
    });
    println!("Infinite loops found: {}", infinite_loops_found.lock());
}
