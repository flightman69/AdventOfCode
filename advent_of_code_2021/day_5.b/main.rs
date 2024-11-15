use std::cmp::max;
use std::cmp::min;

fn main() {
    let file = include_str!("./input.txt");

    let coords: Vec<(i32, i32, i32, i32)> = file
        .lines()
        .filter_map(|line| {
            let mut parts = line
                .split(&[',', ' ', '-', '>'][..])
                .filter(|s| !s.is_empty());
            let x1 = parts.next()?.parse().ok()?;
            let y1 = parts.next()?.parse().ok()?;
            let x2 = parts.next()?.parse().ok()?;
            let y2 = parts.next()?.parse().ok()?;

            Some((x1, y1, x2, y2))
        })
        .collect();

    let max_coord: usize = coords
        .iter()
        .flat_map(|&(x1, y1, x2, y2)| vec![x1, y1, x2, y2])
        .max()
        .unwrap_or(0) as usize;

    let mut danger_lines = vec![vec![0; max_coord + 1]; max_coord + 1];
    for &(x1, y1, x2, y2) in coords.iter() {
        if x1 == x2 {
            for i in min(y1, y2)..=max(y1, y2) {
                danger_lines[i as usize][x1 as usize] += 1;
            }
        } else if y1 == y2 {
            for i in min(x1, x2)..=max(x1, x2) {
                danger_lines[y1 as usize][i as usize] += 1;
            }
        } else if (x1 - x2).abs() == (y1 - y2).abs() {
            let x_step = if x1 < x2 { 1 } else { -1 };
            let y_step = if y1 < y2 { 1 } else { -1 };

            let (mut x, mut y) = (x1, y1);
            while x != x2 + x_step && y != y2 + y_step {
                danger_lines[y as usize][x as usize] += 1;
                x += x_step;
                y += y_step;
            }
        }
    }

    let total_danger_points = danger_lines
        .iter()
        .flat_map(|row| row.iter()) // Flatten each row to access individual values
        .filter(|&n| *n > 1) // Filter values greater than 1
        .count();

    println!("{:?}", total_danger_points);
}