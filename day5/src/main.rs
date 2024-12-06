use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let lines = input.lines().collect::<Vec<&str>>();

    let (rules, jobs) = parse_input(lines);

    let valid_jobs: Vec<&Vec<u8>> = jobs
        .iter()
        .filter(|job| is_job_valid(&rules, job))
        .collect();

    let mut total_of_middle_pages: u32 = 0;
    for job in valid_jobs {
        total_of_middle_pages += job[job.len() / 2] as u32;
    }
    println!("Total of middle pages: {}", total_of_middle_pages);
}

fn is_job_valid(rules: &HashMap<u8, Vec<u8>>, job: &Vec<u8>) -> bool {
    let mut valid = true;

    'page: for page_i in 0..job.len() {
        let page = job[page_i];
        if !rules.contains_key(&page) {
            continue 'page;
        }
        'rule: for page_rule in rules.get(&page).unwrap() {
            if !job.contains(page_rule) {
                continue 'rule;
            }
            // Check if the page_rule appears after the current page by scanning after page_i
            for i in 0..page_i {
                if job[i] == *page_rule {
                    valid = false;
                    break 'page;
                }
            }
        }
    }
    valid
}

fn parse_input(lines: Vec<&str>) -> (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut jobs: Vec<Vec<u8>> = vec![];

    for line in lines {
        if line.contains('|') {
            let before: u8 = line.split("|").collect::<Vec<&str>>()[0].parse().unwrap();
            let after: u8 = line.split("|").collect::<Vec<&str>>()[1].parse().unwrap();
            rules.entry(before).or_insert(Vec::new()).push(after);
        } else if line.contains(',') {
            jobs.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        }
    }

    (rules, jobs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_job_valid() {
        let input = fs::read_to_string("small.txt").expect("Something went wrong reading the file");
        let lines = input.lines().collect::<Vec<&str>>();
        let (rules, jobs) = parse_input(lines);

        println!("{:?}", rules);
        println!("{:?}", jobs[0]);
        assert_eq!(is_job_valid(&rules, &jobs[0]), true);
        assert_eq!(is_job_valid(&rules, &jobs[1]), true);
        assert_eq!(is_job_valid(&rules, &jobs[2]), true);
        assert_eq!(is_job_valid(&rules, &jobs[3]), false);
        assert_eq!(is_job_valid(&rules, &jobs[4]), false);
        assert_eq!(is_job_valid(&rules, &jobs[5]), false);
    }
}
