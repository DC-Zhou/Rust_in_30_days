// Box in Rust
// fn just_takes_a_variable<T>)(item: T) {}
//
//
// pub fn test2() {
//     let my_num = 1;
//     just_takes_a_variable(my_num);
//     just_takes_a_variable(my_num);
//
//     let my_box = Box::new(1);
//
//     just_takes_a_variable(my_box.clone());
//     just_takes_a_variable(my_box);
// }

use std::error::Error;
use std::fmt;
use std::fmt::{write, Formatter};

#[derive(Debug)]
struct ErrorOne;

impl  Error for ErrorOne { }

impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result    {
        write!(f, "You got the first error!")
    }
}

#[derive(Debug)]
struct ErrorTwo;

impl Error for ErrorTwo{}

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
    match input {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine to me".to_string()),

    }
}

pub fn test1() {
    let vec_of_u8s = vec![0_u8, 1, 70];

    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{}", message),
        }
    }
}

use std::sync::mpsc::RecvError;

enum MyError {
    TooMuchStuff,
    CantConnect,
    NoUserRegistered,
    SomethingElse,
}

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Wouldn't you like to know...")
    }
}

impl fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LOL not telling you what went wrong").finish()
    }
}

fn give_error_back(is_true:bool) -> Box<dyn Error> {
    if is_true {
        Box::new(MyError::TooMuchStuff)
    } else {
        Box::new(RecvError)
    }
}

pub fn test2() {
    let errs = [true, false, false, true].into_iter().map(|boolean| give_error_back(boolean))
        .collect::<Vec<_>>();
}


fn main() {
    println!("Hello, Box and documentations !");
    test1();
    test2();
}
