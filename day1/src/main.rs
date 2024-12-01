use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let mut one: Vec<u32> = vec![];
    let mut two: Vec<u32> = vec![];

    for line in contents.lines() {
        let left = line.split(' ').nth(0).unwrap().parse::<u32>().unwrap();
        let right = line.split(' ').nth(3).unwrap().parse::<u32>().unwrap();
        one.push(left);
        two.push(right);
    }
    one.sort();
    two.sort();

    // Part one
    let mut distance: u32 = 0;
    for pair in one.iter().zip(two.iter()) {
        if pair.0 > pair.1 {
            distance += pair.0 - pair.1;
        } else {
            distance += pair.1 - pair.0;
        }
    }
    println!("Distance: {}", distance);

    // Part two
    let mut similarity: u32 = 0;
    for location in one.iter() {
        let instances = two.iter().filter(|&x| x == location).count();
        similarity += instances as u32 * location;
    }
    println!("Similarity: {}", similarity);
}
