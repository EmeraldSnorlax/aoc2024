use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet}, fs, hash::Hash, sync::Arc
};

#[derive(Debug)]
struct Antenna {
    frequency: char,
    x: usize,
    y: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let bounds: (usize, usize) = (
        input.lines().count() - 1,
        input.lines().next().unwrap().len() - 1,
    );
    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, frequency) in line.chars().enumerate() {
            if frequency == '.' {
                continue;
            }
            antennas
                .entry(frequency)
                .or_insert(Vec::new())
                .push(Antenna { frequency, x, y });
        }
    }

    let antinodes: Arc<Mutex<HashSet<(usize, usize)>>> = Arc::new(Mutex::new(HashSet::new()));
    let resonant_antiodes: Arc<Mutex<HashSet<(usize, usize)>>> = Arc::new(Mutex::new(HashSet::new()));

    antennas.par_iter().for_each(|(&_frequency, antennas)| {
        let pairs: Vec<(&Antenna, &Antenna)> = antennas
            .iter()
            .combinations(2)
            .map(|v| (v[0], v[1]))
            .collect();
        for (a1, a2) in pairs {
            // Part 1
            antinodes.lock().extend(generate_antinode_locations(a1, a2, bounds, false));

            // Part 2
            resonant_antiodes.lock().extend(generate_antinode_locations(a1, a2, bounds, true));
        }
    });
    println!("Antinodes: {:?}", antinodes.lock().len());
    println!("Resonant Antinodes: {:?}", resonant_antiodes.lock().len());
    
}

fn generate_antinode_locations(
    a1: &Antenna,
    a2: &Antenna,
    bounds: (usize, usize),
    resonant: bool,
) -> HashSet<(usize, usize)> {
    let displacement_from_a1_to_a2 = displacement(&(a1.x, a1.y), &(a2.x, a2.y));
    let displacement_from_a2_to_a1 = displacement(&(a2.x, a2.y), &(a1.x, a1.y));

    let locations_a1 = step_by_displacement(&(a1.x, a1.y), &displacement_from_a2_to_a1, resonant, &bounds);
    let locations_a2 = step_by_displacement(&(a2.x, a2.y), &displacement_from_a1_to_a2, resonant, &bounds);
    
    let mut antinodes = HashSet::from_iter(locations_a1.into_iter().chain(locations_a2.into_iter()));

    if resonant {
        antinodes.insert((a1.x, a1.y));
        antinodes.insert((a2.x, a2.y));
    }
    antinodes
}

fn is_within_bounds(point: &(i32, i32), bounds: &(usize, usize)) -> bool {
    point.0 >= 0 && point.0 <= bounds.0 as i32 && point.1 >= 0 && point.1 <= bounds.1 as i32
}

fn displacement(p1: &(usize, usize), p2: &(usize, usize)) -> (i32, i32) {
    ((p2.0 as i32 - p1.0 as i32), (p2.1 as i32 - p1.1 as i32))
}

fn step_by_displacement(p1: &(usize, usize), displacement: &(i32, i32), resonant: bool, bounds: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut antinode_locations = Vec::new();
    let mut antinode = (p1.0 as i32 + displacement.0, p1.1 as i32 + displacement.1);

    if is_within_bounds(&antinode, bounds) && !resonant {
        antinode_locations.push((antinode.0 as usize, antinode.1 as usize));
        return antinode_locations;
    }

    while is_within_bounds(&antinode, bounds) {
        antinode_locations.push((antinode.0 as usize, antinode.1 as usize));
        antinode = (antinode.0 + displacement.0, antinode.1 + displacement.1);
    }


    antinode_locations
}