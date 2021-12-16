use std::fs;
use std::str::Lines;


// Could use reduce?
fn get_most_common_bit(numbers: &[&str], bit_index: usize) -> char {
    let mut bit_count = 0;
    for num in numbers {
        let char = num.chars().nth(bit_index).unwrap();
        bit_count += if char == '1' { 1 } else { -1 };
    }
    return if bit_count >= 0 { '1' } else { '0' };
}

fn get_oxygen_generator_rating(report_lines: Lines) -> &str {
    let mut report_numbers: Vec<&str> = vec![];

    for line in report_lines {
        report_numbers.push(line)
    }

    let number_of_bits = report_numbers[0].len();

    let mut bit_index = 0;

    // Get oxygen generator rating
    while report_numbers.len() > 1 && bit_index < number_of_bits {

        let most_common_bit = get_most_common_bit(&report_numbers, bit_index);

        println!("Most common bit at index [{}]: {}", bit_index, most_common_bit);

        report_numbers.retain(
            |&num| num.chars().nth(bit_index).unwrap() == most_common_bit
        );

        println!("numbers remaining: {}", report_numbers.len());
        bit_index += 1;
    }

    return report_numbers[0];
}

fn get_co2_scrubber_rating(report_lines: Lines) -> &str {
    let mut report_numbers: Vec<&str> = vec![];

    for line in report_lines {
        report_numbers.push(line)
    }

    let number_of_bits = report_numbers[0].len();

    let mut bit_index = 0;

    while report_numbers.len() > 1 && bit_index < number_of_bits {

        let least_common_bit
            = if get_most_common_bit(&report_numbers, bit_index) == '0' { '1' } else { '0' };

        println!("Least common bit at index [{}]: {}", bit_index, least_common_bit);

        report_numbers.retain(
            |num| num.chars().nth(bit_index).unwrap() == least_common_bit
        );

        println!("numbers remaining: {}", report_numbers.len());
        bit_index += 1;
    }

    return report_numbers[0];
}

fn binary_string_to_number(string: &str) -> u32 {
    let mut num: u32 = 0;
    for char in string.chars() {
        num += char.to_digit(10).unwrap();
        num = num << 1;
    }
    num = num >> 1;
    return num;
}

fn main() {

    let diagnostic_report = fs::read_to_string("../input")
        .expect("Couldn't read input file.");

    let oxygen_generator_rating = get_oxygen_generator_rating(diagnostic_report.lines());
    println!("Oxygen generator rating: {}", oxygen_generator_rating);

    let co2_scrubber_rating = get_co2_scrubber_rating(diagnostic_report.lines());
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);

    let oxygen_generator_rating_val = binary_string_to_number(oxygen_generator_rating);
    let co2_scrubber_rating_val = binary_string_to_number(co2_scrubber_rating);

    println!("oxygen_generator_rating_val: {}", oxygen_generator_rating_val);
    println!("co2_scrubber_rating_val: {}", co2_scrubber_rating_val);
    println!("life support rating: {}", oxygen_generator_rating_val * co2_scrubber_rating_val);
}
