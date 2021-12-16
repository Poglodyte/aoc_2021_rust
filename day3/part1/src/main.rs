use std::fs;

fn add_to_bit_counts(line: &str, bit_counts: &mut Vec<i32>) {
    for (i, char) in line.chars().enumerate() {
        if char == '0' {
            // println!("{} - {} zero", i, char);
            bit_counts[i] -= 1;
        } else {
            // println!("{} - {} one", i, char);
            bit_counts[i] += 1;
        }
    }

    // for count in bit_counts {
    //     println!("{}", count);
    // }
}

fn main() {

    let diagnostic_report = fs::read_to_string("../input")
        .expect("Couldn't read input file.");

    let mut report_lines = diagnostic_report.lines();
    let first_line = report_lines.next().unwrap();

    let num_bits = first_line.len();

    let mut most_common_bit_counts = vec![0; num_bits];

    // println!("first line: {}", first_line);
    // println!("num bits: {}", num_bits);


    for line in diagnostic_report.lines() {
        // println!("{}, length={}", line, line.len());
        add_to_bit_counts(line, &mut most_common_bit_counts);
    }

    // Build gamma rate and epsilon rate values from bit counts
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    for count in most_common_bit_counts {

        if count > 0 {
            // 1 was most common bit, add to gamma_rate
            gamma_rate += 1;
            println!("gamma_rate_val: 1");
        }
        else {
            // 1 was least common bit, add to epsilon_rate
            epsilon_rate += 1;
            println!("gamma_rate_val: 0");
        }

        // Shift left 1 position
        gamma_rate = gamma_rate << 1;
        epsilon_rate = epsilon_rate << 1;

        // println!("count={}, gamma_rate={}", count, gamma_rate);

    }

    // Shift back right one position (there's one extra shift in above loop)
    gamma_rate = gamma_rate >> 1;
    epsilon_rate = epsilon_rate >> 1;

    println!("Gamma rate: {}", format!("{:b}", gamma_rate));
    println!("Epsilon rate: {}", format!("{:b}", epsilon_rate));

    // println!("Epsilon rate: {}", epsilon_rate);
    // println!("binary test: {}", format!("{:b}", 8));


    // Or we could just flip bits of gamma_rate to get inverse for epsilon_rate
    //  by XOR with all 1s

    println!("Power consumption: {}", gamma_rate * epsilon_rate)


}
