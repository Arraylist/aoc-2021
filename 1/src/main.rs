fn main() {
    let mut count = 0;
    let mut prev = -1;
    let depths: Vec<i32> = include_str!("./input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for depth in depths {
        if depth > prev && prev != -1 {
            count += 1;
        }
        prev = depth;
    }

    println!("{}", count);
}
