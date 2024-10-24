use std::fs;

fn main() {
    let file_path = "../../day_1/inputs.txt";
    let contents = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            println!("{e}");
            String::new()
        }
    };

    let depths: Vec<i32> = contents
        .lines()
        .filter_map(|line| line.trim().parse::<i32>().ok())
        .collect();

    let mut increased_depths = 0;
    for window in depths.windows(2) {
        if window[0] > window[1] {
            increased_depths += 1;
        }
    }

    println!("{increased_depths}");
}
