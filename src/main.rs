use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect(); // En esta linea hacemos un collect de los argumentos pasados desde la consola.
    
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Hubo un problema para obtener los argumentos: {err}");
        process::exit(1);
    });

    println!("Buscando por {}", config.query);
    println!("En el archivo {}\n", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Deberia ser capaz de leer el fichero");

    println!("El texto contenido: \n{contents}"); // Muestra el contenido del fichero que pasamos por parametro
    
    // dbg!(args); // Con esto podemos ver cuales son los argumenots pasados desde la consola

}

/// Creamos una estructura que nos ayuda a manejar la entrada de nuestro programa. 
struct Config{
    query: String,
    file_path: String,
}


///Esta funcion hace el parse de los argumentos enviados por consola al programa.
///Recibe un vec[String] con los argumentos pasados por consla para poder devolver el `query` y el `file_path` en el sgt formato: (query, file_path)
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("No hay arguemntos suficientes");
        }

        let query = args[1].clone();
        let file_path= args[2].clone();

        Ok(Config { query, file_path })
    }    

}
