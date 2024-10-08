//obsługa standardowego wejścia/wyjścia
use std::{io::{stdin, stdout, Write}, process::ExitCode};

fn main() -> ExitCode {
    println!("Test 2 - reading commands");
    print!("give command: ");

    let mut command = String::new();

    let flush_result = stdout().flush();
    if flush_result.is_err()
    {
        println!("Problem with flushing output!");
        return ExitCode::from(10);
    }
    let reading_result = stdin().read_line(&mut command);

    if reading_result.is_err()
    {
        println!("Problem with reading input!");
        return ExitCode::from(1)
    }

    println!("command: {command}");

    return ExitCode::SUCCESS
}
