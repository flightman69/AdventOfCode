use regex::Regex;

fn multiply(exp: &str) -> i32 {}

fn main() {
    let file = include_str!("input.txt");

    let initial_muls: &str = file.split("do()").collect::<Vec<&str>>()[0];
    let normal_mul_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    println!("{:?}", check);
}
