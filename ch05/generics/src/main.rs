
// generic test
#[warn(dead_code)]
fn return_i32(num: i32) -> i32 {  num }
fn return_u32(num: u32) -> u32 {  num }
fn return_u8(num: u8)   -> u8  {  num }

fn return_item<T>(num: T) -> T {
    println!("Here is your item.");
    num
}

fn ex01() {
    let num = return_item(5);
    println!("The value of num is: {}", num);
}

// when print generic, must use
use std::fmt::{Debug, Display};

fn print_generic<T: Debug>(item: T) {
    println!("The value of item is: {:?}", item);
}

pub fn ex02() {
    print_generic(return_item(16));
}

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

pub fn ex03() {
    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 20,
    };

    let num = 55;
    print_generic(num);
    print_generic(charlie);
}

// use two
use std::cmp::PartialOrd;

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, input_1: U, input_2: U)
{
    println!("The value of statement is: {}", statement);
    println!("The value of input_1 is: {}", input_1);
    println!("The value of input_2 is: {}", input_2);
    println!("Is {input_1} greater than {input_2} ? {} ", input_1 > input_2);
}

pub fn ex04() {
    compare_and_display("Listen up!", 4, 5);
}

//
fn compare_and_display_2<T, U>(statement: T, input_1: U, input_2: U)
    where
        T: Display,
        U: Display + PartialOrd,
{
    println!("The value of statement is: {}", statement);
    println!("The value of input_1 is: {}", input_1);
    println!("The value of input_2 is: {}", input_2);
    println!("{statement}, Is the {input_1} > {input_2}? {}", input_1 > input_2);
}

pub fn ex05() {
    compare_and_display_2("Listen up!", 4, 5);
}

// Option
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

pub fn ex06() {
    let small = vec![1, 2, 3];
    let large = vec![6,5,4,3,2,1];
    for vec in vec![small, large] {
        let inside_num = take_fifth(vec);
        if inside_num.is_some() {
            println!("{}", inside_num.unwrap());
        } else {
            println!("Cant got anything.");
        }
    }
}

// Option -> .is_some() / .is_none()
// Result -> .is_ok() / .is_err()
fn see_if_num_is_even(input: i32) -> Result<(), ()> {
    return if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn ex07() {
    if see_if_num_is_even(5).is_ok() {
        println!("Its even number");
    } else {
        println!("Cant get even number");
    }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(5),
        _ => Err("This number is not five!".to_string()),
    }
}

pub fn ex08() {
    for number in 4..=7 {
        println!("The number is {:?}", check_if_five(number));
    }
}

// pattern match by let else
pub fn ex09() {
    let my_vec = vec![1, 2, 3, 4, 5];

    for num in 3..10 {
        if let Some(number) = my_vec.get(num) {
            println!("The number is {}", number);
        }
        let Some(number) = my_vec.get(num) else {
            continue;
        };
        println!("The number is {}", number);
    }
}

// pattern match by while else
pub fn ex10() {
    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the City of {} :", city[0]);
        while let Some(information) = city.pop() {
            if let Ok(number) = information.parse::<i32>() {
                println!("The city number is {}", number);
            }
        }
    }
}



fn main() {
    println!("Generic, Optional, Result in rust!");
    // ex01();
    // ex02();
    // ex03();
    // ex04();
    // ex05();
    // ex06();
    // ex07();
    // ex08();
    // ex09();
    ex10();
}
