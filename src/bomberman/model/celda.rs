pub use super::objeto::Objeto;

/// Representa una celda en el laberinto con un objeto específico.
#[derive(Debug, PartialEq, Clone)]
pub struct Celda {
    /// Objeto contenido en la celda.
    pub objeto: Objeto,
    /// Coordenada x de la celda en el laberinto.
    pub x: usize,
    /// Coordenada y de la celda en el laberinto.
    pub y: usize,
}
