use crate::bomberman::{detonar_bomba, escribir_error_en_archivo, guardar_laberinto_en_archivo};
use bomberman::model::laberinto::Laberinto;
use std::env;
mod bomberman;

fn main() {
    if let Err(error) = run_program() {
        eprintln!("Error: {}", error);
        escribir_error_en_archivo("error.txt", &error);
    }
}

fn run_program() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        return Err("Argumentos inválidos".to_string());
    }

    let mut laberinto = Laberinto::cargar(&args[1])
        .map_err(|e| format!("Error al cargar el laberinto: {}", e))?;

    let x: usize = args[3]
        .parse()
        .map_err(|_| "No se pudo convertir x".to_string())?;

    let y: usize = args[4]
        .parse()
        .map_err(|_| "No se pudo convertir y".to_string())?;

    if let Err(e) = detonar_bomba(&mut laberinto, x, y) {
        escribir_error_en_archivo(&args[2], &format!("Error al detonar la bomba: {}", e));
        return Err(format!("Error al detonar la bomba: {}", e));
    }

    if let Err(e) = guardar_laberinto_en_archivo(&laberinto, &args[2], &args[1]) {
        escribir_error_en_archivo(&args[2], &format!("Error al guardar el laberinto: {}", e));
        return Err(format!("Error al guardar el laberinto: {}", e));
    }

    Ok(())
}



