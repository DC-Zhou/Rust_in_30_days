
// lifetimes marks
fn return_str() -> &'static str {
    let my_string = String::from("hello world");
    "hello Rust lifetimes marks"
}

pub fn ex01() {
    let my_str = return_str();
    println!("{}", my_str);
}

// use in struct
#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    date_founded: u32,
}

pub fn ex02() {
    let city_name = vec!["kunming".to_string(), "beijing".to_string()];

    let my_city = City{
        name: &city_name[0],
        date_founded: 1949,
    };

    println!("{} was founded in {} ", my_city.name, my_city.date_founded);
}

// anonymous lifetime
struct Adventurer<'a> {
    name: &'a str,
    health_point: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.health_point -= 3;
        println!("{} has {} hit points left!", self.name, self.health_point);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} has {} heath points left", self.name, self.health_point)
    }
}

pub fn ex03() {
    let mut billy = Adventurer {
        name: "Billy",
        health_point: 30,
    };
    println!("{}", billy);
    billy.take_damage();
}

// interior mutability
// cell
use std::cell::{Cell, Ref};

#[derive(Debug)]
struct PhoneInfoinCell {
    company_name:String,
    model_name:String,
    screen_name:f32,
    memory:usize,
    date_issued:u32,
    on_sale: Cell<bool>,
}

impl PhoneInfoinCell {
    fn make_not_on_sale(&self)
    {
        self.on_sale.set(false);
    }
}

pub fn ex04() {
    let iphone_15 = PhoneInfoinCell{
        company_name: "Apple".to_string(),
        model_name: "iphone15 pro max".to_string(),
        screen_name: 8.0,
        memory: 32,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };

    iphone_15.make_not_on_sale();

    println!("{iphone_15:#?}");
}

// RefCell
use std::cell::RefCell;

struct PhoneInfoRefCell{
    company_name:String,
    model_name:String,
    screen_name:f32,
    memory:usize,
    date_issued:u32,
    on_sale: RefCell<bool>,
}

pub fn ex05() {
    let iphone_15 = PhoneInfoRefCell{
        company_name: "Apple".to_string(),
        model_name: "iphone15 pro max".to_string(),
        screen_name: 8.0,
        memory: 32,
        date_issued: 2020,
        on_sale: RefCell::new(true),
    };

    println!("use RefCell: Does iphone 15 on sale: {:?}", iphone_15.on_sale);

    *iphone_15.on_sale.borrow_mut() = false;

    println!("use RefCell: Does iphone 15 on sale: {:?}", iphone_15.on_sale);

    // 1.only use borrow_mut without assigning
    let borrow_1 = iphone_15.on_sale.borrow_mut();
    // panicked! let borrow_2 = iphone_15.on_sale.borrow_mut();
    // 2. use try_borrow_mut()
    let borrow_3 = iphone_15.on_sale.try_borrow_mut();
    // be careful, RefCell cannot be shared between threads safely.

}

// Mutex in thread safely
use std::sync::Mutex;

#[derive(Debug)]
struct PhoneInfoMutex{
    company_name:String,
    model_name:String,
    screen_name:f32,
    memory:usize,
    date_issued:u32,
    on_sale: Mutex<bool>,
}

pub fn ex06() {
    let iphone_15 = PhoneInfoMutex{
        company_name: "Apple".to_string(),
        model_name: "iphone15 pro max".to_string(),
        screen_name: 8.0,
        memory: 32,
        date_issued: 2020,
        on_sale: Mutex::new(true),
    };

    println!("useMutex: Does iphone 15 on sale: {:?}", iphone_15.on_sale);

    *iphone_15.on_sale.lock().unwrap() = false;

    println!("useMutex: Does iphone 15 on sale: {:?}", iphone_15.on_sale);

}

// check if get mutex lock
pub fn ex06_1(){
    let _my_mutex = Mutex::new(5);
    let mut _mutex_changer = _my_mutex.lock().unwrap();

    *_mutex_changer = 6;

    let mut _other_mutex_changer = _my_mutex.try_lock();

    if let Ok(value) = _other_mutex_changer {
        println!("The MutexGuard has: {value}")
    } else {
        println!("Did not get the lock")
    }

    drop(_mutex_changer);

    let mut _other_mutex_changer_2 = _my_mutex.try_lock();

    if let Ok(value) = _other_mutex_changer_2 {
        println!("The MutexGuard has: {value}")
    } else {
        println!("Did not get the lock")
    }
}

// RwLock
use std::sync::RwLock;
#[derive(Debug)]
struct PhoneInfoRwLock{
    company_name:String,
    model_name:String,
    screen_name:f32,
    memory:usize,
    date_issued:u32,
    on_sale: RwLock<bool>,
}

pub fn ex07() {
    let _iphone_15 = PhoneInfoRwLock{
        company_name: "Apple".to_string(),
        model_name: "iphone 15".to_string(),
        screen_name: 8.0,
        memory: 32,
        date_issued: 2020,
        on_sale: RwLock::new(true),
    };

    let read_1= _iphone_15.on_sale.read().unwrap();
    let read_2= _iphone_15.on_sale.read().unwrap();
    println!("{read_1:#?}");
    println!("{read_2:#?}");

    // forRwlock
    drop(read_1);
    drop(read_2);

    let mut changer = _iphone_15.on_sale.write().unwrap();

    *changer = false;

    // only use drop for read / write it would be right
    drop(changer);

    let read_3= _iphone_15.on_sale.read().unwrap();

    println!("{read_3:#?}");


    // if let Ok(value) = _iphone_15.on_sale.try_write(){
    //     let mut _write = _iphone_15.on_sale.write().unwrap();
    //     *_write = false;
    //     println!("use RwLock: Does iphone 15 on sale: {:?}", _iphone_15.on_sale);
    // } else {
    //     println!("Did not get the lock");
    // }
    //
}





fn main() {
    println!("Hello, Rust Lifetimes!");
    ex01();
    ex02();
    ex03();
    ex04();
    ex05();
    ex06();
    ex06_1();
    ex07();
}
