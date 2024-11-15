use std::time::Instant;

fn modify_cycle(fish_count: &mut Vec<i64>) {
    let new_fish = fish_count[0];
    for day in 0..8 {
        fish_count[day as usize] = fish_count[(day as usize) + 1];
    }
    fish_count[6] += new_fish;
    fish_count[8] = new_fish;
}

fn main() {
    let start = Instant::now();
    let file = include_str!("./input.txt");
    let fish_lives: Vec<i64> = file
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    let mut fish_day_cycle: Vec<i64> = vec![0; 9];
    for day_left in fish_lives {
        fish_day_cycle[day_left as usize] += 1;
    }

    for _ in 1..=256 {
        modify_cycle(&mut fish_day_cycle);
    }

    let total_fish: i64 = fish_day_cycle.into_iter().sum();
    println!("Total fish: {}", total_fish);
    let end = Instant::now();
    println!("Time taken: {:?}", end - start);
}
