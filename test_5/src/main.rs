use std::{ io::{stdin, stdout, Write}, process::ExitCode, vec::Vec};


fn split(text_to_split: String) -> Vec<String> {
    let text_len = text_to_split.len();
    let mut text_i: usize = 0;
    let mut text_vector: Vec<String> = Vec::new();
    let mut text_split_string;

    while text_i < text_len {
        if text_i < text_len - 1{
            text_split_string = text_to_split[text_i..(text_i+2)].to_string();
        }
        else{
            text_split_string = [text_to_split.chars().nth(text_i).unwrap(),"_".chars().nth(0).unwrap()].iter().collect();
        }
        text_vector.insert(text_i / 2, text_split_string);
        text_i += 2;
    }
    return text_vector;
}


fn main() -> ExitCode {
    println!("Test 5 - Codewars task Split Strings");
    print!("Insert String: "); let _ = stdout().flush();

    let mut text_to_split: String = String::new();
    let result = stdin().read_line(&mut text_to_split);

    match result{
        Err(e) => {
            println!("Error with reading line!\n{e}");
            return ExitCode::FAILURE
        },
        Ok(_v) => {
            print!("Inserted text: {text_to_split}");
            text_to_split = text_to_split.trim().to_string();
        }
    }

    let text_vector: Vec<String> = split(text_to_split);
    text_to_split = text_vector.join("] [");
    print!("Splitted: [{}]",text_to_split);

    return ExitCode::SUCCESS
}