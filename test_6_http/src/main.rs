
use std::{ io::{stdin, stdout, Write}, process::ExitCode};
use reqwest::{blocking::{ Client}};
use http::{Method, Response};

struct SimpleData {
    number: i32,
    text: String
}

impl From<String> for SimpleData {
    fn from(value: String) -> Self {
        if value.len() < 1 {
            return SimpleData {
                number: 0,
                text: "".to_string()
            };
        }
        
        return SimpleData {
            number: value.chars().nth(0).unwrap() as i32,
            text: value.get(1..).unwrap().to_string()
        }
    }
}

fn loc_get<T>(url: String) -> Result<Response<T>, reqwest::Error>
where T: From<String>
{   

    let client = Client::builder().build().unwrap();
    let request = client.request(Method::GET, url).build().unwrap();
    let response: Result<reqwest::blocking::Response,reqwest::Error> = client.execute(request);
    match response {
        Ok(r) => {
            return Result::Ok(Response::new(T::from(r.text().unwrap())));
        },
        Err(_e) => {
            return Result::Err(_e);
        }
    }
}

fn main() -> ExitCode {
    println!("Execute Request");
    print!("path: "); let _ = stdout().flush();
    let mut url_path = String::new();

    match stdin().read_line(&mut url_path) {
        Ok(_v) => {},
        Err(_e) => {
            println!("Error with processing url");
            return ExitCode::FAILURE;
        }
    }
    println!("DEBUG: Inserted path: {}",url_path);

    let loc_response = loc_get::<SimpleData>(url_path);
    match loc_response {
        Ok(v) => {
            println!("Request executed");
            let value = v.body();
            println!("number -> {}\ntext -> \"{}\"",value.number,value.text);
        },
        Err(_e) => {
            println!("ERROR: request failed\n{}",_e);
            return ExitCode::FAILURE;
        }
    }

    return ExitCode::SUCCESS;

}