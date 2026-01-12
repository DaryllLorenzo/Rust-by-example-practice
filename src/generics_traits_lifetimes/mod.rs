mod enums;
use enums::estado;
use enums::estado_fijo;

use std::fmt::Debug;

pub struct GTL {}

impl GTL {
    pub fn run(){
        println!("----------------------------- Ex7 Generics, traits, lifetimes --------------");

        // Crear item para cualquier tipo
        let mut libro1 = Libro{
            autor : String::from("Pepe"),
            paginas : 32
        };

        let item1 = LibraryItem::new(&mut libro1, estado::estado_fijo(estado_fijo::Disponible));

        println!("Library item 1: {:?}", item1);

        let mut dvd1 = DVD{
            director: String::from("Juan"),
            duracion: 5
        };

        let item2 = LibraryItem::new(&mut dvd1, estado::estado_fijo(estado_fijo::Disponible));

        println!("Library item 2: {:?}", item2);

        GTL::verificar_lectura(&item1);

        // GTL::verificar_lectura(item2); Da error porque item DVD no implementa el trait Legible

        GTL::procesar_item_lectura_paginable_escucha(&item1, &item2);

        let mut libro2 = Libro{
            autor : String::from("Juan"),
            paginas : 15
        };
        let mut item3 = LibraryItem::new(&mut libro2, estado::estado_fijo(estado_fijo::Disponible));
        let usuario1 = String::from("Prestamista");
        let prestamo = GTL::prestar_item(&mut item3, &usuario1);
        println!("Prestamo aplicado al item: {:?}", prestamo);

        println!("Probando edicion de libro");
        LibraryItem::editar_libro(prestamo.0, libro1);
        println!("Libro de prestamo actualizado: {:?}", prestamo.0);

        let mut ll1 = media::Libro(
            Libro {
                autor : String::from("Pepe"),
                paginas : 32
            }
        );
        let mut ll2 = media::Libro(
            Libro{
                autor : String::from("Pea"),
                paginas : 33
            }
        );

        let mut dvdd1 = media::DVD(
            DVD {
                director: String::from("Al"),
                duracion: 12
            }
        );

        let itemm1 = LibraryItem::new(&mut ll1, estado::estado_fijo(estado_fijo::Disponible));
        let itemm2 = LibraryItem::new(&mut ll2, estado::estado_fijo(estado_fijo::Disponible));
        let itemm3 = LibraryItem::new(&mut dvdd1, estado::estado_fijo(estado_fijo::Disponible));

        let mut biblioteca = Biblioteca::new();

        biblioteca.add_item(itemm1);
        biblioteca.add_item(itemm2);
        biblioteca.add_item(itemm3);

        biblioteca.list_items();

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

    fn prestar_item<'a,'b, T>(item: &'a mut LibraryItem<'a, T>, user: &'b String) -> (&'a mut LibraryItem<'a, T>, &'b str){
        item.estado = estado::estado_fijo(estado_fijo::Prestado);
        return (item, user.as_str());
    }
}

#[derive(Debug)]
struct LibraryItem<'a, T>{
    content: &'a mut T,
    estado: estado
}

impl<'a, T> LibraryItem<'a, T>{
    fn new(content: &'a mut T, estado: estado) -> Self{
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
    fn paginar(self) -> Libro;
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
    fn paginar(self) -> Libro {
        println!("Paginar...");
        self
    }
}

impl Escuchable for DVD{
    fn escuchar()->(){
        println!("Escuchando");
    }
}

impl<'a, 'b> GestionBiblioteca<'a> for LibraryItem<'b, Libro>
where Libro: Paginable
{
    type Item = LibraryItem<'b, Libro>;
    type Item2 = Libro;
    fn editar_libro(item: &'a mut Self::Item, item_actualizado: Self::Item2) -> &'a Self::Item {
        let libro = item_actualizado.paginar();
        item.content.autor = libro.autor;
        item.content.paginas = libro.paginas;
        return item;
    }
}

// ── Tipos asociados (type Item) ───────────────
// - 1 impl por struct
// - Define UN tipo fijo por implementación
// - Ideal cuando la relación es única (ej: Iterator → Item)

// ── Traits genéricos (trait Foo<T>) ───────────
// - Múltiples impls por struct (una por T distinto)
// - Ideal para relaciones flexibles (ej: From<T>, Into<T>)

// Regla rápida:
// "Uno a uno" → type Item
// "Uno a muchos" → trait con <T>

trait GestionBiblioteca<'a> {
    type Item;
    type Item2;

    fn editar_libro(item: &'a mut Self::Item, item_actualizado: Self::Item2) -> &'a Self::Item;
}


struct Biblioteca<'a, T>{
    items: Vec<LibraryItem<'a, T>>
}

impl<'a> Biblioteca<'a, media>
{
    fn new() -> Self{
        Biblioteca { items: vec![] }
    }

    fn add_item(&mut self, item: LibraryItem<'a, media>){
        self.items.push(item);
    }

    fn list_items(&self){
        println!("----- ITEMS EN LA BIBLIOTECA ----- ");
        for item in self.items.iter(){
            println!("{:?}", item);
        }
    }
}

#[derive(Debug)]
enum media {
    Libro(Libro),
    DVD(DVD),
    Revista(Revista)
}

