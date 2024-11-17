fn main() {
    let file = include_str!("./input.txt");

    let mut trial: Vec<Vec<&str>> = Vec::new();
    let mut output: Vec<Vec<&str>> = Vec::new();

    for line in file.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let trial_temp: Vec<&str> = parts[0].trim().split_whitespace().collect();
        let output_temp: Vec<&str> = parts[1].trim().split_whitespace().collect();
        trial.push(trial_temp);
        output.push(output_temp);
    }

    let combination_1 = "ab";
    let combination_2 = "acdfg";
    let combination_3 = "abcdf";
    let combination_4 = "abef";
    let combination_5 = "bcdef";
    let combination_6 = "bcdefg";
    let combination_7 = "abd";
    let combination_8 = "abcdefg";
    let combination_9 = "abcdef";

    let mut total_count = 0;
    for combination in 0..trial.len() {
        let mut tried_1 = false;
        let mut tried_2 = false;
        let mut tried_3 = false;
        let mut tried_4 = false;
        let mut tried_5 = false;
        let mut tried_6 = false;
        let mut tried_7 = false;
        let mut tried_8 = false;
        let mut tried_9 = false;

        for tried in 0..trial[combination as usize].len() {
            let trial_number = trial[combination as usize][tried as usize];
            let mut trial_number: Vec<char> = trial_number.chars().collect();
            trial_number.sort();
            let trial_number: String = trial_number.into_iter().collect();

            match trial_number {
                combination_1 => tried_1 = true,
                combination_2 => tried_2 = true,
                combination_3 => tried_3 = true,
                combination_4 => tried_4 = true,
                combination_5 => tried_5 = true,
                combination_6 => tried_6 = true,
                combination_7 => tried_7 = true,
                combination_8 => tried_8 = true,
                combination_9 => tried_9 = true,
                _ => {}
            }
        }
        let mut four_digit_number: i32 = 0;
        for output_tried in 0..output[combination as usize].len() {
            let output_number = output[combination as usize][output_tried as usize];
            let mut output_number: Vec<char> = output_number.chars().collect();
            output_number.sort();
            let output_number: String = output_number.into_iter().collect();

            match output_number {
                number if number == combination_1 && tried_1 => {
                    four_digit_number *= 10;
                    four_digit_number += 1;
                }
                number if number == combination_2 && tried_2 => {
                    four_digit_number *= 10;
                    four_digit_number += 2;
                }
                number if number == combination_3 && tried_3 => {
                    four_digit_number *= 10;
                    four_digit_number += 3;
                }
                number if number == combination_4 && tried_4 => {
                    four_digit_number *= 10;
                    four_digit_number += 4;
                }
                number if number == combination_5 && tried_5 => {
                    four_digit_number *= 10;
                    four_digit_number += 5;
                }
                number if number == combination_6 && tried_6 => {
                    four_digit_number *= 10;
                    four_digit_number += 6;
                }
                number if number == combination_7 && tried_7 => {
                    four_digit_number *= 10;
                    four_digit_number += 7;
                }
                number if number == combination_8 && tried_8 => {
                    four_digit_number *= 10;
                    four_digit_number += 8;
                }
                number if number == combination_9 && tried_9 => {
                    four_digit_number *= 10;
                    four_digit_number += 9;
                }
                _ => {}
            }
        }
        total_count += four_digit_number;
    }

    println!("{}", total_count);
}
