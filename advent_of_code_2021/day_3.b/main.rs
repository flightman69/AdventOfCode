fn main() {
    let file = include_str!("./input.txt");
    let input_bits: Vec<&str> = file.lines().collect();

    let oxygen_meter = get_oxygen(&input_bits, 0 as usize);
    let co2_meter = get_co2(&input_bits, 0 as usize);

    let life_possibility = oxygen_meter * co2_meter;
    println!("{}", life_possibility);
}

fn get_oxygen(inputs: &Vec<&str>, index: usize) -> i32 {
    if inputs.len() == 1 {
        return bin_2_dec(inputs[0]);
    }
    let mut one_bits: Vec<&str> = vec![];
    let mut zero_bits: Vec<&str> = vec![];

    for bit in inputs.iter() {
        match bit.chars().nth(index) {
            Some('0') => zero_bits.push(bit),
            Some('1') => one_bits.push(bit),
            _ => {}
        };
    }
    if zero_bits.len() > one_bits.len() {
        get_oxygen(&zero_bits, (index + 1) as usize)
    } else {
        get_oxygen(&one_bits, (index + 1) as usize)
    }
}

fn get_co2(inputs: &Vec<&str>, index: usize) -> i32 {
    if inputs.len() == 1 {
        return bin_2_dec(inputs[0]);
    }
    let mut one_bits: Vec<&str> = vec![];
    let mut zero_bits: Vec<&str> = vec![];

    for bit in inputs.iter() {
        match bit.chars().nth(index) {
            Some('0') => zero_bits.push(bit),
            Some('1') => one_bits.push(bit),
            _ => {}
        };
    }
    if zero_bits.len() > one_bits.len() {
        get_co2(&one_bits, (index + 1) as usize)
    } else {
        get_co2(&zero_bits, (index + 1) as usize)
    }
}

fn bin_2_dec(string: &str) -> i32 {
    let mut decimal = 0;
    for (i, bit) in string.chars().rev().enumerate() {
        if bit == '1' {
            decimal += 1 << i;
        }
    }
    decimal
}
