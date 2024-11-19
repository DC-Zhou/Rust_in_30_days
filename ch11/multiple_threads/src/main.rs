
// rename and include module
enum FileState {
    CannotAccessFile,
    FileIsBroken,
    NoSuchFileExists,
    FileOpenedAndReady,
    FoundSimilarFileNameInSameDirectory,
}

fn get_file_state(input: &FileState) {
    use FileState::{
      CannotAccessFile as CantAccess,
      FileIsBroken     as Broken,
      NoSuchFileExists as NoExists,
      FileOpenedAndReady  as OpenAndReady,
      FoundSimilarFileNameInSameDirectory as Similar,
    };

    match input {
        &CantAccess => println!("Cant Access!"),
        &Broken => println!("File is Broken!"),
        &NoExists => println!("No such file exists"),
        &OpenAndReady => println!("Opened and ready and read"),
        &Similar => println!("Similar file name in same directory"),
    }
}

pub fn ex01()
{
    println!("File state name_as...");
}

// use todo! tell compiler to wait
struct Book;

enum BookType {
    HardCover,
    SoftCover,
}

fn get_book(book: &Book) -> Option<String> {
    todo!();
}

fn delete_book(book: &Book) -> Result<(), String> {
    todo!();
}

fn check_book_types(book_type: &BookType) {
    match book_type {
        BookType::HardCover => println!("This book have HardCover"),
        BookType::SoftCover => println!("This book have SoftCover"),
    }
}

pub fn ex02() {
    let book_type = BookType::HardCover;
    check_book_types(&book_type);
}

use std::borrow::Cow;
use std::fmt::format;

#[derive(Debug)]
struct ErrorInfo{
    error: LocalError,
    message: String,
}

#[derive(Debug)]
enum LocalError {
    TooBig,
    TooSmall,
}

fn generate_message(message: &'static str, error_info: Option<ErrorInfo>) -> Cow<'static, str> {
    match error_info {
        None => message.into(),
        Some(info)=>format!("{message}: {info:?}").into(),
    }
}

pub fn ex03() {
    let msg1 = generate_message(
        "Everything is fine",
        None,
    );

    let msg2 = generate_message(
        "Got an error",
        Some(ErrorInfo {
            error: LocalError::TooBig,
            message: "Too Big".to_string(),
        }),
    );

    for msg in [msg1, msg2] {
        match msg {
            Cow::Borrowed(msg) => {
                println!("Borrowed, did not need an allocation: \n {msg} ")
            }
            Cow::Owned(msg) => {
                println!("Owned, did not need an allocation: \n {msg} ")
            }
        }
    }
}

// cow:: borrow and owned
struct User {
    name: Cow<'static, str>,
}

pub fn ex04() {
    // borrow use in 'static
    let user_name1 = "User1";
    // own use in another ot when it changed
    let usr_name2 = "User2".to_string();

    let usr1 = User{
        name: user_name1.into(),
    };

    let usr2 = User{
        name: usr_name2.into(),
    };

    for name in [usr1.name, usr2.name] {
        match name {
            Cow::Borrowed(n) => {
                println!("Borrowed, did not need an allocation: \n {n:?}")
            }
            Cow::Owned(n) => {
                println!("Owned, did not need an allocation: \n {n:?}")
            }
        }
    }
}

// use Rc to Reference Count
use std::rc::Rc;

struct City{
    name: Rc<String>,
    population:u32,
    city_history: Rc<String>,
}

struct CityData{
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>,
}

pub fn ex05() {
    let calgary_name = Rc::new("Calgary".to_string());
    let calgary_history         = Rc::new("CalgaryHistory".to_string());

    let calgary = City{
        name: Rc::clone(&calgary_name),
        population: 1_200_000,
        city_history: Rc::clone(&calgary_history),
    };

    let canada_city = CityData{
        names: vec![Rc::clone(&calgary_name)],
        histories: vec![Rc::clone(&calgary_history)],
    };

    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary_name));
}


fn main() {
    println!("Hello, world!");
    ex01();
    ex02();
    ex03();
    ex04();
    ex05();
}
