use std::fs;

fn main() {
    println!("Hello, world!");

    let sonar_report = fs::read_to_string("./input")
        .expect("Couldn't read the file.");

    let lines = sonar_report.lines();

    let mut num_increased = 0;

    let mut previous: i32 = -1;

    for line in lines {

        println!("{}", line);
        
        let current: u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                break;
            }
        };

        if previous != -1 {
            if current > previous.try_into().unwrap() {
                println!("depth increased");
                num_increased += 1;
            }
            else {
                println!("depth did not increase")
            }
        }

        previous = current.try_into().unwrap();
    }

    println!("Number of measurements that increased: {}", num_increased);



}
