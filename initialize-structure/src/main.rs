struct Person {
    name: String
}

impl Person {
    fn new(n: &String) -> Person {
        Person { name: n }
    }
    
    fn hello(&self) {
        println!("Hello, I'm {}", self.name);
    }
}
    

fn main() {
    let name = String::from("Kalle");
    let kalle = Person::new(&name);
    
    kalle.hello();
    println!("Person: {}", name);
}
