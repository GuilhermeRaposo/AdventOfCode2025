fn part_1(lines: Vec<&str>) -> i64 {
    let (number_lines, operation_line) = lines.split_at(lines.len() - 1); 

    let numbers_grid: Vec<Vec<i64>> = number_lines.iter().map(|line| {
        line.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
    }).collect();
    let length = numbers_grid[0].len();
    let height = lines.len() - 1;
    
    let operators: Vec<char> = operation_line[0]
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect();

    let mut part1_result: i64 = 0;
    for i in 0..length {
        let operation = operators[i];
        let mut operation_result: i64 = 0;
        if operation == '*' {
            operation_result = 1;
        }

        for j in 0..height {
            if operation == '+'{
                operation_result += numbers_grid[j][i]
            }
            else {
                operation_result *= numbers_grid[j][i]
            }
        }

        part1_result += operation_result;
    }

    part1_result
}

fn part_2(lines: Vec<&str>) -> i64 {
	let (number_lines, operation_line) = lines.split_at(lines.len() - 1); 

	let starts = operation_line[0]
		.chars()
		.enumerate()
		.filter(|&(_, ch)| ch != ' ')
		.map(|(i, _)| i)
		.collect::<Vec<_>>();

	let operators = operation_line[0].chars().filter(|ch| *ch != ' ').collect::<Vec<_>>();

	let map = number_lines
		.into_iter()
		.map(|line| line.chars().collect::<Vec<char>>())
		.collect::<Vec<Vec<char>>>();

	let length = map[0].len();
	let height = map.len();

	let mut result = 0;
	for (col_index, &col_start) in starts.iter().enumerate() {
		let operator = operators[col_index];
		let col_end = starts.get(col_index + 1).copied().unwrap_or(length);

		let values = (col_start..col_end)
			.filter_map(|col| {
				(0..height)
					.map(|row| map[row][col])
					.filter(|&ch| ch != ' ')
					.collect::<String>()
					.parse::<i64>()
					.ok()
			})
			.collect::<Vec<_>>();

		if operator == '+' {
			result += values.iter().sum::<i64>();
		} else {
			result += values.iter().product::<i64>();
		}
	}

	result
}

fn main() {
    let lines: Vec<&str> = include_str!("../problem.txt")
        .lines()
        .collect();

    println!("part1: {}", part_1(lines.clone()));
    println!("part2: {}", part_2(lines.clone()));
}
