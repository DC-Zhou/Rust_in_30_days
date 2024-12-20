
mod print_things{
    use std::fmt::Display;

    pub fn prints_one_thing<T: Display> (input: T){
        println!("{input}");
    }

    #[derive(Debug)]
    pub struct  Billy {
        name: String,
        pub times_to_print: u32,
    }

    impl Billy {
        pub fn new(times_to_print: u32) -> Self {
            Self {
                name: "Billy".to_string(),
                times_to_print
            }
        }

        pub fn print_billy(&self) {
            for _ in 0..self.times_to_print {
                println!("{}", self.name);
            }
        }
    }

}

pub fn test1() {
    use print_things::prints_one_thing;

    prints_one_thing("Try to print a num:".to_string());
    prints_one_thing(6);
}

pub fn test2() {
    use print_things::Billy;

    let my_billy = Billy::new(4);
    my_billy.print_billy();
}

mod country {
    fn print_country(country: & str) {
        println!("We are in the country of {country} !");
    }

    pub mod province {
        fn print_province(province: &str) {
            println!("We are in the province of {province} !!");
        }

        pub mod city {
            pub fn print_city(country: &str, province: &str, city: &str) {
                crate::country::print_country(country);
                super::super::print_country(country);
                crate::country::province::print_province(province);
                super::print_province(province);
                println!("We are in the City of {city}!!!");
            }
        }
    }
}

pub fn test3() {
    country::province::city::print_city("China", "YuNan", "KunMing");
}

// test
fn return_two() -> i8 {
    2
}

fn return_three() -> i8 {
    3
}

fn return_four() -> i8 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_return_two() {
        assert_eq!(return_two(), 2);
    }

    #[test]
    fn it_return_three() {
        assert_eq!(return_three(), 3);
    }

    #[test]
    fn it_return_four() {
        assert_eq!(return_four(), 4);
    }
}

fn main() {
    println!("Hello, Module and Test Drive Develop!");
    test1();
    test2();
    test3();
}
