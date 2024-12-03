use regex::Regex;

fn main() {
    let file = include_str!("./input.txt");

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let matches: Vec<&str> = re.find_iter(file).map(|mat| mat.as_str()).collect();

    let mut sol = 0;

    for input in matches {
        let trim = input.trim_start_matches("mul(").trim_end_matches(")");
        let (num1, num2) = {
            let parts: Vec<&str> = trim.split(",").collect();
            println!("{:?}", parts);
            let n1: i32 = parts[0].parse().unwrap();
            let n2: i32 = parts[1].parse().unwrap();
            (n1, n2)
        };
        sol += num1 * num2;
    }

    println!("Solution: {}", sol);
}
