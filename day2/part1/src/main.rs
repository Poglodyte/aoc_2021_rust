use std::fs;
use std::str::Lines;


fn print_position(position: (i32, i32)) {
    println!("Position - Horizontal = {}, Depth = {}",
             position.0, position.1);
}

fn apply_command(command: &str, position: (i32, i32)) -> (i32, i32) {

    let mut move_type = "";
    let mut move_num= 0;

    let command_parts = command.split_ascii_whitespace();
    for (i, part) in command_parts.enumerate() {
        if i == 0 {
            move_type = part;
        }
        else if i == 1 {
            move_num = part.parse().unwrap();
        }
    }

    return if move_type.eq("forward") {
        // println!("go forward by {}", move_num);
        (position.0 + move_num, position.1)
    } else if move_type.eq("up") {
        // println!("go up by {}", move_num);
        (position.0, position.1 - move_num)
    } else if move_type.eq("down") {
        // println!("go down by {}", move_num);
        (position.0, position.1 + move_num)
    } else {
        println!("Invalid command");
        // TODO: Throw error instead?
        position
    }
}

fn process_commands(input_commands: Lines, start_position: (i32, i32)) -> (i32, i32) {

    let mut position = start_position;

    for command in input_commands {
        // println!("{}", command);
        position = apply_command(command, position);
        // print_position(position);
    }

    return position;
}


fn main() {

    let start_position = (0, 0);

    // print_position(start_position);

    let input_commands = fs::read_to_string("../input")
        .expect("Couldn't read input file.");

    let (final_horizontal_pos, final_depth) = process_commands(input_commands.lines(), start_position);

    println!("Solution: {}", final_horizontal_pos * final_depth);

}
