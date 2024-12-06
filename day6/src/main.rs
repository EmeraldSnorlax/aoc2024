use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq)]
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

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
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
                    guard.position = (i, j);
                    row.push(Position::Empty);
                }
                _ => panic!("Unknown character: {}", c),
            }
        }
        grid.push(row);
    }

    while guard.move_forward(&grid) != MoveOutcome::Finished {
        if guard.move_forward(&grid) == MoveOutcome::Blocked {
            guard.turn_right();
        }
    }
    println!("Visited locations: {:?}", guard.visited.len());
}
