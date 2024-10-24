use std::fs;

fn main() {
    let file_path = "./input.txt";

    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            println!("{e}");
            String::new()
        }
    };

    let inputs: Vec<&str> = file_content.lines().collect();

    let mut horizon = 0;
    let mut depth = 0;
    for input in inputs.iter() {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let value: i32 = parts[1].parse().unwrap();

        match command {
            "forward" => horizon += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => {}
        };
    }

    let final_value = horizon * depth;
    println!("{final_value}");
}
