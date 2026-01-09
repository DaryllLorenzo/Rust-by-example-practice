use crate::generics_traits::Run;

mod hello_world;
mod primitives;
mod basic_to_do;
mod generics_traits;
mod shopping;
mod lifetimes;

fn main(){
    let ex1 = hello_world::HelloWorld::new();
    ex1.greet();

    let ex2 = primitives::primitives::new();
    ex2.code(); 

    let mut ex3 = basic_to_do::TodoList::new();
    ex3.run();

    generics_traits::Program::execute();

    shopping::Shopping::run();

    lifetimes::Lifetimes::run();
}