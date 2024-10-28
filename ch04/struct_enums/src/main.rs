use std::collections::hash_map::Values;
use std::env::Args;
use std::io::SeekFrom;

// 1.struct
struct ColorRgb(u8, u8, u8);
pub fn ex01() {
    let my_color = ColorRgb(50, 0, 50);
    println!("{}", my_color.0);
    println!("{}", my_color.1);
    println!("{}", my_color.2);
}

struct Pixels {
    len: i32,
    color: ColorRgb,
}

pub fn ex02() {
    let _my_color = ColorRgb(255, 0, 255);

    let _pix = Pixels {
        len: 30,
        color: _my_color
    };

    println!("Pixel array contents: {}", _pix.len);
}

struct Village {
    population: u32,
    capital: u32,
    n_longitude: String,
    e_longitude: String,
}

pub fn ex03() {
    let population = 3;
    let capital = 1;
    let n_longitude = String::from("N-42");
    let e_longitude = String::from("E-24");

    let Kunming = Village {
        population,
        capital,
        n_longitude,
        e_longitude,
    };
    println!("Show the status of a City:");
    println!("{}", Kunming.population);
    println!("({},{})", Kunming.e_longitude, Kunming.n_longitude);
}

pub fn ex04() {
    let Kunming = Village {
        population: 50_000_000,
        capital: 10,
        n_longitude: String::from("N-42"),
        e_longitude: String::from("E-52"),
    };
    println!("Show the status of a City:");
    println!("{}", Kunming.population);
    println!("({},{})", Kunming.e_longitude, Kunming.n_longitude);
}

// enums enums means only one of choice
enum Weather {
    Sun,
    fogy,
    cloudy,
    drizzle,
    rain,
}

struct WeatherReport {
    name: String,
    weather: Weather,
    temperature: f32,
}

pub fn ex05() {
    let _kunming_weather = WeatherReport {
        name: String::from("Kunming"),
        weather: Weather::drizzle,
        temperature: 27.4,
    };
    println!("{} Weather Report:", _kunming_weather.name);
    println!("temperature is {}", _kunming_weather.temperature);

    let _weather_str = match _kunming_weather.weather {
        Weather::Sun => "Sun",
        Weather::fogy => "Fogy",
        Weather::cloudy => "Cloudy",
        Weather::drizzle => "Drizzle",
        Weather::rain => "Rain",
    };

    println!("weather    is {}", _weather_str);
}

//
enum ThingsInTheSky{
    Sun(String),
    Star(String),
    Moon(String),
}

fn create_sky_state(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun(String::from("Sun")),
        18..=24 => ThingsInTheSky::Moon(String::from("Moon")),
        _       => ThingsInTheSky::Star(String::from("Star")),
    }
}

fn check_sky_state(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) =>  {  println!("{description}")},
        ThingsInTheSky::Star(description) => {  println!("{description}")},
        ThingsInTheSky::Moon(description) => {  println!("{description}")},
    }
}

pub fn ex06(){
    let time = 8;
    let sky_state = create_sky_state(time);
    check_sky_state(&sky_state);
}

enum Star{
    BrownDwarf  = 10,
    RedDwarf    = 50,
    YellowDwarf = 100,
    RedGiant    = 1000,
    DeadGiant,
}

pub fn ex07(){
    use Star::*;
    let star_vec = vec![BrownDwarf, RedDwarf, YellowDwarf, RedGiant, DeadGiant];

    for star in star_vec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star.."),
            size if size >= 80 && size <= 200 => println!("good-sized star.."),
            other_size => println!("That star is pretty big!, It's {other_size}"),
        }
    }
}

// impl
#[derive(Debug)]
enum AnimalType{
    Cat,
    Dog,
}

#[derive(Debug)]
struct Animal{
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new_cat() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn check_type(&self){
        match self.animal_type {
            AnimalType::Cat => println!("The animal is a Cat"),
            AnimalType::Dog => println!("The animal is a Dog"),
        }
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed animal to dog!, Now it's {self:?}");
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed animal to cat!, Now it's {self:?}");
    }
}

pub fn ex08(){
    let mut new_animal = Animal::new_cat();
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();
}

struct Person {
    name: String,
    real_name: String,
    height: f32,
    happiness: bool,
}

pub fn ex09() {
    let papa_doc = Person{
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170.0,
        happiness: false,
    };

    // destruct
    let Person {
        name: fake_name,
        real_name,
        height: cm,
        happiness,
    } = papa_doc;

    println!("They call him {fake_name} but his real name is {real_name}\
    He is {cm} cm tall and is he happy ? {happiness}");
}

struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(
        name: &str,
        name_before: &str,
        population: u32,
        date_founded: u32,
    ) -> Self  {
        Self {
            name: String::from(name),
            name_before: String::from(name_before),
            population,
            date_founded,
        }
    }

    fn print_names(&self) {
        let City {
            name,
            name_before,
            ..
        } = self;

        println!("The city {name} used to be called {name_before} .");
    }
}

pub fn ex10()  {
    let tallinn = City::new("Tallinn", "Reval", 426_538, 1219);
    tallinn.print_names();
}

struct SadPerson {
    name: String,
    real_name: String,
    height: f32,
    happiness: bool,
}

fn check_if_happy(person: &SadPerson) {
    println!("Is {} happy? {}", person.name, person.happiness);
}

fn check_if_happy_destructed(SadPerson { name, happiness, ..}: &SadPerson) {
    println!("Is {name} happy ? {happiness}");
}

pub fn ex11() {
    let papa_doc = SadPerson{
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170.0,
        happiness: false,
    };

    check_if_happy(&papa_doc);

    check_if_happy_destructed(&papa_doc);
}



fn main() {
    println!("Hello, Rust World, Show Some Types!");
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
