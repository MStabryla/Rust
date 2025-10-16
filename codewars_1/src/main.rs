/// PROBLEM WITH IMPORTING bitvec
// extern crate bitvec;
// use bitvec::{array::BitArray, order::Lsb0};

use std::{ io::{stdin, stdout,  Write}, process::ExitCode};


fn count_bits(n: i64) -> u32 {
    let bytes_amount = i64::count_ones(n);

    // PROBLEM WITH IMPORTING bitvec
    // let mut bytes_array_string = String::new();
    // let bit_arr = BitArray::<u64,Lsb0>::new(n as u64);

    // for i in &bit_arr[..(bit_arr.len()-2)] {
    //     bytes_array_string.push_str(&i.to_string());
    //     bytes_array_string += ","
    // }
    // bytes_array_string.push_str(&bit_arr[bit_arr.len()-1].to_string());    
    // println!("[{bytes_array_string}]");
    
    let mut bytes_array_string = format!("{:b}",n);
    let byte_array_chars : Vec<String> = bytes_array_string.chars().map(|x| x.to_string()).collect();
    bytes_array_string = byte_array_chars.join(",");
    println!("Byte array of a number: [{bytes_array_string}]");
    return bytes_amount
}

fn main() -> ExitCode {
    println!("Test 6 - Codewars task BitCounting");
    let mut number_string = String::new();
    let number: i64;

    // let mut stdout_main = stdout().lock();
    // stdout_main.write_all(b"Give a number: ").unwrap();
    // let test = stdout_main.flush().unwrap();
    print!("Give a number: ");
    stdout().flush().unwrap();
    let read_response = stdin().read_line(&mut number_string);
    let convert_response = number_string.trim().parse::<i64>();


    match read_response.is_err() | convert_response.is_err() {
        true => {
            println!("Error with receiving a number!");
            return ExitCode::FAILURE;
        },
        false => {
            number = convert_response.unwrap();
            let byte_count = count_bits(number);
            println!("Bytes in {number} = {byte_count}");
            return ExitCode::SUCCESS;
        }
    }
}