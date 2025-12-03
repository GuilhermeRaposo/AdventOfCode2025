use std::fs;

fn main() {
    let problem = fs::read_to_string("day02/problem.txt")
        .expect("Should have been able to read the file");
    let id_ranges: Vec<&str> = problem
        .split(",")
        .collect();

    let mut invalid_ids_part1: Vec<String> = Vec::new();
    let mut invalid_ids_part2: Vec<String> = Vec::new();

    for range in &id_ranges {
        let min_max: Vec<&str> = range.split("-").collect();
        let min = min_max[0].parse::<i64>().unwrap();
        let max = min_max[1].parse::<i64>().unwrap();

        for i in min..=max {
            let as_string = i.to_string();
            let length = as_string.len();

            if length % 2 == 0 {
                let half = length / 2;
                let (first, second) = as_string.split_at(half);

                if first == second {
                    invalid_ids_part1.push(as_string.clone());
                }
            }

            for seq_length in 1..=length / 2 {
                if length % seq_length != 0 {
                    continue;
                }

                let times = length / seq_length;
                let (pattern, _) = as_string.split_at(seq_length);
                if pattern.repeat(times) == as_string {
                    invalid_ids_part2.push(as_string.clone());
                    break;
                }
            }
        }
    }

    let mut result_part1 = 0;
    for id in &invalid_ids_part1 {
        result_part1 += id.parse::<i64>().unwrap();
    }
    println!("Part 1: {}", result_part1);

    let mut result_part2 = 0;
    for id in &invalid_ids_part2 {
        result_part2 += id.parse::<i64>().unwrap();
    }
    println!("Part 2: {}", result_part2);
}
