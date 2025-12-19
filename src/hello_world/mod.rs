pub struct HelloWorld{}


impl HelloWorld{
    pub fn new() -> Self {
        Self{}
    }
    pub fn greet(&self) -> () {
        println!("Hello World!");
        println!("I'm a Rustacean!");
    }
}