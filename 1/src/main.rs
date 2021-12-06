fn main() {
    let depths: Vec<i32> = include_str!("./input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("{}", part_1(&depths));
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
