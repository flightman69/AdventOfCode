fn main() {
    let file = include_str!("./input.txt");

    let mut unique_count = 0;

    for line in file.lines() {
        let parts: Vec<&str> = line.split(" | ").collect();
        let outputs: Vec<&str> = parts[1].split_whitespace().collect();

        // Count occurrences of unique segment lengths (2, 3, 4, 7)
        for output in outputs {
            match output.len() {
                2 | 3 | 4 | 7 => unique_count += 1,
                _ => {}
            }
        }
    }

    println!(
        "Total count of unique segment length outputs: {}",
        unique_count
    );
}
