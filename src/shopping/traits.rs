use crate::shopping::structs::Product;
use crate::shopping::enums::Category;

pub trait Calculable {
    fn calculate_discount(&self) -> f64;
}

pub trait Filtrable {
    fn filter_by_category(&self, category: Category) -> Vec<&Product>;
}