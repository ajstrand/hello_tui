fn main() {
    println!("Hello, world!");
    
    let mut x = 42;
    let y = "test string";
    
    if x > 0 {
        println!("x is positive: {}", x);
    }
    
    for i in 0..5 {
        println!("Loop iteration: {}", i);
    }
    
    // This is a comment
    match x {
        42 => println!("The answer!"),
        _ => println!("Something else"),
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }
}
