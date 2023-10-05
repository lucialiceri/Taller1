/// Enumeración que define los diferentes tipos de objetos en el laberinto.
#[derive(Debug, PartialEq, Clone)]
pub enum Objeto {
    /// Representa un enemigo con una cantidad específica de vidas.
    Enemigo(i32),

    /// Representa una bomba con un alcance específico.
    Bomba(i32),

    /// Representa una bomba traspasable con un alcance específico.
    BombaTraspaso(i32),

    /// Representa una roca en el laberinto.
    Roca,

    /// Representa una pared en el laberinto.
    Pared,

    /// Representa un desvío con una dirección específica.
    Desvio(super::direccion::Direccion),

    /// Representa una celda vacía en el laberinto.
    Vacio,
}
