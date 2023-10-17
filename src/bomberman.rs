use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
pub mod model;
use model::celda::Celda;
use model::direccion::Direccion;
use model::laberinto::Laberinto;
use model::objeto::Objeto;

fn quitar_vida_enemigo(actual: &mut Celda) {
    if let Objeto::Enemigo(mut vidas) = actual.objeto {
        // Restar 1 vida al enemigo
        vidas -= 1;
        // Si quedó con 0 vidas, cambiar a objeto vacío
        if vidas == 0 {
            actual.objeto = Objeto::Vacio;
            return;
        }
        actual.objeto = Objeto::Enemigo(vidas);
    }
}

fn obtener_direccion_del_desvio(celda: &Celda) -> Option<(i32, i32)> {
    if let Objeto::Desvio(direccion) = &celda.objeto {
        match direccion {
            Direccion::Izquierda => Some((-1, 0)),
            Direccion::Derecha => Some((1, 0)),
            Direccion::Arriba => Some((0, -1)),
            Direccion::Abajo => Some((0, 1)),
        }
    } else {
        None
    }
}
/// Detona una bomba en el laberinto y causa una explosión.
///
/// Esta función permite detonar una bomba en las coordenadas `(x, y)` del laberinto.
/// La explosión se propaga en cuatro direcciones: izquierda, derecha, arriba y abajo.
/// El alcance de la explosión está determinado por la potencia de la bomba, que puede ser de
/// dos tipos: bomba normal o bomba de traspaso. Si una bomba de traspaso alcanza una pared o
/// una roca, seguirá explotando al otro lado.
///
/// El laberinto se modifica como resultado de la explosión. Los enemigos y objetos cercanos
/// pueden ser destruidos o afectados por la explosión.
///
/// # Argumentos
///
/// - `laberinto`: Una referencia mutable al laberinto en el que se va a detonar la bomba.
/// - `x`: La coordenada X en la que se va a detonar la bomba.
/// - `y`: La coordenada Y en la que se va a detonar la bomba.
///
/// # Ejemplo
///
/// ```rust
/// use laberinto::{detonar_bomba, cargar_laberinto, Laberinto};
///
/// let mut laberinto = cargar_laberinto("laberinto.txt");
///
/// if let Ok(ref mut laberinto) = laberinto {
///     // Detonar una bomba en la coordenada (4, 2) del laberinto
///     detonar_bomba(laberinto, 4, 2);
///
///     // El laberinto ha sido modificado después de la explosión.
/// } else {
///     println!("Error al cargar el laberinto");
/// }
/// ```
///
/// # Notas
///
/// - Si las coordenadas `(x, y)` están fuera de los límites del laberinto, la función
///   imprimirá un mensaje de advertencia y no realizará ninguna acción.
/// - La explosión afecta a las celdas adyacentes dentro del alcance de la bomba.
/// - Las celdas afectadas por la explosión pueden cambiar su contenido, destruyendo enemigos
///   y objetos.
///
/// # Errores
///
/// Esta función no devuelve errores directamente, pero modifica el estado del laberinto.
/// Para verificar posibles errores, es importante verificar el estado del laberinto después
/// de llamar a esta función.

pub fn detonar_bomba(
    laberinto: &mut Laberinto,
    x: usize,
    y: usize,
) -> Result<(), io::Error> {
    if x >= laberinto.tamano || y >= laberinto.tamano {
        println!("Fuera de los parámetros del laberinto\n");
        return Err(io::Error::new(io::ErrorKind::Other, "Fuera de los parámetros del laberinto"));
    }

    let objeto = &laberinto.grid[y][x].objeto;

    // Verificar si la bomba que inicia la explosión es de traspaso
    let es_bomba_de_traspaso = matches!(objeto, Objeto::BombaTraspaso(_));

    match objeto {
        Objeto::Bomba(alcance) | Objeto::BombaTraspaso(alcance) => {
            detonar_bomba_recursive(laberinto, x, y, *alcance, es_bomba_de_traspaso);
        }
        _ => return Ok(()), // No es una bomba, no hacemos nada
    }
    laberinto.grid[y][x].objeto = Objeto::Vacio;
    Ok(())
}

fn detonar_bomba_recursive(
    laberinto: &mut Laberinto,
    x: usize,
    y: usize,
    alcance: i32,
    es_bomba_de_traspaso: bool,
) {
    laberinto.grid[y][x].objeto = Objeto::Vacio;
    let mut visited = vec![vec![false; laberinto.tamano]; laberinto.tamano];
    visited[y][x] = true;
    for &(mut dx, mut dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let mut new_x = x;
        let mut new_y = y;
        for _n in 1..=alcance {
            new_x = new_x.wrapping_add(dx as usize);
            new_y = new_y.wrapping_add(dy as usize);

            if new_x >= laberinto.tamano || new_y >= laberinto.tamano {
                break; // Salir del bucle si estamos fuera del laberinto o ya visitamos esta celda
            }

            if let Some((desvio_dx, desvio_dy)) =
                obtener_direccion_del_desvio(&laberinto.grid[new_y][new_x])
            {
                // Aplicar el desvío a las nuevas coordenadas
                new_x = new_x.wrapping_add(desvio_dx as usize);
                new_y = new_y.wrapping_add(desvio_dy as usize);
                dx = desvio_dx;
                dy = desvio_dy;
            }

            if (laberinto.grid[new_y][new_x].objeto == Objeto::Pared)
                || (laberinto.grid[new_y][new_x].objeto == Objeto::Roca && !es_bomba_de_traspaso)
            {
                break;
            }
            if !visited[new_y][new_x] {
                quitar_vida_enemigo(&mut laberinto.grid[new_y][new_x]);
                visited[new_y][new_x] = true;
            }

            if let Objeto::BombaTraspaso(_) | Objeto::Bomba(_) = laberinto.grid[new_y][new_x].objeto
            {
                match laberinto.grid[new_y][new_x].objeto {
                    Objeto::Bomba(alcance) => {
                        laberinto.grid[new_y][new_x].objeto = Objeto::Vacio;
                        detonar_bomba_recursive(laberinto, new_x, new_y, alcance, false);
                    }
                    Objeto::BombaTraspaso(alcance) => {
                        laberinto.grid[new_y][new_x].objeto = Objeto::Vacio;
                        detonar_bomba_recursive(laberinto, new_x, new_y, alcance, true);
                    }
                    _ => return, // No es una bomba, no hacemos nada
                }
            }
        }
    }
}

/// Guarda un laberinto en un archivo de texto.
///
/// Esta función toma un laberinto y lo guarda en un archivo de texto especificado.
/// Cada celda del laberinto se representa en el archivo como un carácter según el tipo de objeto
/// que contiene. Los objetos se representan de la siguiente manera:
///
/// - Enemigo: 'F' seguido del número de vidas.
/// - Bomba: 'B' seguido del alcance.
/// - Bomba de Traspaso: 'S' seguido del alcance.
/// - Roca: 'R'.
/// - Pared: 'W'.
/// - Desvío: 'D' seguido de la dirección ('L', 'R', 'U', 'D').
/// - Vacío: '_'.
///
/// Cada fila del laberinto se representa como una línea en el archivo, y los objetos de cada fila
/// se separan por espacios.
///
/// # Argumentos
///
/// - `laberinto`: Una referencia al laberinto que se va a guardar.
/// - `archivo_salida`: El nombre del archivo de texto en el que se guardará el laberinto.
///
/// # Ejemplo
///
/// ```rust
/// use laberinto::{guardar_laberinto_en_archivo, cargar_laberinto, Laberinto};
///
/// // Cargar un laberinto desde un archivo
/// let mut laberinto = cargar_laberinto("laberinto.txt");
///
/// if let Ok(ref mut laberinto) = laberinto {
///     // Realizar operaciones en el laberinto, como detonar bombas o mover personajes.
///
///     // Guardar el laberinto modificado en un archivo de texto
///     if let Err(e) = guardar_laberinto_en_archivo(laberinto, "laberinto_modificado.txt") {
///         eprintln!("Error al guardar el laberinto: {}", e);
///     } else {
///         println!("Laberinto guardado en {}", "laberinto_modificado.txt");
///     }
/// } else {
///     println!("Error al cargar el laberinto");
/// }
/// ```
///
/// # Errores
///
/// Esta función retorna un `Result` que puede contener un error si no se puede abrir o escribir en
/// el archivo especificado.
///
/// # Notas
///
/// - Si el archivo de salida no existe, se creará automáticamente.
/// - Cada fila en el archivo de salida se termina con un salto de línea '\n'.
///
/// # Ejemplo
///
/// ```rust
/// use laberinto::{guardar_laberinto_en_archivo, cargar_laberinto, Laberinto};
///
/// // Cargar un laberinto desde un archivo
/// let mut laberinto = cargar_laberinto("laberinto.txt");
///
/// if let Ok(ref mut laberinto) = laberinto {
///     // Realizar operaciones en el laberinto, como detonar bombas o mover personajes.
///
///     // Guardar el laberinto modificado en un archivo de texto
///     if let Err(e) = guardar_laberinto_en_archivo(laberinto, "laberinto_modificado.txt") {
///         eprintln!("Error al guardar el laberinto: {}", e);
///     } else {
///         println!("Laberinto guardado en {}", "laberinto_modificado.txt");
///     }
/// } else {
///     println!("Error al cargar el laberinto");
/// }
/// ```
///
/// # Errores
///
/// Esta función retorna un `Result` que puede contener un error si no se puede abrir o escribir en
/// el archivo especificado.
///
/// # Notas
///
/// - Si el archivo de salida no existe, se creará automáticamente.
/// - Cada fila en el archivo de salida se termina con un salto de línea '\n'.
///
/// # Notas
///
/// - Si el archivo de salida no existe, se creará automáticamente.
/// - Cada fila en el archivo de salida se termina con un salto de línea '\n'.

pub fn guardar_laberinto_en_archivo(
    laberinto: &Laberinto,
    dir_salida: &str,
    archivo_entrada: &str,
) -> Result<(), io::Error> {
    // Obtén el nombre del archivo de entrada
    let archivo_salida = Path::new(archivo_entrada)
        .file_name()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "No se pudo obtener el nombre del archivo de entrada",
            )
        })
        .and_then(|n| n.to_str().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "No se pudo convertir el nombre del archivo a cadena",
            )
        }))?;

    // Construye la ruta completa al archivo de salida
    let ruta_salida = Path::new(dir_salida).join(archivo_salida);

    // Crea el directorio de salida si no existe
    if let Some(dir) = ruta_salida.parent() {
        fs::create_dir_all(dir)?;
    }

    // Concatena las líneas en una cadena
    let mut contenido = String::new();

    for fila in &laberinto.grid {
        for celda in fila {
            match &celda.objeto {
                Objeto::Enemigo(vidas) => {
                    contenido.push_str(&format!("F{}", vidas));
                }
                Objeto::Bomba(alcance) => {
                    contenido.push_str(&format!("B{}", alcance));
                }
                Objeto::BombaTraspaso(alcance) => {
                    contenido.push_str(&format!("S{}", alcance));
                }
                Objeto::Roca => {
                    contenido.push_str("R");
                }
                Objeto::Pared => {
                    contenido.push_str("W");
                }
                Objeto::Desvio(direccion) => {
                    let dir_str = match direccion {
                        Direccion::Izquierda => "L",
                        Direccion::Derecha => "R",
                        Direccion::Arriba => "U",
                        Direccion::Abajo => "D",
                    };
                    contenido.push_str(&format!("D{}", dir_str));
                }
                Objeto::Vacio => {
                    contenido.push_str("_");
                }
            }
            if celda != &fila[laberinto.tamano - 1]{
                contenido.push_str(" ");
            }
        }
        if fila != &laberinto.grid[laberinto.tamano -1]{
            contenido.push('\n'); // Nueva línea para la siguiente fila
        }
    }

    // Abre el archivo de salida para escritura, creándolo si no existe.
    let mut archivo = match File::create(&ruta_salida) {
        Ok(file) => file,
        Err(e) => {
            // Si no se puede crear el archivo, intenta escribir el error en el archivo
            let mut archivo = File::create(&archivo_salida)?;
            archivo.write_all(format!("ERROR: {}", e).as_bytes())?;
            return Err(e);
        }
    };

    // Intenta escribir el contenido en el archivo
    if let Err(e) = archivo.write_all(contenido.as_bytes()) {
        // Si ocurre un error durante la escritura, intenta escribir el error en el archivo
        let mut archivo = File::create(&archivo_salida)?;
        archivo.write_all(format!("ERROR: {}", e).as_bytes())?;
        return Err(e);
    }

    Ok(())
}

pub fn escribir_error_en_archivo(archivo: &str, mensaje: &str) {
    if let Ok(mut archivo_error) = File::create(archivo) {
        if let Err(e) = writeln!(archivo_error, "ERROR: {}", mensaje) {
            eprintln!("Error al escribir el mensaje de error en el archivo: {}", e);
        }
    } else {
        eprintln!("Error al crear el archivo de error");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quitar_vida_a_enemigo() {
        // Crea una celda con un enemigo
        let mut celda = Celda {
            objeto: Objeto::Enemigo(3),
            x: 1,
            y: 1, // Un enemigo con 3 vidas
        };

        // Llama a la función para detonar la celda
        quitar_vida_enemigo(&mut celda);

        // Verifica que el enemigo haya perdido una vida
        assert!(matches!(celda.objeto, Objeto::Enemigo(2)));
    }

    #[test]
    fn test_obtener_direccion_del_desvio() {
        // Prueba para obtener dirección de desvío desde una celda
        let celda_con_desvio = Celda {
            objeto: Objeto::Desvio(Direccion::Arriba),
            x: 1,
            y: 1,
        };

        let celda_sin_desvio = Celda {
            objeto: Objeto::Pared,
            x: 2,
            y: 2,
        };

        assert_eq!(
            obtener_direccion_del_desvio(&celda_con_desvio),
            Some((0, -1)) // Debería ser una dirección hacia arriba
        );

        assert_eq!(
            obtener_direccion_del_desvio(&celda_sin_desvio),
            None // No debería haber dirección de desvío
        );
    }
}
