use itertools::{repeat_n, Itertools};
use std::fs;

#[derive(Debug)]
struct PotentialCalculation {
    total: i64,
    inputs: Vec<i64>,
}

fn add(left: i64, right: i64) -> i64 {
    left + right
}

fn multiply(left: i64, right: i64) -> i64 {
    left * right
}

fn concat(left: i64, right: i64) -> i64 {
    format!("{}{}", left, right).parse().unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let calculations: Vec<PotentialCalculation> = input
        .lines()
        .map(|line| PotentialCalculation {
            total: line.split(": ").next().unwrap().parse().unwrap(),
            inputs: line
                .split(": ")
                .nth(1)
                .unwrap()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect(),
        })
        .collect();

    let mut total_of_valid: i64 = 0;

    let possible_operations = [add, multiply, concat]; // Remove concat to solve part 1
    'calculation: for calculation in calculations {
        let perms: Vec<Vec<fn(i64, i64) -> i64>> = repeat_n(possible_operations, calculation.inputs.len() - 1).multi_cartesian_product().collect();
        for perm in perms {
            let mut total = calculation.inputs[0];
            for i in 0..perm.len() {
                total = perm[i](total, calculation.inputs[i + 1]);
            }
            if total == calculation.total {
                total_of_valid += total;
                continue 'calculation;
            }
        }
    }

    println!("Total of valid: {:?}", total_of_valid);
}
