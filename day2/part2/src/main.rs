use std::fs;
use std::str::Lines;

#[derive(Clone, Copy)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32
}

fn print_position(position: Position) {
    println!("Position: Horizontal = {}, Depth = {}, Aim = {}",
             position.horizontal, position.depth, position.aim);
}

// Increase HORIZONTAL_POS by X units, and
// Increase DEPTH by AIM * X
fn apply_forward(mut position: Position, x: i32) -> Position {
    position.horizontal += x;
    position.depth += position.aim * x;
    return position;
}

// Decrease AIM by X units
fn apply_up(mut position: Position, x: i32) -> Position {
    position.aim -= x;
    return position;
}

// Increase AIM by X units
fn apply_down(mut position: Position, x: i32) -> Position {
    position.aim += x;
    return position;
}

fn apply_command(command: &str, position: Position) -> Position {

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

    let mut new_position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0
    };

    if move_type.eq("forward") {
        new_position = apply_forward(position, move_num);
    } else if move_type.eq("up") {
        new_position = apply_up(position, move_num);
    } else if move_type.eq("down") {
        new_position = apply_down(position, move_num);
    } else {
        println!("Invalid command");
        // TODO: Throw error instead?
    }

    return new_position;

}

fn process_commands(input_commands: Lines, mut position: Position) -> Position {

    for command in input_commands {
        position = apply_command(command, position);
        // print_position(position);
    }

    return position;
}


fn main() {

    let position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0
    };

    print_position(position);

    let input_commands = fs::read_to_string("../input")
        .expect("Couldn't read input file.");

    let final_position = process_commands(input_commands.lines(), position);

    println!("Solution: {}", final_position.horizontal * final_position.depth);

}
