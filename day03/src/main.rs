use std::fs;

fn get_total_from_banks(banks: &[&str], digits: usize) -> u64 {
    let mut total = 0;
    for bank in banks {
        let mut bank_total: u64 = 0;

        let mut index = 0;
        let mut index_offset: isize = -1;
        while index < digits {
            let mut largest_battery_value: u64 = 0;
            let mut largest_battery_index = 0;
            let start: usize = (index_offset + 1) as usize;
            let end = bank.len() - (digits - index - 1);

            for (i, battery) in bank[start..end].chars().enumerate() {
                let index_without_offset = start + i;

                let current_battery: u64 = u64::from(battery.to_digit(10).unwrap());
                if current_battery > largest_battery_value {
                    largest_battery_value = current_battery;
                    largest_battery_index = index_without_offset;
                }
            }

            bank_total = bank_total * 10 + largest_battery_value;
            index += 1;
            index_offset = largest_battery_index as isize;
        }

        total += bank_total;
    }

    total
}

fn main() {
    let problem = fs::read_to_string("day03/problem.txt")
        .expect("Should have been able to read the file");
    let banks: Vec<&str> = problem
        .lines()
        .collect();

    let total_part1 = get_total_from_banks(&banks, 2);
    let total_part2 = get_total_from_banks(&banks, 12);

    println!("part1: {}", total_part1);
    println!("part2: {}", total_part2);
}
