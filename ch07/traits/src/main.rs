// // use trait
// struct Dog {
//     name: String,
// }
//
// struct Parrot {
//     name: String,
// }
//
// trait Doglike {
//     fn bark(&self) {
//         println!("Woof, Woof!");
//     }
//     fn run(&self) {
//         println!("The dog is running...");
//     }
// }
//
// impl Doglike for Dog {}
// impl Doglike for Parrot {}
//
// pub fn ex01() {
//     let rover = Dog {
//         name: "Rover".to_string(),
//     };
//
//     let brian = Parrot {
//         name: "Brian".to_string()
//     };
//
//     rover.bark();
//     rover.run();
//     brian.run();
// }

struct Parrot {
    name: String,
}

// traits could only impl to replace an
trait DogLike {
    fn bark(&self)
    {
        println!("Woof, Woof!");
    }
    fn run(&self)
    {
        println!("The dog is running...");
    }
}

impl DogLike for Parrot {
    fn run(&self) {
        println!("{} the parrot is running!", self.name);
    }
}

pub fn ex02() {
    let brain = Parrot {
        name: "Brain".to_string(),
    };

    brain.bark();
    brain.run();
}

// use impl and trait to display
use std::fmt;
use std::fmt::Debug;

struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} year old", self.name, self.age)
    }
}

pub fn ex03() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("{}", mr_mantle);
}

// Impl Display for a type ,you get the ToString trait for free
fn print_excitedly(input: String)
{
    println!("{input}!!!!");
}

pub fn ex04() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    print_excitedly(mr_mantle.to_string());

    println!(
        "Mr. Mantle's String is {} letters log.", mr_mantle.to_string().chars().count()
    );
}

#[derive(Debug)]
struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl FightClose for Ranger {}
impl FightClose for Wizard {}
impl FightFromDistance for Ranger {}
impl Magic for Wizard {}

fn attack_with_bow<T>(pc: &T, opponent: &mut Monster, distance: u32) where T: FightFromDistance + Debug,
{
    if distance < 10 {
        opponent.health -= 10;
        println!("Bow attack! Opponent's health: {}, your health at {pc:?}", opponent.health);
    }
}

fn attack_with_sword<T> (pc: &T, opponent: &mut Monster) where T: FightClose + Debug,
{
    opponent.health -= 8;
    println!("Sword attack! Opponent's health: {}, your health at {pc:?}", opponent.health);
}

fn fireball<T> (pc: &T, opponent: &mut Monster, distance: u32) where T: Magic + Debug,
{
    if distance < 10 {
        opponent.health -= 5;
        println!("Small Fireball attack! Opponent's health: {}, your health at {pc:?}", opponent.health);
    }

    if distance >= 10 {
        opponent.health -= 15;
        println!("Big Fireball attack!, Opponent's health: {}, your health at {pc:?}", opponent.health);
    }
}

*
pub fn ex05() {
    let player1 = Wizard {health: 20};
    let player2 = Ranger {health: 30};

    let mut monkey = Monster{health: 30};

    attack_with_sword(&player1, &mut monkey);
    attack_with_bow(&player2, &mut monkey, 5);
    fireball(&player1, &mut monkey, 11);
}


fn main() {
    println!("Hello, use traits in Rust World!");
    // ex01();
    ex02();
    ex03();
    ex04();
    ex05();
}
