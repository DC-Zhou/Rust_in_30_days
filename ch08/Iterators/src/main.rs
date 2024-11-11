
// iterator in normal
pub fn ex01() {
    let mut new_vec = Vec::new();
    let mut counter = 1;
    loop {
        new_vec.push(counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    println!("{:?}", new_vec);
}

// in a iterator ways
pub fn ex02() {
    let new_vec = (1..).take(10).collect::<Vec<i32>>();
    println!("{:?}", new_vec);
}

pub fn ex03() {
    let _my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let _new_vec = _my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
}

// Iterators and mut reference
pub fn ex04() {
    let _vector1 = vec![1, 2, 3];
    let mut _vector2 = vec![10, 20, 30];

    for num in _vector1.iter() {
        println!("Printing a &i32 = {:?}", num);
    }

    for num in _vector1 {
        println!("Printing an i32 = {:?}", num);
    }

    for num in _vector2.iter_mut() {
        *num *= 10;
        println!("mut vector now is : {:?}", num);
    }

    println!("_vector2: {:?}", _vector2);
    println!("_vector1 is destroy...");
}

// in chaining methods notice: into_iter() would move iter()
pub fn ex05() {
    let _vector1   = vec![1, 2, 3];
    let _vector1_a = _vector1.iter().map(|x| x+1).collect::<Vec<i32>>();
    let _vector1_b = _vector1.into_iter().map(|x| x*10).collect::<Vec<i32>>();
    let mut _vector2 = vec![10, 20, 30];
    _vector2.iter_mut().for_each(|x| *x += 100);

    println!("{:?}", _vector1_a);
    println!("{:?}", _vector1_b);
    println!("{:?}", _vector2);
}

// write an iterator for my own collection
#[derive(Clone, Debug)]
struct BookCollection(Vec<String>);

#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string());
    }

    fn new(name: &str) -> Self {
        Self {
            name:name.to_string(),
            books: BookCollection(Vec::new()),
        }
    }
    fn get_books(&self) -> BookCollection {
        self.books.clone()
    }
}

impl Iterator for BookCollection {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.0.pop() {
            Some(book) => {
                println!("Accessing book:{book}!");
                Some(book)
            }
            None => {
                println!("No book found!");
                None
            }
        }
    }
}

pub fn ex06() {
    let mut my_library = Library::new("Calgary");

    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    for item in my_library.get_books() {
        println!("{item}");
    }
}

// use HashMap
use std::collections::HashMap;

pub fn ex07() {
    let _some_keys = vec![0, 1, 2, 3, 4, 5];
    let _some_value = vec!["one", "two", "three", "four", "five"];

    let _hash_map = _some_keys
        .into_iter()
        .zip(_some_value.into_iter())
        .collect::<HashMap<_, _>>();

    println!("The value at key 2 is : {}", _hash_map.get(&2).unwrap());
}






fn main() {
    println!("Hello, Rust Iterator and closures！");
    ex01();
    ex02();
    ex03();
    ex04();
    ex05();
    ex06();
}
