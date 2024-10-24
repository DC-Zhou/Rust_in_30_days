// stack and heap
pub fn ex01_1() {
    // array is a stack
    let array1: [i32; 5] = [1, 2, 3, 4, 5];

    println!("array is stack, so the address is consecutive");
    println!("array start is {:p}", std::ptr::addr_of!(array1));
    println!("array[0]    is {:p}", std::ptr::addr_of!(array1[0]));
    println!("array[1]    is {:p}", std::ptr::addr_of!(array1[1]));
    println!("array[2]    is {:p}", std::ptr::addr_of!(array1[2]));

    // new is allocated on heap
    let heap_var = Box::new(array1);

    println!("box is heap, so the address is not consecutive");
    println!("heap var start is {:p}", std::ptr::addr_of!(heap_var));
    println!("heap var[0]    is {:p}", std::ptr::addr_of!(heap_var[0]));
    println!("heap var[1]    is {:p}", std::ptr::addr_of!(heap_var[1]));
    println!("heap var[2]    is {:p}", std::ptr::addr_of!(heap_var[2]));

    let box_var = Box::new([1,2,3,4,5]);

    println!("box var start is {:p}", std::ptr::addr_of!(box_var));
    println!("box var[0]    is {:p}", std::ptr::addr_of!(box_var[0]));
    println!("box var[1]    is {:p}", std::ptr::addr_of!(box_var[1]));
    println!("box var[2]    is {:p}", std::ptr::addr_of!(box_var[2]));

    // in stack, the stack address is the first number address ;
    // in heap,  the heap  address store the address of heap begin;
}

// reference
pub fn ex01_2() {
    // var1 in stack
    let var1 = 10;
    let var2 = &var1;
    let var3 = &&var1;

    assert_eq!(var1, 10);
    // error
    // assert_eq!(var2, 10);

    assert_eq!(var2, &10);
    assert_eq!(var3, &&10);

    println!("var1 is {}", var1);
    println!("var2 is {}", var2);
    println!("var3 is {}", var3);

    // show the address if var1 var2 var3
    let address1: *const i32 = std::ptr::addr_of!(var1);
    let address2: *const i32 = std::ptr::addr_of!(*var2);
    let address3: *const i32 = std::ptr::addr_of!(**var3);

    println!("var1 address is {:p}", address1);
    println!("var2 * address is {:p}", address2);
    println!("var3 ** address is {:p}", address3);

}

// &str and String
fn reversed_str1(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.reverse();
    chars.iter().collect()
}

pub fn ex02_1() {
    let orig = "hello world";
    let reversed = reversed_str1(orig);

    println!("before reversed is {}", orig);
    println!("reversed        is {}", reversed);
}
/// &str   is [char; n] on stack (in cpp so-called .c_str())
/// String is Vec[char] on heap  (in cpp so-called String)


pub fn ex02_2() {
    let size_of_string = std::mem::size_of::<String>();
    let size_of_i8 = std::mem::size_of::<i8>();
    let size_of_f64 = std::mem::size_of::<f64>();
    let size_of_jaurim = std::mem::size_of_val("자우림");
    let size_of_adrian = std::mem::size_of_val("Adrian Fahrenheit Țepeș");

    println!("A String is Sized and always {size_of_string} bytes.");
    println!("An i8 is Sized and always {size_of_i8} bytes.");
    println!("An f64 is always Sized and {size_of_f64} bytes.");
    println!("But a &str is not Sized: '자우림' is {size_of_jaurim} bytes.");
    println!("And 'Adrian Fahrenheit Țepeș' is {size_of_adrian} bytes not Sized.");

}

// const and static
pub fn ex03() {
    println!("Show the Diff const and static: ");

    const CONST_MAX_NUM: usize = 20;
    println!("The const maximum number is {}", CONST_MAX_NUM);

    static mut static_max_numbers: usize = 0;
    // you could change static var in unsafe code
    unsafe {
        println!("the static maximum number is {}", static_max_numbers);

        static_max_numbers += 30;

        println!("the static maximum number is {}", static_max_numbers);
    }

}

// im-mutable reference and mutable reference
// 1.you could have many im-mutable reference
// 2.you only could hold one im-mutable reference and you could not have im-mutable and mutable together
pub fn ex04() {
    let var = 100;
    let var1 = &var;
    let var2 = &var1;
    let var3 = &var1;
    println!("im-mutable var could have many: ");
    println!("var  is {}", var);
    println!("var1 is {}", var1);
    println!("var2 is {}", var2);
    println!("var3 is {}", var3);

    println!("mutable var could only one: ");
    let mut m_var = 100;
    println!("var    is {}", m_var);
    let m_var1 = &mut m_var;
    *m_var1 += 100;
    println!("m_var1 is {}", m_var1);
    let mut m_var2 = &m_var1;
    println!("m_var2 is {}", m_var2);
}

// move and reference
fn print_country1(name: String) {
    println!("[use move]:   name is {}", name);
}

fn print_country2(name: String) -> String {
    println!("[use return]: name is {}", name);
    name
}

fn print_country3(name: &String) {
    println!("[use ref]:    name is {}", name);
}

pub fn ex05() {
    let country = String::from("Austria");

    // print_country1(country); only move not return
    let country = print_country2(country);

    print_country3(&country);
}

// why
fn add_hungary(mut str: String) {
    str.push_str("-Hungary");
    println!("{}", str);
}

fn ex06(){
    let mut country = String::from("Austria");
    add_hungary(country);
}

// println!
fn ex07() {
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}

fn main() {
    ex07();
}
