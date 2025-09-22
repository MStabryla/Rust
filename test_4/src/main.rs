
use std::{env, fs::File, io::{BufRead, BufReader}, process::ExitCode};

fn get_first_argumet() -> Option<String> {
    let args = env::args();
    for arg in args {
        let arg_extention = String::from(&arg[(arg.len() - 3)..]);
        // println!("ARG {}",arg);
        // println!("ARG_EXTENTION {}",arg_extention);
        // println!("ARG_EXTENTION_COMPARISON {}",arg_extention == "exe");
        if arg.len() > 0 && arg_extention != "exe" {
            return Some(arg);
        }
    }
    return None;
}

fn main() -> ExitCode {
    
    println!("Reading file ...");

    let file_arg = get_first_argumet();
    let file_name;
    if file_arg == None {
        file_name = "test.tsv".to_string();
    }
    else {
        file_name = file_arg.unwrap();
    }
    let test_file = File::open(&file_name);
    if test_file.is_err()  {
        println!("Problem with reading file {}!",&file_name);
        let reading_file_err = test_file.err().unwrap();
        println!("{reading_file_err}");
        return ExitCode::from(1);
    } 
    let test_file_buffer = BufReader::new(test_file.unwrap());
    // let mut text_line = String::new();
    //Reading first line
    let read_iterator = test_file_buffer.lines();
    for t in read_iterator {
        match t {
            Ok(v) => println!("{v}"),
            Err(_e) => println!("END")
        }
    }
    // while text_line.len() > 0 && !read_status.is_err() {
    //     println!("{text_line}");
    //     test_file_buffer.read_to_string(buf)
    //     read_status = test_file_buffer.read_line(&mut text_line);
    // }

    return ExitCode::SUCCESS;
}
