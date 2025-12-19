/*
Primitives - From book

Rust provides access to a wide variety of primitives. A sample includes:
Scalar Types

    Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    Floating point: f32, f64
    char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
    bool either true or false
    The unit type (), whose only possible value is an empty tuple: ()

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.
Compound Types

    Arrays like [1, 2, 3]
    Tuples like (1, true)

Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. 
Integers default to i32 and floats to f64. Note that Rust can also infer types from context.

*/

pub struct primitives{}

impl primitives{
    pub fn new() -> Self {
        primitives{}
    }
    pub fn code(&self) -> () {
        let logical : bool = true; // boolean
        let a_float : f64 = 1.0; //float
        let an_integer = 5i32; // suffix annotation

        // by default
        let default_float = 1.0; // f64
        let default_integer = 5; // i32

        // A type can also be inferred from context
        let mut inferred_type = 12; // i32 
        inferred_type = 4294967296i64; // -> i64

        // A mutable variable's value can be changed
        let mut number : i32 = 32;
        number = 21;

        // number = true; -> Error, The type of a variable can't be changed

        // Variables can be overwritten with shadowing
        let mut number: bool = true;

        /* Compound types - Array and Tuple */

        // Array signature consists of Type T and length as [T; length]
        let my_array : [i32;5] = [1,2,3,4,5];

        // Tuple is a collection of values of different types
        // and is constructed using parentheses ().
        let my_tuple : (i32, bool, u64) = (-1234, true, 560000);

        println!("Section 2 completed !");
    }
}