use crate::shopping::enums;
use crate::shopping::enums::Category;
use crate::shopping::traits::Calculable;
use crate::shopping::traits::Filtrable;

pub struct Product {
    id : i32,
    name : String,
    price : f64,
    category : enums::Category,
    state : enums::ProductState
}

pub struct Cart {
    pub items : Vec<Product>
}


impl Product {
    pub fn new(id: i32, name: String, price: f64, category: enums::Category, state:enums::ProductState) -> Self {
        Product { id, name, price, category, state}
    }
}


impl Calculable for Cart {
    fn calculate_discount(&self) -> f64 {
        // more than 2 Electronics discount of 30 %
        // more than 5 Clothes discount of 20%

        let total_price = self.items
        .iter()
        .fold(0.0, |acc, element| acc + element.price);

        let total_clothes = self.filter_by_category(Category::Clothes).len();

        let total_electronics = self.filter_by_category(Category::Electronics).len();
        let mut total_discount:f64 = 0.0;

        if total_clothes > 5 {
            let discount = total_price * (20.0 / 100.0);
            total_discount += discount;
        }

        if total_electronics > 2 {
            let discount = total_price * (30.0 / 100.0);
            total_discount += discount
        }

        return total_price - total_discount;
    }
}


impl Filtrable for Cart {
    fn filter_by_category(&self, category: Category) -> Vec<&Product> {
        let filtered_items = self.items
        .iter()
        .filter(|element| element.category == category)
        .collect();

        return filtered_items;
    }
}