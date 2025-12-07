use std::collections::HashMap;

fn main() {
    let lines: Vec<&str> = include_str!("../problem.txt")
    .lines()
    .collect();
    let mut result_str: Vec<String> = lines.clone().iter().map(|x| x.to_string()).collect();

    let start = lines[0].chars().position(| x| x == 'S').unwrap();
    
    let mut part1_result = 0;

    let mut beams = vec![start];
    let mut timelines = HashMap::new();
    timelines.insert(start, 1);

    for (i, line) in lines.iter().enumerate().skip(1) {
        let mut new_beams = vec![];
        let mut new_timelines = HashMap::new();

        for (j, c) in line.char_indices() {
            if beams.contains(&j) {
                if c == '^' {
                    result_str[i].replace_range(j - 1..j, "|");
                    result_str[i].replace_range(j + 1..j + 2, "|");
                    new_beams.append(&mut vec![j - 1, j + 1]);
                    part1_result += 1;
                }
                else {
                    result_str[i].replace_range(j..j + 1, "|");
                    new_beams.push(j);
                }
            }

            if let Some(&count) = timelines.get(&j) {
                if c == '^' {
                    if j > 0 {
                        *new_timelines.entry(j - 1).or_insert(0) += count;
                    }
                    if j + 1 < line.len() {
                        *new_timelines.entry(j + 1).or_insert(0) += count;
                    }
                }
                else {
                    *new_timelines.entry(j).or_insert(0) += count;
                }
            }
        }

        beams = new_beams;
        timelines = new_timelines;
    }


    for line in result_str {
        println!("{}", line);
    }

    let part2_result: i64 = timelines.values().sum();

    println!("part 1: {}", part1_result);
    println!("part 2: {}", part2_result);
}
