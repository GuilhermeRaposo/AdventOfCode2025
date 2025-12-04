fn main() {
    let lines: Vec<&str> = include_str!("../problem.txt")
        .lines()
        .collect();

    let length = lines[0].chars().count();
    let height = lines.len();

    let mut grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut result = 0;
    let mut changed = true;
    while changed {
        changed = false;
        let mut next_grid = grid.clone();

        for i in 0..height {
            for j in 0..length {
                if grid[i][j] == '@' {
                    let mut count = 0;
                    for (di, dj) in directions {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;

                        if  ni >= 0 && ni < height as isize &&
                            nj >= 0 && nj < length as isize &&
                            grid[ni as usize][nj as usize] == '@' 
                        {
                            count += 1;
                        }
                    }

                    if count < 4 {
                        next_grid[i][j] = '.';
                        changed = false; // Change this to false for part 1 result
                        result += 1;
                    }
                }
            }
        }

        grid = next_grid;
    }

    println!("{}", result);
}
