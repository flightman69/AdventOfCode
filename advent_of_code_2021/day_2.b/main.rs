use std::fs;

fn main() {
    let file_path = "./input.txt";

    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            println!("Err");
            String::new()
        }
    };

    let inputs: Vec<&str> = file_content.lines().collect();

    let mut horizon = 0;
    let mut aim = 0;
    let mut depth = 0;
    for input in inputs.iter() {
        let command: &str = input.split_whitespace().collect::<Vec<&str>>()[0];
        let value: i32 = input.split_whitespace().collect::<Vec<&str>>()[1]
            .trim()
            .parse()
            .unwrap();

        match command {
            "forward" => {
                horizon += value;
                depth += aim * value;
            }

            "up" => aim -= value,
            "down" => aim += value,
            _ => {}
        }
    }

    println!("{}", horizon * depth);
}
