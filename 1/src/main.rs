fn main() {
    let depths: Vec<i32> = include_str!("./input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("{}", part_1(&depths));
    println!("{}", part_2(&depths));
}

fn part_1(depths: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev = -1;
    for depth in depths {
        if depth > &prev && prev != -1 {
            count += 1;
        }
        prev = *depth;
    }
    return count;
}

fn part_2(depths: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev = -1;
    for (i, depth) in depths.iter().enumerate() {
        if (i + 2) >= depths.len() {
            break;
        }
        let sum = depth + depths[i + 1] + depths[i + 2];
        if sum > prev && prev != -1 {
            count += 1;
        }
        prev = sum;
    }
    return count;
}
