fn main() {
    let mut count = 0;
    let mut prev = -1;
    let input = include_str!("./input.txt")
        .trim()
        .split("\n")
        .for_each(|depth| {
            let intDepth = depth.parse().unwrap();
            if intDepth > prev && prev != -1 {
                count += 1;
            }
            prev = intDepth;
        });

    println!("{}", count);
}
