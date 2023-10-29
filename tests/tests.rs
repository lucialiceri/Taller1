use bomberman_r::bomberman::{
    detonar_bomba, model::celda::Celda, model::laberinto::Laberinto, model::objeto::Objeto,
};

#[test]
fn test_cargar_laberinto() {
    let contenido = "\
        B2 R R _ F1 _ _\n\
        _ W R W _ W _\n\
        B5 _ _ _ B2 _ _\n\
        _ W _ W _ W _\n\
        _ _ _ _ _ _ _\n\
        _ W _ W _ W _\n\
        _ _ _ _ _ _ _\n";

    let ruta = "laberinto.txt";

    match std::fs::write(ruta, contenido) {
        Ok(_) => {
            match Laberinto::cargar(ruta) {
                Ok(laberinto) => {
                    assert_eq!(laberinto.tamano, 7);
                    let primer_elemento = &laberinto.grid[0][0].objeto;
                    assert_eq!(primer_elemento, &Objeto::Bomba(2));
                    let celda_prueba = Celda {
                        objeto: Objeto::Pared,
                        x: 1,
                        y: 1,
                    };
                    let celda_test = &laberinto.grid[1][1];
                    assert_eq!(celda_test, &celda_prueba);
                }
                Err(e) => {
                    eprintln!("Error al cargar el laberinto: {:?}", e);
                    assert!(false);
                }
            }
        }
        Err(e) => {
            eprintln!("Error al escribir en el archivo de prueba: {:?}", e);
            assert!(false);
        }
    }

    match std::fs::remove_file(ruta) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error al eliminar el archivo de prueba: {:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn test_detonar_bomba_1() {
    let contenido = "\
        B2 R R _ F1 _ _\n\
        _ W R W _ W _\n\
        B5 _ _ _ B2 _ _\n\
        _ W _ W _ W _\n\
        _ _ _ _ _ _ _\n\
        _ W _ W _ W _\n\
        _ _ _ _ _ _ _\n";

    let ruta = "laberinto.txt";

    match std::fs::write(ruta, contenido) {
        Ok(_) => {
            match Laberinto::cargar(ruta) {
                Ok(mut laberinto) => {
                    let _ = detonar_bomba(&mut laberinto, 4, 2);
                    let bomba = &laberinto.grid[2][4].objeto;
                    assert_eq!(bomba, &Objeto::Vacio);
                    let enemigo = &laberinto.grid[0][4].objeto;
                    assert_eq!(enemigo, &Objeto::Vacio);
                    let pared = &laberinto.grid[1][1].objeto;
                    assert_eq!(pared, &Objeto::Pared);
                    let celda1 = &laberinto.grid[0][0].objeto;
                    assert_ne!(celda1, &Objeto::Vacio);
                    let celda2 = &laberinto.grid[1][2].objeto;
                    assert_ne!(celda2, &Objeto::Vacio);
                }
                Err(e) => {
                    eprintln!("Error al cargar el laberinto: {:?}", e);
                    assert!(false);
                }
            }
        }
        Err(e) => {
            eprintln!("Error al escribir en el archivo de prueba: {:?}", e);
            assert!(false);
        }
    }

    match std::fs::remove_file(ruta) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error al eliminar el archivo de prueba: {:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn test_detonar_bomba_otro_laberinto() {
    let ruta = "ejemplos/ejemplo_2.txt";

    let laberinto_result = Laberinto::cargar(ruta);
    assert!(laberinto_result.is_ok());
    let mut laberinto = laberinto_result.unwrap();

    let detonar_result = detonar_bomba(&mut laberinto, 2, 4);
    assert!(detonar_result.is_ok());

    // Verificar que la celda B2 se haya vuelto Vacío
    let celda1 = &laberinto.grid[0][2].objeto;
    assert_eq!(celda1, &Objeto::Vacio);

    // Verificar que la celda B1 se haya vuelto Vacío
    let celda2 = &laberinto.grid[0][4].objeto;
    assert_eq!(celda2, &Objeto::Vacio);

    // Verificar que otras celdas no se vean afectadas
    let celda3 = &laberinto.grid[1][0].objeto;
    assert_eq!(celda3, &Objeto::Vacio);

    let celda4 = &laberinto.grid[2][3].objeto;
    assert_eq!(celda4, &Objeto::Roca);

    let enemigo = &laberinto.grid[2][4].objeto;
    assert_eq!(enemigo, &Objeto::Enemigo(1));

    let ultima_celda = &laberinto.grid[6][6].objeto;
    assert_eq!(ultima_celda, &Objeto::Bomba(1));
}

#[test]
fn test_detonar_bomba_3() {
    let ruta = "ejemplos/ejemplo_3.txt";

    let laberinto_result = Laberinto::cargar(ruta);
    assert!(laberinto_result.is_ok());
    let mut laberinto = laberinto_result.unwrap();

    let detonar_result = detonar_bomba(&mut laberinto, 0, 4);
    assert!(detonar_result.is_ok());

    let celda1 = &laberinto.grid[0][2].objeto;
    assert_eq!(celda1, &Objeto::Vacio);

    let celda2 = &laberinto.grid[0][4].objeto;
    assert_eq!(celda2, &Objeto::Vacio);

    // Verificar que otras celdas no se vean afectadas
    let celda3 = &laberinto.grid[1][0].objeto;
    assert_eq!(celda3, &Objeto::Vacio);

    let celda4 = &laberinto.grid[2][3].objeto;
    assert_eq!(celda4, &Objeto::Roca);

    let enemigo = &laberinto.grid[2][4].objeto;
    assert_eq!(enemigo, &Objeto::Vacio);
}
