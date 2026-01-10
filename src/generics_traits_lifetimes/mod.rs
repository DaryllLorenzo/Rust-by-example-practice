mod enums;
use enums::estado;
use enums::estado_fijo;

use std::fmt::Debug;

pub struct GTL {}

impl GTL {
    pub fn run(){
        println!("----------------------------- Ex7 Generics, traits, lifetimes --------------");

        // Crear item para cualquier tipo
        let libro1 = Libro{
            autor : String::from("Pepe"),
            paginas : 32
        };

        let item1 = LibraryItem::new(&libro1, estado::estado_fijo(estado_fijo::Disponible));

        println!("Library item 1: {:?}", item1);

        let dvd1 = DVD{
            director: String::from("Juan"),
            duracion: 5
        };

        let item2 = LibraryItem::new(&dvd1, estado::estado_fijo(estado_fijo::Disponible));

        println!("Library item 2: {:?}", item2);

        GTL::verificar_lectura(&item1);

        // GTL::verificar_lectura(item2); Da error porque item DVD no implementa el trait Legible

        GTL::procesar_item_lectura_paginable_escucha(&item1, &item2);
    }


    fn verificar_lectura<'a, T>(item: &LibraryItem<'a, T>)
    where T:Legible
    {
        item.content.leer();
    }

    fn procesar_item_lectura_paginable_escucha<'a, T, U>(item: &LibraryItem<'a, T>, item2: &LibraryItem<'a, U>)
    where
    T:Legible + Paginable + Debug,
    U: Escuchable + Debug
    {
        println!("Item legible y con paginas: {:?}", item.content);
        println!("Item escuchable: {:?}", item2.content);
    }
}

#[derive(Debug)]
struct LibraryItem<'a, T>{
    content: &'a T,
    estado: estado
}

impl<'a, T> LibraryItem<'a, T>{
    fn new(content: &'a T, estado: estado) -> Self{
        LibraryItem { content, estado }
    }
}

#[derive(Debug)]
struct Libro{
    pub autor: String,
    pub paginas: i32
}

#[derive(Debug)]
struct DVD {
    pub director: String,
    pub duracion: i32
}

#[derive(Debug)]
struct Revista{
    pub editor: String
}


trait Prestable {
    fn puede_prestarse(&self) -> bool;
}

impl<'a, T> Prestable for LibraryItem<'a, T>{
    fn puede_prestarse(&self) -> bool {
        if self.estado == estado::estado_fijo(estado_fijo::Disponible) {
            return true;
        }
        false
    }
}

trait Legible {
    fn leer(&self) -> ();
}

trait Paginable {
    fn paginar(&self) -> ();
}

trait Escuchable{
    fn escuchar();
}

impl Legible for Revista{
    fn leer(&self) -> () {
        println!("{:?}", self);
    }
}

impl Legible for Libro {
    fn leer(&self) -> () {
        println!("{:?}",self);
    }
}

impl Paginable for Libro{
    fn paginar(&self) -> () {
        println!("Paginar...");
    }
}

impl Escuchable for DVD{
    fn escuchar()->(){
        println!("Escuchando");
    }
}