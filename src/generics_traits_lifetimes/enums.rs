#[derive(Debug)]
#[derive(PartialEq)]
pub enum opciones{
    Reparacion,
    Dañado,
    Archivado
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum estado_fijo {
    Disponible,
    Prestado
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum estado {
    estado_fijo(estado_fijo),
    otro_estado(opciones)
}