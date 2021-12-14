fn main() {
    let fishes: Vec<usize> = include_str!("./input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut nbr_fishes_per_timers = vec![0; 9];
    fishes.into_iter()
        .for_each(|fish| nbr_fishes_per_timers[fish] += 1);

    println!("{}", part_1_and_2(nbr_fishes_per_timers, 256));

}

fn part_1_and_2(nbr_fishes_per_timers: Vec<usize>, days: usize) -> usize {
    let mut n_fishes_per_timers = nbr_fishes_per_timers.clone();
    for _ in 0..days {
        n_fishes_per_timers.rotate_left(1);
        n_fishes_per_timers[6] += n_fishes_per_timers[8];
    }
    return n_fishes_per_timers.into_iter().sum();
}
