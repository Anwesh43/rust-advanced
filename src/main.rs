use std::{io::{stdin, Read}, num::ParseIntError};

struct Animal {
    name: String, age: i64 
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    Reject,
    AcceptWithNote{note : String}
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
}

impl Visitor {
    fn new(name : String) -> Self {
        Self {name : name, action: VisitorAction::Accept}
    } 

    fn greeting(&self) {
        println!("Hello {name}", name = self.name);
        match &self.action {
            VisitorAction::Accept => println!("Accepted"),
            VisitorAction::Reject => println!("Rejected"),
            VisitorAction::AcceptWithNote { note } => println!("Accepted with {note}", note = note)
        }
    }

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
    input.trim().to_string()
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
    let visitors : [&str; 3] = ["An1", "An2", "An3"];
    let inputVisitor = read_line_input();

    let visitorStructs : [Visitor; 3] = [
        Visitor::new(visitors[0].to_string()),
        Visitor::new(visitors[1].to_string()),
        Visitor::new(visitors[2].to_string()),
    ];

    let mut matched : bool = false;
    for visitor in &visitors {
        if visitor == &inputVisitor {
            matched = true;
        }
    }
    let matchedVisitor = visitorStructs.iter().find(|v| v.name == inputVisitor);
    if matched {
        println!("Input is present")
    } else {
        println!("Input is not present")
    }

    match matchedVisitor {
        Some(visitorCurr) => visitorCurr.greeting(),
        None => println!("Visitor not present")
    }

    let mut vistior_list : Vec<Visitor> = vec![];
    loop {

        let name_input = read_line_input();
        if name_input == "Quit".to_string() {
            break;
        }
        vistior_list.push(Visitor::new(name_input));
    }

    vistior_list.iter().for_each(|v| v.greeting())
}
