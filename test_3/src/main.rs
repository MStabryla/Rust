use std::{i32, io::{stdin,stdout, Write}, num::ParseIntError, process::ExitCode};

fn main() -> ExitCode {
    println!("Test 3 - get number input nad operate.");
    print!("Insert Number: "); let _ = stdout().flush();

    let mut prev_number : i32;
    let mut number: i32;
    let mut command = String::new();

    let get_command_response = stdin().read_line(&mut command);
    let parse_command_response =  command.trim().parse::<i32>();

    if get_command_response.is_err()
    {
        println!("Error with getting command!");
        return ExitCode::from(1)
    }
    else if parse_command_response.is_err()
    {
        println!("Error with parsing number!");
        let err : ParseIntError = parse_command_response.err().unwrap(); 
        println!("{err}");
        return ExitCode::from(2)
    }

    number = parse_command_response.unwrap();
    println!("Received number {number}");
    prev_number = number; number = number + 15;
    println!("{prev_number} + 15 = {number}");
    prev_number = number; number = number - 13;
    println!("{prev_number} - 13 = {number}");
    prev_number = number; number = number * 5;
    println!("{prev_number} * 5 = {number}");
    prev_number = number; number = number / 2;
    println!("{prev_number} / 2 = {number}");
    prev_number = number; number = number % 3;
    println!("{prev_number} % 3 = {number}");

    let float_number: f32;
    prev_number = number; float_number = (number as f32) / 0.55;
    println!("{prev_number} / 0.55 = {float_number}");

    return ExitCode::SUCCESS
}