fn main() {
    let file = include_str!("./input.txt");
    let input_bits: Vec<&str> = file.lines().collect(); // array of bits as strings

    let oxygen_meter = get_meter_bit(&input_bits, 0 as usize, "oxygen");
    let co2_meter = get_meter_bit(&input_bits, 0 as usize, "co2");

    let life_possibility = oxygen_meter * co2_meter;
    println!("{}", life_possibility);
}

fn get_meter_bit(inputs: &Vec<&str>, index: usize, which_meter: &str) -> i32 {
    if inputs.len() == 1 {
        return i32::from_str_radix(inputs[0], 2).expect("oops!"); //return the final bits in decimal
    }

    let mut one_bits: Vec<&str> = vec![]; // array for 1 bit
    let mut zero_bits: Vec<&str> = vec![]; // array for 0 bit

    for bit in inputs.iter() {
        match bit.chars().nth(index) {
            Some('0') => zero_bits.push(bit),
            Some('1') => one_bits.push(bit),
            _ => {}
        };
    }

    // recurrsion babyy
    if which_meter == "oxygen" {
        if zero_bits.len() > one_bits.len() {
            get_meter_bit(&zero_bits, (index + 1) as usize, "oxygen")
        } else {
            get_meter_bit(&one_bits, (index + 1) as usize, "oxygen")
        }
    } else {
        if zero_bits.len() > one_bits.len() {
            get_meter_bit(&one_bits, (index + 1) as usize, "co2")
        } else {
            get_meter_bit(&zero_bits, (index + 1) as usize, "co2")
        }
    }
}
