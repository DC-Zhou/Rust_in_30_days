
pub fn ex01() {
    let _array1 = ["one", "two", "three", "four"];
    let _array2 = ["one", "two", "three"];
    let _array1 = [1,2,3,4];
    println!("array1 = {:?}", _array1);

    let _array3 = ["a"; 5];
    println!("array3 = {:?}", _array3);

    let _array4 = ["one", "two", "three", "four"];
    println!("array4 = {}", _array4[2]);
}

// Slice array
pub fn ex02() {
    let _array = [0,1,2,3,4,5,6,7,8,9];

    let a1 = &_array[2..5];
    let a2 = &_array[4..];
    let a3 = &_array[..5];
    let a4 = &_array[..];

    println!("a1 = {:?}", a1);
    println!("a2 = {:?}", a2);
    println!("a3 = {:?}", a3);
    println!("a4 = {:?}", a4);
}

// array by iter
pub fn ex03() {
    let mut _array = [0; 10];
    for element in _array.iter_mut() {
        *element += 1;
    }

    println!("fill array by iter = {:?}", _array);
}

// init array by number
pub fn ex04() {
    const NUM: i32 = 10;

    let mut array = [0; 10];

    for i in 0..10{
        array[i] = array[i] + NUM;
    }
    println!("init array = {:?}", array);
}

// Vec
pub fn ex05() {
    let name1 = String::from("Apple");
    let name2 = String::from("Canada");

    let mut my_vec = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);

    println!("Vec concat = {:?}", my_vec);
}

// init Vec
pub fn ex06() {
    let mut vec1: Vec<String> = Vec::new();

    // fill str1
    vec1.push(String::from("apple"));

    println!("vec1 filled by push = {:?}", vec1);

    // fill by vec!
    let vec2 = vec![String::from("apple"), String::from("canada")];

    println!("str2 filled by vec! = {:?}", vec2);
}

// Vec Slice
pub fn ex07() {
    let vec1 = vec![0,1,2,3,4,5,6,8,9];
    let vec2 = &vec1;
    let vec3 = &vec1[2..5];
    let vec4 = &vec1[1..];
    let vec5 = &vec1[..5];
    let vec6 = &vec1[..];

    println!("vec1 = {:?}", vec1);
    println!("vec2 = {:?}", vec2);
    println!("vec3 = {:?}", vec3);
    println!("vec4 = {:?}", vec4);
    println!("vec5 = {:?}", vec5);
    println!("vec6 = {:?}", vec6);
}

pub fn ex08() {
    let mut vec1 = Vec::new();
    println!("vec1 capacity = {}", vec1.capacity());
    vec1.push(1);
    println!("vec1 push = {}", vec1.len());
    println!("vec1 capacity = {}", vec1.capacity());
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);
    println!("vec1 push = {}", vec1.len());
    println!("vec1 capacity = {}", vec1.capacity());
    vec1.push(5);
    println!("vec1 push = {}", vec1.len());
    println!("vec1 capacity = {}", vec1.capacity());
}

pub fn ex09() {
    let vec1:Vec<i32> = [1,2,3].into();
    println!("vec1 = {:?}", vec1);
    let vec2:Vec<_>   = [1,2,3,4].into();
    println!("vec2 = {:?}", vec2);
}

// tuples
pub fn ex10() {
    let tuple1 = ("Here is a tuple!", 8, vec!['a';3], 'b', [8, 9, 10]);

    println!("tuple1.0 = {:?}", tuple1.0);
    println!("tuple1.1 = {:?}", tuple1.1);
    println!("tuple1.2 = {:?}", tuple1.2);
    println!("tuple1.3 = {:?}", tuple1.3);
    println!("tuple1.4 = {:?}", tuple1.4);
}

pub fn ex11() {
    let (a, b, c) = ("one", "two", "three");
    println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);

    let (_, b, c) =("four", "five", "seven");
    println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);
}

pub fn ex12() {
    let number = 5;
    if number % 2 == 1 && number > 0 {
        println!("its a odd positive number");
    } else if number % 2 == 0 {
        println!("its a even number");
    } else if number > 0{
        println!("its a positive number");
    }
}

pub fn ex13() {
    let num: u8 = 5;
    match num {
        0 => println!("its first zero"),
        1 => println!("its first one"),
        _ => println!("its other numbers"),
    }

    let num2 = match num {
        0 => 0,
        5 => 10,
        _ => 15
    };

    match (num2, num) {
        (5, 10) => println!("its first five"),
        (5, 100) => println!("its second five"),
        _ => println!("its other numbers"),
    }

    let children = 5;
    let married = true;

    match (children, married) {
        (_, false) if married => println!("its children"),
        (_, true)  if married => println!("its married"),
        (5, true)  if children == 5 && married => println!("its married and have its children"),
        (_, _) => println!("its mass"),
    }

}

// match(if matched, it would not match anything else)
fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("red less than 10"),
        (_, g, _) if g < 10 => println!("green less than 10"),
        (_, _, b) if b < 10 => println!("blue less than 10"),
        _ => println!("hmm"),
    }
}
pub fn ex14() {
    let num1 = (200, 0, 0);
    let num2 = (50, 50, 50);
    let num3 = (200, 50, 0);

    match_colors(num1);
    match_colors(num2);
    match_colors(num3);
}

// loop
pub fn ex15() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter = {}", counter);
        if counter == 5 {
            break;
        }
    }
}

pub fn ex16() {
    for num in 0..3 {
        println!("the number is :{}", num);
    }
    for num in 0..=3{
        println!("the number is :{}", num);
    }
}

fn main() {
    println!("Hello More Complex Types: arrays/Vec/tuple...");

    ex16();
}
