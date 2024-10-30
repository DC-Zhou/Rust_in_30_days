
// HashMap and BTreeMap
use std::collections::{BTreeSet, HashMap};

struct City{
    name: String,
    population: HashMap<i32, i32>,
}

pub fn ex01() {
    let mut tallinn = City{
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(2020, 123_456);
    tallinn.population.insert(2021, 456_789);
    tallinn.population.insert(2022, 789_001);

    println!("City name is: {}", tallinn.name);

    for(year, population) in &tallinn.population {
        println!("year: {}, population: {}", year, population);
    }
}

use std::collections::BTreeMap;

struct City2 {
    name : String,
    population: BTreeMap<i32, i32>,
}

pub fn ex02() {
    let mut tallinn = City2 {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallinn.population.insert(2020, 123_456);
    tallinn.population.insert(2021, 456_789);
    tallinn.population.insert(2022, 789_001);

    println!("City name is: {}", tallinn.name);

    for(year, population) in &tallinn.population {
        println!("year: {}, population: {}", year, population);
    }

}

//
pub fn ex03() {
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    let key = 1;
    match book_hashmap.get(&key) {
        Some(val) => println!("Key {key} has a value already: {val}"),
        None => {
            book_hashmap.insert(key, "Le Petit Prince");
        }
    }
    println!("{:?}", book_hashmap.get(&1));
}

pub fn ex04() {
    let mut book_hashmap = HashMap::new();
    let mut moved_hashmap_vec = Vec::new();

    let hashmap_entries = [
        (1, "L'Allemagne Moderne"),
        (1, "Le Petit Prince"),
        (1, "섀도우 오브 유어 스마일"),
        (1, "Eye of the world"),
    ];

    for (key, value) in hashmap_entries {
        if let Some(old_val) = book_hashmap.insert(key, value) {
            println!("Overwriting {old_val} with {value} !");
            moved_hashmap_vec.push(old_val);
        }
    }
    println!("All moved values: {moved_hashmap_vec:?}");
}

//
pub fn ex05() {
    let book_collection = vec! [
      "L'Allemagne Moderne",
      "Le Petit Prince",
      "Eye of the world",
      "Eye of the world",
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }

    for (book, true_or_false) in book_hashmap {
        println!("Do we have {book} ? {true_or_false}");
    }
}

// HashSet use to keep unique key
use std::collections::HashSet;
pub fn ex06() {
    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33, ];

    println!("How many numbers in the Vec? {}", many_numbers.len());

    let mut number_hashset = HashSet::new();

    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();

    println!("There are {hashset_length} unique numbers, so we are missing {}.", 50 - hashset_length);

    println!("It does not contain: ");
    for number in 0..=50 {
        if number_hashset.get(&number).is_none() {
            print!("{number} ");
        }
    }
}

// BTreeSet
pub fn ex07() {
    let many_numbers = vec![37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
                                20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
                                46, 33];
    let mut current_number = i32::MIN;
    let mut number_set = BTreeSet::new();

    for number in many_numbers {
        number_set.insert(number);
    }

    for number in number_set {
        if number > current_number {
            println!("This will never happen !");
        }
        current_number = number;
    }
}

// BinaryHeap will keep the largest number in first, and unordered for other numbers.
use std::collections::BinaryHeap;

pub fn ex08() {
    let many_numbers = vec![0, 5, 10, 20, 30, 35, 20, 5];
    let mut heap = BinaryHeap::new();
    for num in many_numbers {
        heap.push(num);
    }

    println!("First item is largest, others are out of oder: {heap:?}");

    while let Some(num)  = heap.pop() {
        println!("Poped off {num}. Remaining numbers are: {heap:?}");
    }
}

// use BinaryHeap for important of task
pub fn ex09() {
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "Reply E-mail"));
    jobs.push((50,  "Finish report"));
    jobs.push((70,  "Finish PPT "));
    jobs.push((30,  "Watch Video"));

    for(_, job) in jobs {
        println!("You need to: {job}");
    }
}

// VecDeque
use std::collections::VecDeque;

pub fn ex10() {
    let mut vec1 = VecDeque::from(vec![1, 2, 3, 4, 5]);
    let mut vec2 = vec1.clone();
    vec1.remove(0);
    println!("vec1 after remove:\t{vec1:?}");
    vec2.pop_front();
    println!("vec2 after pop_front:\t{vec2:?}");
}

// use ? to replace Ok(()) and when error output Err
use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input
        .parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<i32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}

pub fn ex11() {
    let str_vec = vec!["seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{parsed:?}");
    }
}

// panic

fn main() {
    println!("Hello, Rust More collections and error handling!");
    // ex01();
    // ex02();
    // ex03();
    // ex04();
    // ex05();
    // ex06();
    // ex07();
    // ex08();
    // ex09();
    // ex10();
    ex11();
}
