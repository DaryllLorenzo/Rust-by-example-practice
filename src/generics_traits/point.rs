

use::std::fmt::Display;

#[derive(Debug)]
pub struct Point<T,U> {
    pub x : T,
    pub y : U
}

pub trait Translator<T,U> {
    /// Traduce las coordenadas del punto.
    ///
    /// # Parámetros
    ///
    /// * `x_y` - Una tupla `(T, U)` con las nuevas coordenadas
    ///
    /// # Ejemplo
    ///
    /// ```
    /// let mut punto = Point { x: 0, y: 0 };
    /// punto.translate((5, 10));
    /// assert_eq!(punto.x, 5);
    /// assert_eq!(punto.y, 10);
    /// ```
    fn translate(&mut self, x_y: (T, U) ) -> ();
}

impl<T, U>Translator<T,U> for Point<T, U> {
    fn translate(&mut self, x_y: (T, U) ) -> () {
        self.x = x_y.0;
        self.y = x_y.1;
    }
} 

impl<T, U>Point<T,U>
where 
T:Display,
U:Display
{
    pub fn wow (&self) {
        println!("T and U must implement Display trait");
    }
}