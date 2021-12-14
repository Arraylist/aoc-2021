#![feature(int_abs_diff)]

fn main() {
    let mut crabs_pos: Vec<usize> = include_str!("./input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    crabs_pos.sort();

    println!("{}", part_1(crabs_pos));
}

fn part_1(crabs_pos: Vec<usize>) -> usize  {
    return find_naive(crabs_pos);
}

fn find_naive(crabs_pos: Vec<usize>) -> usize {
    let mut least_total_fuel = usize::MAX;
    for n in 0..=crabs_pos[crabs_pos.len() - 1] {
        let curr = get_total_fuel(&crabs_pos, n);
        if curr < least_total_fuel {
            least_total_fuel = curr;
        }
    }
    return least_total_fuel;
}

 fn get_total_fuel(crabs_pos: &Vec<usize>, final_pos: usize) -> usize {
    let mut sum = 0;
    for crab_pos in crabs_pos {
        sum += crab_pos.abs_diff(final_pos);
    }
    return sum;
}
