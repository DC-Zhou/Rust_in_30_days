
// closures as arguments
fn do_something<F>(f:F) where F:FnOnce() {
    f();
}

pub fn ex01() {
    let some_vec = vec![1,2,3];

    do_something( || {
        some_vec.into_iter().for_each(|x| println!("the number is {}!",x));
    });
}

// use by reference
fn do_something1<F>(f:F) where F:Fn() {
    f();
}

pub fn ex02() {
    let some_vec = vec![1,2,3];

    do_something1( || {
        some_vec.iter().for_each(|x| println!("the number is {}!",x));
    });

    do_something1( || {
        some_vec.iter().for_each(|x| println!("the number is {}!",x));
    });
}

// Fn()     mean cant change value and could be repeated use
// FnOnce() mean could change value and cant be repeated use
fn takes_fnonce<F: FnOnce()>(f: F) {
    f();
}

fn takes_fnmut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn takes_fn<F: Fn()>(f: F) {
    f();
    f();
}

pub fn ex03() {
    let mut my_string = String::from("Hello More Closure");

    let prints_string = || { println!("{my_string}");  };

    takes_fn(prints_string);

    let adds_string = || { my_string.push('!'); println!("{my_string}");  };

    takes_fnmut(adds_string);

    let prints_and_drop = || { println!("Now dropping {my_string}"); drop(my_string);  };

    takes_fnonce(prints_and_drop);
}

struct City{
    name:String,
    years: Vec<u32>,
    population: Vec<u32>,
}

impl City{
    fn change_city_data<F>(&mut self, mut  f: F)
        where F: FnMut(&mut Vec<u32>, &mut Vec<u32>)
    {
        f(&mut self.years, &mut self.population);
    }
}

pub fn ex04() {
    let mut tallinn = City{
        name: "Tallinn".to_string(),
        years: vec![1972, 1564, 1665, 1780, 1800, 1950, 2020],
        population: vec![100, 200, 300, 400, 500, 600, 700],
    };

    tallinn.change_city_data(|x, y| {
        x.push(2030);
        y.push(800);
    });

    tallinn.change_city_data(|years, populations| {
        let new_vec = years
            .iter_mut()
            .zip(populations.iter_mut())
            .take(3)
            .collect::<Vec<(_,_)>>();
        println!("{:?}", new_vec);

    });

    tallinn.change_city_data(|x, y| {
       let position_option = x.iter().position(|&x| x == 2030);
        if let Some(position) = position_option {
            println!("Going to delete {} at postion {:?} now", x[position], position);
            x.remove(position);
            y.remove(position);
        }
    });

    println!("Years left are {:?} \n Populations left are {:?}", tallinn.years, tallinn.population);
}

// regular generics compared to impl trait
use std::fmt::Display;

fn prints_it_impl_trait(input: impl Display) {
    println!("Impl trait print: {}", input);
}

fn prints_it_regular_generic<T: Display>(input: T) {
    println!("regular generic print: {}", input);
}

pub fn ex05(){
    prints_it_regular_generic::<u8>(100);
    prints_it_impl_trait(100);
    prints_it_impl_trait(100u8);
    // but you can not decide type when calling the function
    // prints_it_regular_generic::<u8>(100);

}

// returning closures with impl trait
fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
    match input {
        "double" => |mut number| {
            number *= 2;
            println!("double called {}", number);
            number
        },
        "triple" => |mut number| {
            number = number * 3;
            println!("triple called {}", number);
            number
        },
        _ => |number| {
            println!("Sorry, I will return same number: {}", number);
            number
        },
    }
}

pub fn ex06(){
    let number = 10;
    let mut doubles = returns_a_closure("double");
    let mut triples = returns_a_closure("triple");
    let mut does_nothing = returns_a_closure("does_nothing");

    let doubled = doubles(number);
    let triple = triples(number);
    let does_nothing = does_nothing(number);
}

// Arc
use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn make_arc(number: i32) -> Arc<Mutex<i32>> {
    Arc::new(Mutex::new(number))
}

fn new_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
    Arc::clone(&input)
}

pub fn ex07() {
    let mut  handle_vec = vec![];
    let my_number = make_arc(0);

    for _ in 0..10 {
        let my_number_clone = new_clone(&my_number);
        let handle = spawn(move || {
            for _ in 0..10 {
                let mut value_inside = my_number_clone.lock().unwrap();
                *value_inside += 1;
            }
        });
        handle_vec.push(handle);
    }
    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    println!("{my_number:?}");

}

// channel
use std::sync::mpsc::channel;

pub fn ex08() {
    let (sender, receiver) = channel();

    let sender_clone = sender.clone();

    std::thread::spawn( move || {
        sender.send("Send a &str this time").unwrap();
        sender.send("Send a &str this time").unwrap();
    });

    std::thread::spawn( move || {
        sender_clone.send("And Here is another &str").unwrap();
        sender_clone.send("And Here is another &str").unwrap();
    });

    while let Ok(res) = receiver.recv() {
        println!("{res}");
    }
}



fn main() {
    println!("Hello, More Closure!");
    ex01();
    ex02();
    ex03();
    ex04();
    ex05();
    ex06();
    ex07();
    ex08();
}
