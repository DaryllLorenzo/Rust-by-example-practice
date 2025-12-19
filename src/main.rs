mod hello_world;
mod primitives;

fn main(){
    let ex1 = hello_world::HelloWorld::new();
    ex1.greet();

    let ex2 = primitives::primitives::new();
    ex2.code();
}