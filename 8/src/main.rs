fn main() {
    let patterns_outputs: Vec<(&str, &str)> = include_str!("./input.txt")
        .trim()
        .lines()
        .filter_map(|line| line.split_once(" | "))
        .collect();

    let mut outputs: Vec<&str> = Vec::with_capacity(1000);
    for po in patterns_outputs {
        outputs.push(po.1);
    }
    
    println!("{}", part_1(&outputs));
}

fn part_1(outputs: &Vec<&str>) -> usize {
    let mut single_digit_outputs: Vec<&str> = Vec::with_capacity(1000);
    outputs.into_iter()
        .for_each(|s| {
            let output: Vec<&str> = s.split_whitespace().collect();
            for o in output {
                single_digit_outputs.push(o);
            }
        });

    return single_digit_outputs
        .into_iter()
        .fold(0, |accum, digit| {
            match digit.len() {
                2 | 3 | 4 | 7 => accum + 1,
                _ => accum,
            }
        });
}
