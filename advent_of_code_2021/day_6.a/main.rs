use std::time::Instance;
fn fish_reproduction_cycle(fish_counts: &mut Vec<i32>) {
    let mut new_fish: Vec<i32> = vec![];
    for timer in fish_counts.iter_mut() {
        if *timer == 0 {
            *timer = 6;
            new_fish.push(8);
        } else {
            *timer -= 1;
        }
    }
    fish_counts.extend(new_fish);
}

fn main() {
    let start = Instance::now();
    let file = include_str!("./input.txt");
    let mut fish_timer: Vec<i32> = file
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    for _ in 1..=80 {
        fish_reproduction_cycle(&mut fish_timer);
    }
    let fish_count_80_days = fish_timer.len();
    println!("{}", fish_count_80_days);
    let end = Instance::now();
    println!("{:?}", end.duration_since(start));
}
