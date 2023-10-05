use crate::bomberman::{detonar_bomba, escribir_error_en_archivo, guardar_laberinto_en_archivo};
use bomberman::model::laberinto::Laberinto;
use std::env;
mod bomberman;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("ERROR: argumentos inválidos");
        process::exit(1); // Salir del programa con un código de salida no cero
    }

    if let Ok(mut laberinto) = Laberinto::cargar(&args[1]) {
        let x: usize = match args[3].parse() {
            Ok(valor) => valor,
            Err(_) => {
                let error_msg = "No se pudo convertir x";
                println!("{}", error_msg);
                escribir_error_en_archivo(&args[2], error_msg);
                process::exit(1); // Salir del programa con un código de salida no cero
            }
        };

        let y: usize = match args[4].parse() {
            Ok(valor) => valor,
            Err(_) => {
                let error_msg = "No se pudo convertir y";
                println!("{}", error_msg);
                escribir_error_en_archivo(&args[2], error_msg);
                process::exit(1); // Salir del programa con un código de salida no cero
            }
        };

        detonar_bomba(&mut laberinto, x, y);

        match guardar_laberinto_en_archivo(&laberinto, &args[2], &args[1]) {
            Ok(_) => println!("Laberinto guardado en {}", &args[2]),
            Err(e) => {
                let error_msg = format!("Error al guardar el laberinto: {}", e);
                eprintln!("{}", error_msg);
                escribir_error_en_archivo(&args[2], &error_msg);
                process::exit(1); // Salir del programa con un código de salida no cero
            }
        }
    } else {
        println!("El laberinto dio error");
        escribir_error_en_archivo(&args[2], "El laberinto dio error");
        process::exit(1); // Salir del programa con un código de salida no cero
    }
}
