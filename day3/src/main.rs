use std::fs;
use regex::Regex;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    let mut r = Regex::new(r"mul\(\d+\,\d+\)").unwrap();

    let valid: Vec<&str> = r.find_iter(&contents).map(|m| m.as_str()).collect();

    // Part one
    let mut total = 0;
    for call in valid {
        total += multiply(call);
    }
    println!("Total: {}", total);

    // Part two
    r = Regex::new(r"mul\(\d+\,\d+\)|do\(\)|don't\(\)").unwrap();
    let valid: Vec<&str> = r.find_iter(&contents).map(|m| m.as_str()).collect();
    total = 0;
    let mut condition = true;
    for call in valid {
        if call == "do()" {
            condition = true;
        } else if call == "don't()" {
            condition = false;
        } else if condition {
            total += multiply(call);

        }
    }
    println!("Conditional Total: {}", total);
}

fn multiply(call: &str) -> i32 {
    let n1 = call[4..(call.find(',').unwrap())].parse::<i32>().unwrap();
    let n2 = call[(call.find(',').unwrap() + 1)..(call.find(')').unwrap())].parse::<i32>().unwrap();

    n1 * n2
}