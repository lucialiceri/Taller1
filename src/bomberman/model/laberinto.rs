pub use super::celda::Celda;
use super::direccion::Direccion;
use super::objeto::Objeto;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Representa un laberinto compuesto por celdas con objetos.
pub struct Laberinto {
    /// Tamaño del laberinto (número de filas o columnas).
    pub tamano: usize,
    /// Matriz que contiene las celdas del laberinto.
    pub grid: Vec<Vec<Celda>>,
}

impl Laberinto {
    /// Carga un laberinto desde un archivo especificado por `path`.
    ///
    /// # Argumentos
    ///
    /// * `path`: Ruta al archivo que contiene la definición del laberinto.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use mi_libreria::Laberinto;
    ///
    /// let laberinto = Laberinto::cargar("laberinto.txt");
    /// ```
    pub fn cargar(path: &str) -> Result<Self, io::Error> {
        let lineas = Self::leer_lineas(path)?;
        let mut laberinto = Laberinto {
            tamano: 0,
            grid: Vec::new(),
        };

        for (fila_index, linea) in lineas.enumerate() {
            let linea_limpia = match linea {
                Ok(linea) => Self::eliminar_espacios(linea),
                Err(err) => return Err(err),
            };
            let fila = Self::cargar_laberinto_desde_linea(&linea_limpia, fila_index);
            laberinto.grid.push(fila);
            laberinto.tamano += 1;
        }

        let tamano = laberinto.tamano;

        if laberinto.grid.iter().all(|fila| fila.len() != tamano) {
            println!("Error: el tamaño del tablero es incorrecto\n",);
        }

        Ok(laberinto)
    }

    // Método privado para leer las líneas del archivo
    fn leer_lineas(path: &str) -> Result<io::Lines<BufReader<File>>, io::Error> {
        let file = File::open(path)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn parsear_entero(iter: &mut std::iter::Peekable<std::str::Chars>, default: i32) -> i32 {
        let mut valor = String::new();
        while let Some(&next_char) = iter.peek() {
            if next_char.is_ascii_digit() {
                valor.push(next_char);
                iter.next(); // Avanzar el iterador para consumir el dígito
            } else {
                break;
            }
        }

        match valor.parse::<i32>() {
            Ok(parsed_value) => parsed_value,
            Err(_) => default,
        }
    }

    // Método privado para eliminar espacios de una cadena
    fn eliminar_espacios(linea: String) -> String {
        linea.chars().filter(|c| !c.is_whitespace()).collect()
    }

    // Método privado para cargar un laberinto desde una línea de texto
    fn cargar_laberinto_desde_linea(linea: &str, fila_index: usize) -> Vec<Celda> {
        let mut fila = Vec::new();
        let mut iter = linea.chars().peekable();
        let mut col_index = 0; // Contador de columna

        while let Some(caracter) = iter.next() {
            let objeto = Self::cargar_objeto(caracter, &mut iter);
            fila.push(Celda {
                objeto,
                x: col_index,  // Establecemos la coordenada x
                y: fila_index, // Establecemos la coordenada y (fila)
            });
            col_index += 1;
        }
        fila
    }

    fn parsear_direccion(
        iter: &mut std::iter::Peekable<std::str::Chars>,
        default: Direccion,
    ) -> Direccion {
        if let Some(&next_char) = iter.peek() {
            let direccion = match next_char {
                'L' => Direccion::Izquierda,
                'R' => Direccion::Derecha,
                'U' => Direccion::Arriba,
                'D' => Direccion::Abajo,
                _ => default,
            };
            iter.next(); // Avanzar el iterador para consumir la dirección
            direccion
        } else {
            default
        }
    }

    // Método privado para cargar un objeto desde un carácter
    fn cargar_objeto(c: char, iter: &mut std::iter::Peekable<std::str::Chars>) -> Objeto {
        match c {
            'F' => {
                let puntos_vida = Self::parsear_entero(iter, 1);
                Objeto::Enemigo(puntos_vida)
            }
            'B' => {
                let alcance = Self::parsear_entero(iter, 0);
                Objeto::Bomba(alcance)
            }
            'S' => {
                let alcance = Self::parsear_entero(iter, 0);
                Objeto::BombaTraspaso(alcance)
            }
            'R' => Objeto::Roca,
            'W' => Objeto::Pared,
            'D' => {
                let direccion = Self::parsear_direccion(iter, Direccion::Arriba);
                Objeto::Desvio(direccion)
            }
            '_' => Objeto::Vacio,
            _ => Objeto::Vacio, // Caracter desconocido
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eliminar_espacios() {
        let linea = String::from("A B C1 D E2");
        let linea_limpia = Laberinto::eliminar_espacios(linea);
        assert_eq!(linea_limpia, "ABC1DE2");
    }

    #[test]
    fn test_cargar_objeto() {
        // Prueba para cargar diferentes objetos
        assert_eq!(
            Laberinto::cargar_objeto('F', &mut "3".chars().peekable()),
            Objeto::Enemigo(3)
        );
        assert_eq!(
            Laberinto::cargar_objeto('B', &mut "2".chars().peekable()),
            Objeto::Bomba(2)
        );
        assert_eq!(
            Laberinto::cargar_objeto('S', &mut "1".chars().peekable()),
            Objeto::BombaTraspaso(1)
        );
        assert_eq!(
            Laberinto::cargar_objeto('R', &mut "".chars().peekable()),
            Objeto::Roca
        );
        assert_eq!(
            Laberinto::cargar_objeto('W', &mut "".chars().peekable()),
            Objeto::Pared
        );
        assert_eq!(
            Laberinto::cargar_objeto('D', &mut "R".chars().peekable()),
            Objeto::Desvio(Direccion::Derecha)
        );
        assert_eq!(
            Laberinto::cargar_objeto('_', &mut "".chars().peekable()),
            Objeto::Vacio
        );
    }

    #[test]
    fn test_parsear_entero() {
        // Prueba para parsear enteros de una cadena
        assert_eq!(
            Laberinto::parsear_entero(&mut "123".chars().peekable(), 0),
            123
        );
        assert_eq!(
            Laberinto::parsear_entero(&mut "42abc".chars().peekable(), 0),
            42
        );
        assert_eq!(
            Laberinto::parsear_entero(&mut "abc".chars().peekable(), 10),
            10
        ); // El valor por defecto se usa en caso de error
    }

    #[test]
    fn test_parsear_direccion() {
        // Prueba para parsear direcciones de una cadena
        assert_eq!(
            Laberinto::parsear_direccion(&mut "LRU".chars().peekable(), Direccion::Abajo),
            Direccion::Izquierda
        );
        assert_eq!(
            Laberinto::parsear_direccion(&mut "R".chars().peekable(), Direccion::Arriba),
            Direccion::Derecha
        );
        assert_eq!(
            Laberinto::parsear_direccion(&mut "XYZ".chars().peekable(), Direccion::Izquierda),
            Direccion::Izquierda
        );
    }

    #[test]
    fn test_cargar_laberinto_desde_linea() {
        // Prueba para cargar un laberinto desde una cadena de línea
        let linea = "_WF5B1_".to_string();
        let fila = Laberinto::cargar_laberinto_desde_linea(&linea, 0);

        assert_eq!(
            fila,
            vec![
                Celda {
                    objeto: Objeto::Vacio,
                    x: 0,
                    y: 0
                },
                Celda {
                    objeto: Objeto::Pared,
                    x: 1,
                    y: 0
                },
                Celda {
                    objeto: Objeto::Enemigo(5),
                    x: 2,
                    y: 0
                },
                Celda {
                    objeto: Objeto::Bomba(1),
                    x: 3,
                    y: 0
                },
                Celda {
                    objeto: Objeto::Vacio,
                    x: 4,
                    y: 0
                },
            ]
        );
    }
}
