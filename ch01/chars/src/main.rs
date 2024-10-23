#[allow(unused_variables)]
pub fn ex01() {
    let first_letter = 'A';
    let space = ' ';
    let other_language_char = 'Ꮔ';
    let cat_face = '🐈';

    let my_number = 100 as u8;
    println!("{}", my_number as char);
    println!("{}", 101 as u8 as char);

    // println!("{}", 256 as u8 as char);
}

pub fn ex02() {
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());
    println!("Size of 𓅱: {}", "𓅱".len());
}

pub fn ex03_01() {
    let str1 = "Hello!";
    println!("str1 is {} bytes.", str1.len());
    let str2 = "안녕!";
    println!("str2 is {} bytes.", str2.len());
}

pub fn ex03_02() {
    let num1 = 1000;
    let num2 = 1000_i32;
    let big_num1  = 100_000_000_i32;
    let num1_float = 5.;
    let num2_float = 4.5;
    let num3_float :f32 = 10.5;

    let res = num1_float + num3_float as f64;

    println!("num1:i32       = {}, len = {}", num1, std::mem::size_of_val(&num1));
    println!("num2:i32       = {}, len = {}", num2, std::mem::size_of_val(&num2));
    println!("big_num1:i32   = {}, len = {}", big_num1, std::mem::size_of_val(&big_num1));
    println!("num1_float:f64 = {}, len = {}", num1_float, std::mem::size_of_val(&num1_float));
    println!("num2_float:f64 = {}, len = {}", num2_float, std::mem::size_of_val(&num2_float));
    println!("num3_float:f32 = {}, len = {}", num3_float, std::mem::size_of_val(&num3_float));
    println!("res       :f64 = {}, len = {}", res, std::mem::size_of_val(&res));
}

pub fn ex04(){
    let num_float = 3.1415926;
    // {} lifetime
    {
        let num_float = 3.14;
        println!("num_float: {}", num_float);
    }
    println!("num_float: {}", num_float);
}

fn get_param() -> i32{
    100
}

pub fn ex05_01() {
    let num = get_param();
    println!("getParm: num: {}", num);
}

fn get_num() -> i32 {
    return 100;
    // unreachable expression
    80
}

pub fn ex05_02() {
    let num = get_num();
    println!("getNum by return: num: {}", num);
}

fn get_num_v2() -> i32 {
    80; // use in statements
    return 100;
}

pub fn ex05_03() {
    let num = get_num_v2();
    println!("getNum by return: num: {}", num);
}

fn multiply(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    result
}

pub fn ex06() {
    let num = multiply(3, 4);
    println!("num: {num}");
}

pub fn ex07() {
    let color1= String::from("red");
    let color2= String::from("green");
    let color3= String::from("blue");

    println!("color1: {}, color2: {}, color3: {}", color1, color2, color3);
}

pub fn ex07_01() {
    let str1 = "google";
    let str2 = "youtube";
    let str3 = "rust";

    println!("google  web: www.{str1}.com");
    println!("youtube web: www.{str2}.com");
    println!("rust    web: www.{str3}.com");
}

pub fn ex08() {
    let num = {
        // 带有；的就是语句，语句没有返回值
        let res = 8;
        // 不带有；的就是表达式，表达式必然有返回值
        res + 9
    };
    println!("num = {}", num);
}

pub fn ex09() {
    println!("the smallest i8:  {}, the biggest i8:  {}", std::i8::MIN, std::i8::MAX);
    println!("the smallest i16: {}, the biggest i16: {}", std::i16::MIN, std::i16::MAX);
    println!("the smallest u16: {}, the biggest u16: {}", std::u16::MIN, std::u16::MAX);
    println!("the smallest i32: {}, the biggest i32: {}", std::i32::MIN, std::i32::MAX);
    println!("the smallest u32: {}, the biggest u32: {}", std::u32::MIN, std::u32::MAX);
    println!("the smallest i64: {}, the biggest i64: {}", std::i64::MIN, std::i64::MAX);
    println!("the smallest u64: {}, the biggest u64: {}", std::u64::MIN, std::u64::MAX);

    println!("the smallest i128: {}, the biggest i128: {}", std::i128::MIN, std::i128::MAX);
    println!("the smallest u128: {}, the biggest u128: {}", std::u128::MIN, std::u128::MAX);
}

// shadow
pub fn ex10() {
    let x = 9;

    println!("x:i32  = {}", x);
    println!("x size = {}", std::mem::size_of_val(&x));

    let mut x = x as f32;
    x *= 10.0;

    println!("x:f32  = {}", x);
    println!("x size = {}", std::mem::size_of_val(&x));
}

fn main() {
    ex10();
}
