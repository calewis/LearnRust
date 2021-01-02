fn main() {
    let max = Pet::Dog(String::from("Max"));
    let tab = Pet::Cat(String::from("Tabitha"));

    max.make_sound();
    tab.make_sound();

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

enum Pet {
    Dog(String),
    Cat(String),
}

impl Pet {
    fn make_sound(&self){
        match self {
            Pet::Dog(name) => println!("{} says woof!", name),
            Pet::Cat(name) => println!("{} says meow!", name),
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
