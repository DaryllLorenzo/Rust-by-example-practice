use crate::generics_traits::point::{Point, Translator};

mod point;

pub trait Run {
    fn execute() -> ();
}

pub struct Program {}


impl Run for Program {
    fn execute() -> () {
        println!("------------------------- EX4 -----------------------");
        // Create struct
        let mut point1 = Point{
            x: 5,
            y: 4
        };

        // test fn
        point1.wow();

        println!("{:?} Coordenadas del punto", point1);

        point1.translate((3, 6));

        println!("{:?} Coordenadas del punto", point1);

    }
}