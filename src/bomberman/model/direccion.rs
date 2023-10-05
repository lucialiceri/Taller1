/// Enumeración que representa las direcciones posibles.
#[derive(Debug, PartialEq, Clone)]
pub enum Direccion {
    /// Representa la dirección hacia la izquierda.
    Izquierda,

    /// Representa la dirección hacia la derecha.
    Derecha,

    /// Representa la dirección hacia arriba.
    Arriba,

    /// Representa la dirección hacia abajo.
    Abajo,
}
