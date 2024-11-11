
pub fn ex01() {
    let _months = vec!["January", "February", "March", "April", "May",
    "June", "July", "August", "September", "October", "November", "December"];

    let _filtered_months = _months
    .into_iter()
        .filter(|month| {month.len() < 5})
        .filter(|month| {month.contains('u')})
        .collect::<Vec<&str>>();

    println!("{:?}", _filtered_months);
}

struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self{
        let ceo = match ceo {
            "" => None,
            _ => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

pub fn ex02() {
    let _company_vec = vec![Company::new("JanuarySpring", "January"),
    Company::new("FebruaryRobot", "February"),
    Company::new("MarchAir", ""),
    Company::new("AprilAir", ""),];

    let all_the_ceos = _company_vec
        .iter().filter_map(|company| {company.get_ceo()})
        .collect::<Vec<_>>();

    println!("{:?}", all_the_ceos);
}

pub fn ex03() {
    let user_input = vec![
        "8.9",
        "Eight point nine",
        "8.0",
        "7.6",
        "Nine point nine",
    ];

    let _successful_numbers = user_input
        .iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<_>>();

    println!("use filter_map to check: {:?}", _successful_numbers);
}

struct CompanyInfo {
    name: String,
    ceo: Option<String>,
}

impl CompanyInfo {
    fn new(name: &str, ceo: &str) -> Self{
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };

        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

pub fn ex04() {
    let _company_vec = vec![Company::new("JanuarySpring", "January"),
    Company::new("FebruaryRobot", "February"),
    Company::new("MarchAir", ""),
    Company::new("AprilAir", ""),];

    let results: Vec<Result<String, &str>> = _company_vec
        .iter()
        .map(|company| company.get_ceo().ok_or("No CEO found!"))
        .collect();

    for it in results {
        println!("{:?}", it);
    }
}

// ok_or_else()
fn get_current_datetime() -> String {
    "2024-11-11T21:00::00".to_string()
}

pub fn ex05() {
    let _company_vec = vec![Company::new("JanuarySpring", "January"),
    Company::new("FebruaryRobot", "February"),
    Company::new("MarchAir", ""),
    Company::new("AprilAir", ""),];

    let results: Vec<Result<String, String>> = _company_vec
        .iter()
        .map(|company| {
            company.get_ceo().ok_or_else(
            || {
                let err_message = format!("No CEO found for {}", company.name);
                println!("{err_message} at {}", get_current_datetime());
                err_message
            })
        }).collect();

    results.iter().filter(|res| res.is_ok())
        .for_each(|res| println!("{:?}", res));
}

// Some More Iterator and related methods
// and_then
pub fn ex06() {
    let _num_array = ["8", "9", "Hi", "898989856"];
    let mut _char_vec = vec![];

    for index in 0..5 {
        _char_vec.push(
            _num_array.get(index)
                .and_then(|number| number.parse::<u32>().ok())
                .and_then(|number| char::try_from(number).ok()),
        );
    }
    println!("{:?}", _char_vec);
}

// and
pub fn ex07() {
    let _vec1 = vec![Some("Okey"), None, Some("Okey"), None];
    let _vec2 = vec![Some("Okey"), None, Some("Okey"), Some("Okey")];
    let _vec3 = vec![None,         None, Some("Okey"), Some("Okey")];

    for i in 0.._vec1.len() {
        println!("{:?}", _vec1[i].and(_vec2[i]).and(_vec3[i]));
    }
}

// .any() and .all()
pub fn ex08() {
    let _vec1 = vec![5; 10];
    let _vec2 = vec![6; 10];

    println!("Any 6 in vec1 ? {:?}", _vec1.iter().rev().any(|&x| x == 6));
    println!("All 5 in vec1 ? {:?}", _vec1.iter().rev().all(|&x| x == 5));

    println!("Any 6 in vec2 ? {:?}", _vec2.iter().rev().any(|&x| x == 6));
    println!("All 5 in vec2 ? {:?}", _vec2.iter().rev().all(|&x| x == 5));
}

// .find() and .position()
pub fn ex09() {
    let _vec1 = vec![0,1,2,3,4,5,6,7,8,9];

    let mut result = _vec1.iter().filter(|x| *x % 3 == 0).collect::<Vec<&i32>>();

    println!("filter which %3 == 0: {:?}", result);

    let mut result = _vec1.iter().rev().find(|num| *num % 3 == 0);

    println!("find which %3 == 0: {:?}", result);

    let mut result = _vec1.iter().position(|num| *num % 3 == 0);

    println!("position at which %3 == 0: {:?}", result);
}

// cycle() zip()
pub fn ex10() {
    let _even_odd_iter = ["even", "odd"].into_iter().cycle();

    let _even_odd_vec: Vec<(i32, &str)> = (0..=5)
        .zip(_even_odd_iter)
        .collect();

    println!("{:?}", _even_odd_vec);
}

#[derive(Debug)]
struct CombinedEvents {
    num_of_events: u32,
    data: Vec<String>,
}

pub fn ex11() {
    let _events = [
        "go school",
        "read book",
        "go home",
        "feed cat",
    ];

    let empty_events = CombinedEvents {
        num_of_events: 0,
        data: vec![],
    };

    let combined_events = _events.iter()
        .fold(empty_events, |mut total_events, next_event| {
            total_events.num_of_events += 1;
            total_events.data.push(next_event.to_string());
            total_events
        });
    println!("{combined_events:#?}")
}



fn main() {
    println!("Ch09 Iterators and Closures in Rust!");
    ex01();
    ex02();
    ex03();
    ex04();
    ex05();
    ex06();
    ex07();
    ex08();
    ex09();
    ex10();
    ex11();
}
