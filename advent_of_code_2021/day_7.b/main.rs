use std::collections::HashMap;

use std::cmp::min;
fn main() {
    let file = include_str!("./input.txt");
    let mut cheap_cost = i32::MAX;
    let horizontal_position: Vec<i32> = file
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    let mut my_hash: HashMap<i32, i32> = HashMap::new();
    for i in horizontal_position.iter() {
        *my_hash.entry(*i).or_insert(0) += 1;
    }

    let min_pos = *horizontal_position.iter().min().unwrap();
    let max_pos = *horizontal_position.iter().max().unwrap();

    for key in min_pos..=max_pos {
        let mut curr_cost = 0;
        for (keyj, valj) in my_hash.iter() {
            let n = (key - keyj).abs();
            let temp = (n * (n + 1)) / 2;
            curr_cost += temp * valj;
        }
        cheap_cost = min(cheap_cost, curr_cost);
    }

    println!("Cheapest cost: {}", cheap_cost);
}
