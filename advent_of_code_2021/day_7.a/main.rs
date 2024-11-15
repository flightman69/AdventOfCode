use std::cmp::min;
fn main() {
    let file = include_str!("./input.txt");
    let mut cheap_cost = i32::MAX;
    let horizontal_position: Vec<i32> = file
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    for i in horizontal_position.iter() {
        let mut current_cost = 0;
        for j in horizontal_position.iter() {
            current_cost += (i - j).abs();
        }
        cheap_cost = min(cheap_cost, current_cost);
    }
    println!("Cheapest cost: {}", cheap_cost);
}
