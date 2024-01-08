use std::fs::read_to_string;

pub fn get_codes(path: &str){
    let file_content = read_to_string(path);
    let mut sum: i32 = 0;
    
    for line in file_content.unwrap().lines() {
       let code = get_code_from_string(line);
       match code {
        Some(number) => {
            sum += i32::from(number);
        },
        None => panic!("did not find any number for line {line}")
       }
    }

    println!("{sum}");
}

fn get_code_from_string(line: &str) -> Option<u8>{
    let first_digit = line
    .chars()
    .into_iter()
    .find(|&x| x.is_ascii_digit())?;

    let last_digit = line.chars().rev().find(|&x| x.is_ascii_digit())?;
    
    let number = format!("{}{}", first_digit, last_digit).parse::<u8>();
    match number{
        Ok(result) => Some(result),
        Err(_) => {
            panic!("failed to parse {line}, found following digits: {first_digit} and {last_digit}");
        }
    }



}