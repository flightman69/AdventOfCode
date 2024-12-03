use std::collections::HashMap;
fn main() {
    let file = include_str!("./input.txt");

    let (vec1, vec2): (Vec<i32>, Vec<i32>) = file
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
                if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                    return Some((num1, num2));
                }
            }
            None
        })
        .unzip();

    let mut res = 0;
    let mut second_col_count: HashMap<i32, i32> = HashMap::new();

    for num in vec2 {
        *second_col_count.entry(num).or_insert(0) += 1;
    }

    for num in vec1 {
        if let Some(count) = second_col_count.get(&num) {
            res += num * count;
        }
    }

    println!("{}", res);
}
