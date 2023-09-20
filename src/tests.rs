// Pruebas integracion
#[cfg(test)]
mod tests {

    use crate::bomberman::{Celda, Objeto, cargar_laberinto, detonar_bomba};

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
                assert_eq!(primer_elemento, &Objeto::Bomba(2));
                let celda_prueba = &Celda { objeto: Objeto::Pared, x: 1, y: 1};
                let celda_test = &laberinto.grid[1][1];
                assert_eq!(celda_test, celda_prueba);
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
    /// Verifica que al detonar una bomba, se encuentren los objetos esperados en el 
    /// laberinto
    fn test_detonar_bomba_1(){
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
        
        match cargar_laberinto(ruta){
            Ok(mut laberinto) => {
                detonar_bomba(&mut laberinto, 4, 2);
                // Verificar que la celda de la bomba se haya vuelto Vacío
                let bomba = &laberinto.grid[2][4].objeto;
                assert_eq!(bomba, &Objeto::Vacio);

                // Verificar que la celda del enemigo se haya vuelto Vacío
                let enemigo = &laberinto.grid[0][4].objeto;
                assert_eq!(enemigo, &Objeto::Vacio);

                // Verificar que la celda de la pared siga siendo Pared
                let pared = &laberinto.grid[1][1].objeto;
                assert_eq!(pared, &Objeto::Pared);

                // Verificar que otras celdas no se vean afectadas
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
    #[test]
fn test_detonar_bomba_otro_laberinto() {
    
    let ruta = "ejemplo_2.txt";
    
    if let Ok(mut laberinto) = cargar_laberinto(ruta) {
        detonar_bomba(&mut laberinto, 2, 4);

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

    } else {
        eprintln!("Error al cargar el laberinto.");
        assert!(false);
    }
}

#[test]

fn test_detonar_bomba_3() {
    
    let ruta = "ejemplo_3.txt";
    
    if let Ok(mut laberinto) = cargar_laberinto(ruta) {
        detonar_bomba(&mut laberinto, 0, 4);

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

    } else {
        eprintln!("Error al cargar el laberinto.");
        assert!(false);
    }
}
   
}
