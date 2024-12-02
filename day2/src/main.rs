use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    let reports: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let mut safe_reports_p1: u32 = 0;
    let mut safe_reports_p2: u32 = 0;
    for report in reports.iter() {
        let p1_safe = validate_report(&report);
        safe_reports_p1 += p1_safe as u32;

        let mut p2_safe: bool = false;
        for i in 0..report.len() {
            let mut edited_report = report.clone();
            edited_report.remove(i);
            if validate_report(&edited_report) {
                p2_safe = true;
                break;
            }
        }
        safe_reports_p2 += p2_safe as u32;
    }
    println!("Safe reports: {}", safe_reports_p1);
    println!("Safe reports dampened: {}", safe_reports_p2);
}

fn validate_report(report: &Vec<i32>) -> bool {
    let ascending: bool = &report[0] < report.last().unwrap();
    let mut safe: bool = true;
    for pair in report.windows(2) {
        // &= preserves true, until it encounters a false; then it's false forever
        safe &= !((ascending && pair[0] > pair[1])
            || (!ascending && pair[0] < pair[1])
            || (pair[0] - pair[1]).abs() > 3
            || pair[0] == pair[1]);
    }
    safe
}
