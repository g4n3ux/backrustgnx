// definimos los modulos a exporta

pub mod usuario;
pub mod rol;

// exportamos para poder usar 'database::usuario' directamente
pub use usuario::Usuario;
pub use rol:: Rol;