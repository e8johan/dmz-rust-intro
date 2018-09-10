use std::rc::Rc;

struct Person {
    name: Rc<String>
}

impl Person {
    fn new(n: Rc<String>) -> Person {
        Person { name: n }
    }
    
    fn hello(&self) {
        println!("Hello, I'm {}", self.name);
    }
    
    // More here
}

fn main() {
    let name = Rc::new(String::from("Kalle"));
    let kalle = Person::new(name.clone());
    
    kalle.hello();
    println!("Person: {}", name);
}
