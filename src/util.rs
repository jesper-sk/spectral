use std::{fmt::Debug, io::Write, str::FromStr};

use itertools::Itertools;

pub fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read user input");
    buffer.truncate(buffer.trim_end().len());
    return buffer;
}

pub fn input<T: FromStr<Err: Debug> + Copy>() -> T {
    input_validated(|_| true)
}

pub fn input_validated<T, F>(validate: F) -> T
where
    T: FromStr<Err: Debug> + Copy,
    F: Fn(T) -> bool + Copy,
{
    loop {
        print!("Choose> ");
        std::io::stdout().flush().unwrap();
        let value = read_line();
        match value.parse::<T>() {
            Ok(value) => {
                if !validate(value) {
                    println!("Validation error");
                    continue;
                }
                return value;
            }
            Err(err) => {
                println!("Can't parse: {:?} (in: '{}')", err, value);
                continue;
            }
        }
    }
}

pub fn print_list<T: Debug>(list: &Vec<T>) -> String {
    list.iter()
        .enumerate()
        .map(|(i, host)| format!("{:2} {:?}", i, host))
        .join("\n")
}
