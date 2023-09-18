use std::error::{Error, self};
//use std::fmt::Error;
use std::fs::File;
use std::io::{self};
use std::io::{BufRead, Lines, BufReader};
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

#[derive(Debug, Clone)]
pub struct Celda{
   pub objeto: Objeto,
   pub x: usize,
   pub y: usize,
}

#[derive(Clone, Debug)]
pub struct Laberinto{
  tamano: usize,
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
        for n in 1..=alcance {

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

pub fn imprimir_laberinto(laberinto: &Laberinto) {
    for fila in &laberinto.grid {
        for celda in fila {
            match &celda.objeto {
                Objeto::Enemigo(vidas) => {
                    print!("F{} ", vidas);
                }
                Objeto::Bomba(alcance) => {
                    print!("B{} ", alcance);
                }
                Objeto::BombaTraspaso(alcance) => {
                    print!("S{}) ", alcance);
                }
                Objeto::Roca => {
                    print!("R ");
                }
                Objeto::Pared => {
                    print!("W ");
                }
                Objeto::Desvio(direccion) => {
                    let dir_str = match direccion {
                        Direccion::Izquierda => "L",
                        Direccion::Derecha => "R",
                        Direccion::Arriba => "U",
                        Direccion::Abajo => "D",
                    };
                    print!("D{} ", dir_str);
                }
                Objeto::Vacio => {
                    print!("_ ");
                }
            }
        }
        println!(); // Nueva línea para la siguiente fila
    }
}


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
    fn test_cargar_laberinto() {
        // Crea un archivo de ejemplo en tiempo de ejecución con el contenido que deseas
        // y obtén su ruta
        let contenido = "\
            B2 R R _ F1 _ _\n\
            _ W R W _ W _\n\
            B5 _ _ _ B2 _ _\n\
            _ W _ W _ W _\n\
            _ _ _ _ _ _ _\n\
            _ W _ W _ W _\n\
            _ _ _ _ _ _ _\n";
        
        let ruta = "laberinto.txt";
        
        std::fs::write(ruta, contenido).expect("No se pudo crear el archivo de prueba");
        
        // Llama a la función cargar_laberinto y verifica el resultado
        match cargar_laberinto(ruta) {
            Ok(laberinto) => {
                assert_eq!(laberinto.tamano, 7); // Verifica el tamaño esperado
                let primer_elemento = &laberinto.grid[0][0].objeto;
                assert_eq!(primer_elemento, &Objeto::Bomba(2))
                // Realiza más aserciones según tus necesidades
            }
            Err(e) => {
                // En caso de error, imprime el error para la depuración
                eprintln!("Error al cargar el laberinto: {:?}", e);
                assert!(false); // Indica que la prueba ha fallado
            }
        }
        
        // Limpia el archivo de prueba después de usarlo
        std::fs::remove_file(ruta).expect("No se pudo eliminar el archivo de prueba");
    }

    #[test]
    fn test_detonar_bomba(){
        let contenido = "\
            B2 R R _ F1 _ _\n\
            _ W R W _ W _\n\
            B5 _ _ _ B2 _ _\n\
            _ W _ W _ W _\n\
            _ _ _ _ _ _ _\n\
            _ W _ W _ W _\n\
            _ _ _ _ _ _ _\n";
        
        let ruta = "laberinto.txt";
        
        std::fs::write(ruta, contenido).expect("No se pudo crear el archivo de prueba");
        
        // Llama a la función cargar_laberinto y verifica el resultado
        match cargar_laberinto(ruta){
            Ok(mut laberinto) => {
                detonar_bomba(&mut laberinto, 4, 2);
                let bomba = &laberinto.grid[2][4].objeto;
                let enemigo = &laberinto.grid[0][4].objeto;
                assert_eq!(bomba, &Objeto::Vacio);
                assert_eq!(enemigo, &Objeto::Vacio);
            }
            Err(e) => {
                eprintln!("Error al cargar el laberinto: {:?}", e);
                assert!(false);
            }
        }
        


    }
}
