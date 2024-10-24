use std::fs;

fn main() {
    let file_path = "./inputs.txt";

    let file: String = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{e}");
            String::new()
        }
    };

    let numbers: Vec<i32> = file
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect();

    let mut count: i32 = 0;

    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            count += 1;
        }
    }

    println!("{count}");
}
