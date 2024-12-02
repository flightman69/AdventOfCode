fn main() {
    let file = include_str!("./input.txt");
    let mut tot_safe_levels = 0;

    let data: Vec<Vec<i32>> = file
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    for row in data {
        let mut is_safe = true;
        let mut inc = true;
        let mut dec = true;

        for ind in 0..(row.len() - 1) {
            let a = row[ind];
            let b = row[ind + 1];
            let diff = (a - b).abs();
            if diff > 3 || diff < 1 {
                is_safe = false;
                break;
            }
            if a > b {
                inc = false;
            }
            if a < b {
                dec = false;
            }

            if !inc && !dec {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            println!("{:?}", row);
            tot_safe_levels += 1;
        }
    }

    println!("safes {}", tot_safe_levels);
}
