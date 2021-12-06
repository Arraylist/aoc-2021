fn main() {
    let instructions: Vec<Vec<&str>> = include_str!("./input.txt")
        .trim()
        .split("\n")
        .into_iter()
        .map(|s| s.split(' ').collect())
        .collect();

    println!("{}", part_1(&instructions));
    println!("{}", part_2(&instructions));
}

fn to_int(s: &str) -> i32 {
    return s.parse::<i32>().unwrap();
}

fn part_1(instructions: &Vec<Vec<&str>> ) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for instruction in instructions {
        if instruction[0] == "forward" {
            horizontal += to_int(instruction[1]);
        } else if instruction[0] == "down" {
            depth += to_int(instruction[1]);
        } else if instruction[0] == "up" {
            depth -= to_int(instruction[1]);
        }
    }
    return horizontal * depth;
}

fn part_2(instructions: &Vec<Vec<&str>> ) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instruction in instructions {
        if instruction[0] == "forward" {
            horizontal += to_int(instruction[1]);
            depth += aim * to_int(instruction[1]);
        } else if instruction[0] == "down" {
            aim += to_int(instruction[1]);
        } else if instruction[0] == "up" {
            aim -= to_int(instruction[1]);
        }
    }
    return horizontal * depth;
}
