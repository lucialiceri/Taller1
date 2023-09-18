use std::env;

use crate::bomberman::{cargar_laberinto, detonar_bomba, imprimir_laberinto, guardar_laberinto_en_archivo};
mod bomberman;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5{
        print!("ERROR: argumentos invalidos\n")
    } else if let Ok(mut laberinto) = cargar_laberinto(&args[1]){
         {
            let x: usize = match args[3].parse(){
                Ok(valor) => valor,
                Err(_) => {
                    println!("No se pudo convertir x\n");
                    std::process::exit(1);
                }
            };
            let y: usize = match args[4].parse(){
                Ok(valor) => valor,
                Err(_) => {
                    println!("No se pudo convertir y\n");
                    std::process::exit(1);
                }
            };
            detonar_bomba(&mut laberinto, x, y);
            imprimir_laberinto(&laberinto);
            match guardar_laberinto_en_archivo(&laberinto, &args[2]) {
                Ok(_) => println!("Laberinto guardado en {}", &args[2]),
                Err(e) => eprintln!("Error al guardar el laberinto: {}", e),
            }
        }
    }
    else {
        println!("El laberinto dio error\n");
    }

}
