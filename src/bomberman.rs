use std::fs::File;
use std::io::{self};
use std::io::{BufRead, BufReader};
use std::io::{Result, Write};


#[derive(Debug, PartialEq, Clone)]
pub enum Objeto {
    Enemigo(i32),
    Bomba(i32),
    BombaTraspaso(i32),
    Roca,
    Pared,
    Desvio(Direccion),
    Vacio,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Direccion{
   Izquierda,
   Derecha,
   Arriba,
  Abajo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Celda{
   pub objeto: Objeto,
   pub x: usize,
   pub y: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Laberinto{
  pub tamano: usize,
  pub grid: Vec<Vec<Celda>>
}

// Abre el archivo y devuelve la lineas del mismo o error dado el caso
fn leer_lineas(path: &str) -> Result<io::Lines<BufReader<File>>> {
    match File::open(path){
        Ok(archivo) => Ok(io::BufReader::new(archivo).lines()),
        Err(error) => Err(error),
    }
}

fn cargar_objeto(c: char, iter: &mut std::iter::Peekable<std::str::Chars>) -> Objeto {
    match c {
        'F' => {
            let puntos_vida = parsear_entero(iter, 1);
            Objeto::Enemigo(puntos_vida)
        }
        'B' => {
            let alcance = parsear_entero(iter, 0);
            Objeto::Bomba(alcance)
        }
        'S' => {
            let alcance = parsear_entero(iter, 0);
            Objeto::BombaTraspaso(alcance)
        }
        'R' => Objeto::Roca,
        'W' => Objeto::Pared,
        'D' => {
            let direccion = parsear_direccion(iter, Direccion::Arriba);
            Objeto::Desvio(direccion)
        }
        '_' => Objeto::Vacio,
        _ => Objeto::Vacio, // Caracter desconocido
    }
}

fn parsear_entero(iter: &mut std::iter::Peekable<std::str::Chars>, default: i32) -> i32 {
    let mut valor = String::new();
    while let Some(&next_char) = iter.peek() {
        if next_char.is_digit(10) {
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

fn cargar_laberinto_desde_linea(linea: &str, fila_index: usize) -> Vec<Celda> {
    let mut fila = Vec::new();
    let mut iter = linea.chars().peekable();
    let mut col_index = 0; // Contador de columna

    while let Some(caracter) = iter.next() {
        let objeto = cargar_objeto(caracter, &mut iter);
        fila.push(Celda {
            objeto,
            x: col_index,   // Establecemos la coordenada x
            y: fila_index,  // Establecemos la coordenada y (fila)
        });
        col_index += 1;
    }
    fila
}

fn eliminar_espacios(linea: String) -> String {
    linea.chars().filter(|c| !c.is_whitespace()).collect::<String>()
}

/// Carga un laberinto desde un archivo de texto y lo representa como una estructura de datos.
///
/// Esta función toma la ruta de un archivo de texto como entrada y carga el contenido del
/// archivo en un laberinto, representado como una estructura de datos `Laberinto`. El archivo
/// debe contener una representación textual del laberinto, donde cada carácter representa un
/// objeto en una celda del laberinto. Los objetos posibles incluyen enemigos, bombas, rocas,
/// paredes, desvíos y celdas vacías.
///
/// # Argumentos
///
/// - `path`: La ruta del archivo de texto que contiene el laberinto.
///
/// # Errores
///
/// Esta función puede devolver un error si ocurren problemas al abrir o leer el archivo.
///
/// # Ejemplo
///
/// ```rust
/// use laberinto::{cargar_laberinto, Laberinto};
///
/// let laberinto = cargar_laberinto("laberinto.txt");
///
/// match laberinto {
///     Ok(l) => {
///         println!("Laberinto cargado con éxito.");
///         // Realizar operaciones con el laberinto cargado
///     }
///     Err(e) => {
///         eprintln!("Error al cargar el laberinto: {:?}", e);
///     }
/// }
/// ```
///
/// # Notas
///
/// - El archivo de entrada debe tener un formato específico donde cada línea representa una fila
///   del laberinto y cada carácter en una línea representa un objeto en una celda.
/// - La función realiza una validación del tamaño del laberinto verificando que todas las filas
///   tengan la misma longitud.
///
/// # Devoluciones
///
/// - `Ok(Laberinto)`: Un resultado que contiene el laberinto cargado como una estructura `Laberinto`
///   si se realizó con éxito.
/// - `Err(std::io::Error)`: Un error que indica un problema al abrir o leer el archivo.
///

pub fn cargar_laberinto(path: &str) -> Result<Laberinto> {
    let lineas = leer_lineas(path)?;
    let mut laberinto = Laberinto {
        tamano: 0,
        grid: Vec::new(),
    };

    for (fila_index, linea) in lineas.enumerate() {
        let linea_limpia = match linea {
            Ok(linea) => eliminar_espacios(linea),
            Err(err) => return Err(err),
        }; 
        let fila = cargar_laberinto_desde_linea(&linea_limpia, fila_index);
        laberinto.grid.push(fila);
        laberinto.tamano += 1;
    }

    let tamano = laberinto.tamano;
    
    if laberinto.grid.iter().all(|fila| fila.len() != tamano){
        println!("Error: el tamaño del tablero es incorrecto\n",);
    }

    Ok(laberinto)
}

fn quitar_vida_enemigo(actual: &mut Celda){
    match actual.objeto {
        Objeto::Enemigo(mut vidas) => {
            // Restar 1 vida al enemigo
            vidas -= 1;  
            // Si quedó con 0 vidas, cambiar a objeto vacío
            if vidas == 0 {
                actual.objeto = Objeto::Vacio;
                return;
            }
            actual.objeto = Objeto::Enemigo(vidas);      
        }
        
        _ => return,
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
/// let mut laberinto = cargar_laberinto("laberinto.txt").expect("Error al cargar el laberinto");
///
/// // Detonar una bomba en la coordenada (4, 2) del laberinto
/// detonar_bomba(&mut laberinto, 4, 2);
///
/// // El laberinto ha sido modificado después de la explosión.
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
pub fn detonar_bomba(laberinto: &mut Laberinto, x: usize, y: usize) {
    if x >= laberinto.tamano || y >= laberinto.tamano {
        println!("Fuera de los parametros del laberinto\n");
        return;
    }

    let objeto = &laberinto.grid[y][x].objeto;

     // Verificar si la bomba que inicia la explosión es de traspaso
     let es_bomba_de_traspaso = match objeto {
        Objeto::BombaTraspaso(_) => true,
        _ => false,
    };

    match objeto {
        Objeto::Bomba(alcance) | Objeto::BombaTraspaso(alcance) => {
            let mut visited = vec![vec![false; laberinto.tamano]; laberinto.tamano];
            visited[y][x] = true;
            
            detonar_bomba_recursive(laberinto, x, y, *alcance, &mut visited, es_bomba_de_traspaso);
        }
        _ => return, // No es una bomba, no hacemos nada
    }
    laberinto.grid[y][x].objeto = Objeto::Vacio;
}

fn detonar_bomba_recursive(
    laberinto: &mut Laberinto,
    x: usize,
    y: usize,
    alcance: i32,
    mut visited: &mut Vec<Vec<bool>>,
    es_bomba_de_traspaso: bool,
) {    
    for &(mut dx, mut dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let mut new_x = x;
        let mut new_y = y;
        for _n in 1..=alcance {

            new_x = new_x.wrapping_add((dx as i32) as usize);
            new_y = new_y.wrapping_add((dy as i32) as usize);
            
            if new_x >= laberinto.tamano || new_y >= laberinto.tamano || visited[new_y][new_x] {
                break; // Salir del bucle si estamos fuera del laberinto o ya visitamos esta celda
            }

            if let Some((desvio_dx, desvio_dy)) = obtener_direccion_del_desvio(&laberinto.grid[new_y][new_x]) {
                // Aplicar el desvío a las nuevas coordenadas
                new_x = new_x.wrapping_add(desvio_dx as usize);
                new_y = new_y.wrapping_add(desvio_dy as usize);
                dx = desvio_dx;
                dy = desvio_dy;
            }

            if (laberinto.grid[new_y][new_x].objeto == Objeto::Pared) || (laberinto.grid[new_y][new_x].objeto == Objeto::Roca && !es_bomba_de_traspaso ){
                break;
            }
            quitar_vida_enemigo(&mut laberinto.grid[new_y][new_x]);
            if let Objeto::Enemigo(_vidas) = laberinto.grid[new_y][new_x].objeto {
                visited[new_y][new_x] = false; // Quiero que vuelva a visitarla por si hay un enemigo
        
            } else {
                visited[new_y][new_x] = true;
            }

            if let Objeto::BombaTraspaso(_) | Objeto::Bomba(_) = laberinto.grid[new_y][new_x].objeto {
                match laberinto.grid[new_y][new_x].objeto {
                    Objeto::Bomba(alcance) => {
                        laberinto.grid[new_y][new_x].objeto = Objeto::Vacio;
                        detonar_bomba_recursive(laberinto, new_x, new_y, alcance, &mut visited, false);
                    }
                    Objeto::BombaTraspaso(alcance) => {
                        laberinto.grid[new_y][new_x].objeto = Objeto::Vacio;
                        detonar_bomba_recursive(laberinto, new_x, new_y, alcance, &mut visited, true);
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
/// ```
/// use laberinto::{guardar_laberinto_en_archivo, cargar_laberinto, Laberinto};
///
/// // Cargar un laberinto desde un archivo
/// let mut laberinto = cargar_laberinto("laberinto.txt").expect("Error al cargar el laberinto");
///
/// // Realizar operaciones en el laberinto, como detonar bombas o mover personajes.
///
/// // Guardar el laberinto modificado en un archivo de texto
/// guardar_laberinto_en_archivo(&laberinto, "laberinto_modificado.txt").expect("Error al guardar el laberinto");
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
pub fn guardar_laberinto_en_archivo(laberinto: &Laberinto, archivo_salida: &str) -> Result<()> {
    // Abre el archivo de salida para escritura, creándolo si no existe.
    let mut archivo = File::create(archivo_salida)?;

    for fila in &laberinto.grid {
        for celda in fila {
            match &celda.objeto {
                Objeto::Enemigo(vidas) => {
                    write!(archivo, "F{} ", vidas)?;
                }
                Objeto::Bomba(alcance) => {
                    write!(archivo, "B{} ", alcance)?;
                }
                Objeto::BombaTraspaso(alcance) => {
                    write!(archivo, "S{}) ", alcance)?;
                }
                Objeto::Roca => {
                    write!(archivo, "R ")?;
                }
                Objeto::Pared => {
                    write!(archivo, "W ")?;
                }
                Objeto::Desvio(direccion) => {
                    let dir_str = match direccion {
                        Direccion::Izquierda => "L",
                        Direccion::Derecha => "R",
                        Direccion::Arriba => "U",
                        Direccion::Abajo => "D",
                    };
                    write!(archivo, "D{} ", dir_str)?;
                }
                Objeto::Vacio => {
                    write!(archivo, "_ ")?;
                }
            }
        }
        writeln!(archivo)?; // Nueva línea para la siguiente fila
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_eliminar_espacios(){
        let linea = String::from("A B C1 D E2");
        let linea_limpia = eliminar_espacios(linea);
        assert_eq!(linea_limpia, "ABC1DE2");
    }
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
    fn test_cargar_objeto() {
        // Prueba para cargar diferentes objetos
        assert_eq!(cargar_objeto('F', &mut "3".chars().peekable()), Objeto::Enemigo(3));
        assert_eq!(cargar_objeto('B', &mut "2".chars().peekable()), Objeto::Bomba(2));
        assert_eq!(cargar_objeto('S', &mut "1".chars().peekable()), Objeto::BombaTraspaso(1));
        assert_eq!(cargar_objeto('R', &mut "".chars().peekable()), Objeto::Roca);
        assert_eq!(cargar_objeto('W', &mut "".chars().peekable()), Objeto::Pared);
        assert_eq!(cargar_objeto('D', &mut "R".chars().peekable()), Objeto::Desvio(Direccion::Derecha));
        assert_eq!(cargar_objeto('_', &mut "".chars().peekable()), Objeto::Vacio);
    }

    #[test]
    fn test_parsear_entero() {
        // Prueba para parsear enteros de una cadena
        assert_eq!(parsear_entero(&mut "123".chars().peekable(), 0), 123);
        assert_eq!(parsear_entero(&mut "42abc".chars().peekable(), 0), 42);
        assert_eq!(parsear_entero(&mut "abc".chars().peekable(), 10), 10); // El valor por defecto se usa en caso de error
    }

    #[test]
    fn test_parsear_direccion() {
        // Prueba para parsear direcciones de una cadena
        assert_eq!(parsear_direccion(&mut "LRU".chars().peekable(), Direccion::Abajo), Direccion::Izquierda);
        assert_eq!(parsear_direccion(&mut "R".chars().peekable(), Direccion::Arriba), Direccion::Derecha);
        assert_eq!(parsear_direccion(&mut "XYZ".chars().peekable(), Direccion::Izquierda), Direccion::Izquierda);
    }

    #[test]
    fn test_cargar_laberinto_desde_linea() {
        // Prueba para cargar un laberinto desde una cadena de línea
        let linea = "_WF5B1_".to_string();
        let fila = cargar_laberinto_desde_linea(&linea, 0);

        assert_eq!(
            fila,
            vec![
                Celda { objeto: Objeto::Vacio, x: 0, y: 0 },
                Celda { objeto: Objeto::Pared, x: 1, y: 0 },
                Celda { objeto: Objeto::Enemigo(5), x: 2, y: 0 },
                Celda { objeto: Objeto::Bomba(1), x: 3, y: 0 },
                Celda { objeto: Objeto::Vacio, x: 4, y: 0 },
            ]
        );
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
