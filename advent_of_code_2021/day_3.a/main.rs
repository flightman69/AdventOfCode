fn main() {
    let file = include_str!("./input.txt");

    let total_bits = file.lines().count();
    let bits_len = file.lines().collect::<Vec<_>>()[0].len();
    println!("{:?}, {bits_len:?}", total_bits);

    let mut one_bit_count = vec![0; bits_len];
    for bits in file.lines() {
        for bit in 0..bits_len {
            if bits.chars().nth(bit) == Some('1') {
                one_bit_count[bit] += 1;
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for bit_count in one_bit_count {
        println!("{bit_count}");
        match bit_count.cmp(&(total_bits / 2)) {
            std::cmp::Ordering::Greater => {
                gamma += "1";
                epsilon += "0";
            }

            std::cmp::Ordering::Less => {
                gamma += "0";
                epsilon += "1";
            }
            _ => {}
        };
    }
    println!("Gamma: {gamma}, Epsilon: {epsilon}");
    let gamma_decimal = bin_2_dec(gamma);
    let epsilon_decimal = bin_2_dec(epsilon);
    println!("{}", gamma_decimal * epsilon_decimal);
}

fn bin_2_dec(str: String) -> u32 {
    let mut decimal = 0;
    for (i, bit) in str.chars().rev().enumerate() {
        if bit == '1' {
            decimal += 1 << i;
        }
    }
    decimal
}
