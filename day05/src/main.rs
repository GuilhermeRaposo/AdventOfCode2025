fn main() {
    let problem = include_str!("../problem.txt").replace("\r\n", "\n");
    let split: Vec<&str> = problem.split("\n\n").collect();

    let mut fresh_id_ranges: Vec<(u64, u64)> = split[0]
        .lines()
        .map(|r| {
            let (a, b) = r.split_once('-').unwrap();
            let min = a.parse::<u64>().unwrap();
            let max = b.parse::<u64>().unwrap();
            (min, max)
        })
        .collect();
    fresh_id_ranges.sort_by_key(|x| x.0);
    let available_ids = split[1].lines();

    // Merge overlapping or adjacent ranges.
    // This makes it easier to get the part2 result 
    // and also makes getting the result to part1 twice as fast :)
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for &(start, end) in &fresh_id_ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                // Extend the last merged range
                last.1 = last.1.max(end);
                continue;
            }
        }
        // No overlap, push new range
        merged.push((start, end));
    }

    let part2_result: u64 = merged
    .iter()
    .map(|(min, max)| (max - min) + 1)
    .sum();

    let mut part1_result = 0;
    for available_id in available_ids {
        for (min, max) in &merged {
            let id = available_id.parse::<u64>().unwrap();

            if id >= *min && id <= *max {
                part1_result += 1;
                break;
            }
        }
    }

    println!("part1: {}", part1_result);
    println!("part2: {}", part2_result);
}
