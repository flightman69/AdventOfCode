use std::fs;

fn main() {
    let file_path = "./inputs.txt";
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Error reading file");
            String::new()
        }
    };

    let depths: Vec<i32> = file_content
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect();

    let mut increased_depths = 0;
    for window in depths.windows(4) {
        let a = window[0] + window[1] + window[2];
        let b = window[1] + window[2] + window[3];
        if b > a {
            increased_depths += 1;
        }
    }
    println!("{increased_depths}");
}
