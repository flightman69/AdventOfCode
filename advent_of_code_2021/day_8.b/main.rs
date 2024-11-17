use std::collections::HashSet;

fn main() {
    let file = include_str!("./input.txt");

    let mut total_sum = 0;

    for line in file.lines() {
        let parts: Vec<&str> = line.split(" | ").collect();
        let patterns: Vec<HashSet<char>> = parts[0]
            .split_whitespace()
            .map(|p| p.chars().collect())
            .collect();
        let outputs: Vec<HashSet<char>> = parts[1]
            .split_whitespace()
            .map(|o| o.chars().collect())
            .collect();

        // Identify digits 1, 4, 7, and 8 by their unique segment counts
        let mut digit_map = vec![HashSet::new(); 10];
        for pattern in &patterns {
            match pattern.len() {
                2 => digit_map[1] = pattern.clone(),
                4 => digit_map[4] = pattern.clone(),
                3 => digit_map[7] = pattern.clone(),
                7 => digit_map[8] = pattern.clone(),
                _ => {}
            }
        }

        // Deduce the remaining digits
        for pattern in &patterns {
            match pattern.len() {
                6 => {
                    if pattern.intersection(&digit_map[4]).count() == 4 {
                        digit_map[9] = pattern.clone();
                    } else if pattern.intersection(&digit_map[1]).count() == 2 {
                        digit_map[0] = pattern.clone();
                    } else {
                        digit_map[6] = pattern.clone();
                    }
                }
                5 => {
                    if pattern.intersection(&digit_map[1]).count() == 2 {
                        digit_map[3] = pattern.clone();
                    } else if pattern.intersection(&digit_map[4]).count() == 3 {
                        digit_map[5] = pattern.clone();
                    } else {
                        digit_map[2] = pattern.clone();
                    }
                }
                _ => {}
            }
        }

        // Decode the output values
        let mut number = 0;
        for output in outputs {
            for (i, digit_pattern) in digit_map.iter().enumerate() {
                if &output == digit_pattern {
                    number = number * 10 + i;
                    break;
                }
            }
        }
        total_sum += number;
    }

    println!("Total sum of all output values: {}", total_sum);
}
