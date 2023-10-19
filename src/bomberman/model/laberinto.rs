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
        // Obtén el directorio actual
        let directorio_actual = std::env::current_dir()?;

        // Construye la ruta completa al archivo
        let ruta_completa = directorio_actual.join(path);

        // Comprueba si el archivo existe
        if !ruta_completa.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "El archivo no existe",
            ));
        }

        let ruta_completa_str = ruta_completa.to_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "La ruta del archivo no es válida",
            )
        })?;

        let lineas = Self::leer_lineas(ruta_completa_str)?;

        let mut laberinto = Laberinto {
            tamano: 0,
            grid: Vec::new(),
        };

        for (fila_index, linea) in lineas.enumerate() {
            let linea_limpia = match linea {
                Ok(linea) => Self::eliminar_espacios(linea),
                Err(err) => return Err(err),
            };
            let fila = Self::cargar_laberinto_desde_linea(&linea_limpia, fila_index)?;
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
        let file = File::open(path).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Error al abrir el archivo: {}", e))
        })?;
    
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
    fn cargar_laberinto_desde_linea(
        linea: &str,
        fila_index: usize,
    ) -> Result<Vec<Celda>, io::Error> {
        let mut fila = Vec::new();
        let mut iter = linea.chars().peekable();
        let mut col_index = 0; // Contador de columna

        while let Some(caracter) = iter.next() {
            let objeto = match Self::cargar_objeto(caracter, &mut iter) {
                Ok(obj) => obj,
                Err(err) => return Err(err),
            };

            fila.push(Celda {
                objeto,
                x: col_index,  // Establecemos la coordenada x
                y: fila_index, // Establecemos la coordenada y (fila)
            });
            col_index += 1;
        }

        Ok(fila)
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
    fn cargar_objeto(
        c: char,
        iter: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<Objeto, io::Error> {
        match c {
            'F' => {
                let puntos_vida = Self::parsear_entero(iter, 1);
                Ok(Objeto::Enemigo(puntos_vida))
            }
            'B' => {
                let alcance = Self::parsear_entero(iter, 0);
                Ok(Objeto::Bomba(alcance))
            }
            'S' => {
                let alcance = Self::parsear_entero(iter, 0);
                Ok(Objeto::BombaTraspaso(alcance))
            }
            'R' => Ok(Objeto::Roca),
            'W' => Ok(Objeto::Pared),
            'D' => {
                let direccion = Self::parsear_direccion(iter, Direccion::Arriba);
                Ok(Objeto::Desvio(direccion))
            }
            '_' => Ok(Objeto::Vacio),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Carácter desconocido",
            )),
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

}
