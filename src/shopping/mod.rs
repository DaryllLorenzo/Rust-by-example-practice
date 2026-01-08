use crate::shopping::{structs::{Cart, Product}, traits::Calculable};


mod structs;
mod enums;
mod traits;
pub struct Shopping{}

impl Shopping {
    pub fn run() -> () {
        println!("-------------------- Excercise 5 shopping ------------");

        let p1 = Product::new(1,
                                "T-shirt".to_string(),
                                1.0,
                                enums::Category::Clothes,
                                enums::ProductState::Available
            );

        let p2 = Product::new(2,
                                "Trousers".to_string(),
                                6.0,
                                enums::Category::Clothes,
                                enums::ProductState::Available
            );
        
        let p3 = Product::new(3,
                                "Laptop".to_string(),
                                300.0,
                                enums::Category::Electronics,
                                enums::ProductState::Available
            );

        let p4 = Product::new(4,
                                "TV".to_string(),
                                500.0,
                                enums::Category::Electronics,
                                enums::ProductState::Available
            );

        let p5 = Product::new(5,
                                "Refrigerator".to_string(),
                                700.0,
                                enums::Category::Electronics,
                                enums::ProductState::Available
            );

        let cart = Cart{
            items : vec![p1,p2,p3,p4,p5]
        };

        println!("Total with discount = {}", cart.calculate_discount());

    }
}