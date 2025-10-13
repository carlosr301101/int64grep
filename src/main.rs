use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::string;
use std::sync::atomic;
use int64grep::search;
use int64grep::search_case_insensitive;

fn main() {
    let args: Vec<String> = env::args().collect(); // En esta linea hacemos un collect de los argumentos pasados desde la consola.
    
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Hubo un problema para obtener los argumentos: {err}");
        process::exit(1);
    });

    println!("Buscando por {}", config.query);
    println!("En el archivo {}\n", config.file_path);

    if let Err(e) = run(config) {
        println!("Error en la aplicacion: {e}");
        process::exit(1);
    }
    
}

/// En esta funcion esta la logica de leer lo que esta en el fichero y de mostrar su contenido.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    //println!("------El texto contenido------\n\n{contents}"); // Muestra el contenido del fichero que pasamos por parametro
    let results= if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };


    for line in results{
        println!("{line}");
    }

    // dbg!(args); // Con esto podemos ver cuales son los argumenots pasados desde la consola
    Ok(())
}

/// Creamos una estructura que nos ayuda a manejar la entrada de nuestro programa. 
struct Config{
    query: String,
    file_path: String,
    ignore_case: bool,
}


///Esta funcion hace el parse de los argumentos enviados por consola al programa.
///Recibe un vec[String] con los argumentos pasados por consla para poder devolver el `query` y el `file_path` en el sgt formato: (query, file_path)
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("No hay arguemntos suficientes");
        }

        let query = args[1].clone();
        let file_path= args[2].clone();
        let ignore_case = Self::ignore_case(args);

        Ok(Config { query, file_path, ignore_case })
    }

    fn ignore_case(args: &[String]) -> bool{
        let arg_ignore = "--ignore".to_string();// Esto se puede mejorar poniendolo en un vector y agregano los tipos ue se pueden

        let ignore_case_env = env::var("IGNORE_CASE").is_ok();
        let ignore_case_arg=args.contains(&arg_ignore);

        return ignore_case_arg||ignore_case_env;

    }    

}
