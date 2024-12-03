fn main() {
    let file = include_str!("./input.txt");

    let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = file
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

    vec1.sort();
    vec2.sort();
    let mut res = 0;
    for ind in 0..(vec1.len()) {
        let diff = vec1[ind] - vec2[ind];
        res += diff.abs();
    }
    println!("Day 1 A: {}", res);
}
