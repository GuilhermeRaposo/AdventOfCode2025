use std::fs;

const START_POSITION: i32 = 50;

#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT
}

impl From<char> for Direction {
    fn from(input: char) -> Self {
        match input {
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => Direction::LEFT
        }
    }
}

struct Instruction {
    direction: Direction,
    clicks: i32
}

fn main() {
    let problem = fs::read_to_string("day01/problem.txt")
        .expect("Should have been able to read the file");
    let instructions: Vec<Instruction> = problem
        .lines()
        .map(|x| Instruction {
            direction: Direction::from(x.chars().next().unwrap()),
            clicks: (x[1..].parse::<i32>().unwrap()) } )
        .collect();

    let mut zero_count= 0;
    let mut zero_count_inc_rotation = 0;
    let mut current_position = START_POSITION;

    // Negative current position means we dialed to the left and if we take (current_position % 100 + 100) % 100
    // We get the final position after accounting for rotations and the negative sign.
    // Instead, a current position that is > 99 means we dialed right but in this case we just need current_position / 100.
    for instruction in &instructions {
        // For a left dial we need this so we don't count the passes through 0 an extra time.
        let start_at_zero = current_position == 0;
        
        if instruction.direction == Direction::LEFT {
            current_position -= instruction.clicks;

            if current_position < 0 {
                zero_count_inc_rotation += ((current_position - 100) / 100).abs();
                if start_at_zero {
                    zero_count_inc_rotation -= 1;
                }
                current_position = (current_position % 100 + 100) % 100;
            }
            // Count a left dial that landed exactly at 0 (didn't wrap around)
            else if current_position == 0 {
                zero_count_inc_rotation += 1;
            }
        }
        else {
            current_position += instruction.clicks;
            if current_position >= 100 {
                zero_count_inc_rotation += current_position / 100;
                current_position = current_position % 100;
            }
        }

        if current_position == 0 {
            zero_count += 1;
        }
    }

    println!("zero count: {}", zero_count);
    println!("zero count including rotations: {}", zero_count_inc_rotation);
}
