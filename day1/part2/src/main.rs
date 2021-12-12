use std::fs;
use std::io::{ErrorKind, Error};

fn main() {

    let window_size = 3;

    let sonar_report = fs::read_to_string("../input")
        .expect("Couldn't read the file.");

    let lines = sonar_report.lines();


    let mut sonar_readings: Vec<u32> = vec![];
    for line in lines {
        // TODO: Use match on parse to handle error like -
        //     let current: u32 = match line.trim().parse() {
        //         Ok(num) => num,
        //         Err(err) => {
        //             println!("{}", err);
        //             break;
        //         }
        //     };
        sonar_readings.push(line.trim().parse().unwrap());
    }


    let mut num_increased = 0;

    let mut previous_sum: i32 = -1;

    // println!("{}", lines.count());

    for (index, value) in sonar_readings.iter().enumerate() {
        // println!("index: {}, value: {}", index, value);

        // Only operate on elements until window exceeds array bounds
        if sonar_readings.len() - index >= window_size {
            println!("index: {}, value: {}", index, value);
            let mut current_sum = 0;
            for n in 0..window_size {
                current_sum += sonar_readings[index + n];
            }
            println!("sum: {}", current_sum);

            // If not the first sum, compare with previous
            if previous_sum != -1 {
                if current_sum > previous_sum as u32 {
                    num_increased += 1;
                }
            }

            previous_sum = current_sum as i32;

        }

    }

    println!("Number of measurements that increased: {}", num_increased);

}