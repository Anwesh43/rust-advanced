use std::{io::{stdin, Read}, num::ParseIntError};

struct Animal {
    name: String, age: i64 
}

impl Animal {

    fn new(&self) -> Animal {
        Animal {name: self.name.to_owned(), age: self.age}
    }

    fn display(&self) {
        println!("Animal: {name} {age}", name = self.name, age = self.age)
    }

    fn doubleAge(&mut self) {
        self.age = self.age * 2
    }
}

fn parse_num(input_age : String) -> Result<i64, ParseIntError> {
    let ageNum = input_age.parse::<i64>()?;
    Ok(ageNum)
}

fn read_line_input() -> String {
    let mut input : String = String::new();
    stdin().read_line( &mut input).expect("Can't read input");
    input
}

fn main() {
    println!("Hello, world!");
    let animal = Animal { name: "Dog".to_string(), age: 2 };
    animal.display();
    let mut animal2 : Animal = animal.new();
    animal2.doubleAge();
    animal2.display();
    let mut inputName: String = read_line_input();
    let mut inputAge : String = read_line_input(); 
    // stdin().read_line(&mut inputName).expect("Failed to read line");
    println!("Got name enter age");
    // stdin().read_line(&mut inputAge).expect("Failed to read line");
    let age_num_result = parse_num(inputAge);
    match age_num_result {
        Ok(age) => {
            let animal3 = Animal {name : inputName, age: age};
            animal3.display();
        }
        Err(e) => {
            println!("{:?}", e);
            let animal3 = Animal {name : inputName, age: 0};
            animal3.display();
        }
    }
}
