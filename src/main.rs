use std::env;
use std::error::Error;
use std::fs;
use std::process;
use int64grep::{search, search_case_insensitive, no_parameter, count};

fn main() {
    let args: Vec<String> = env::args().collect(); // En esta linea hacemos un collect de los argumentos pasados desde la consola.
    
    let _ = if args.len() <3 {
        eprintln!("{}", no_parameter());
        process::exit(1);
    };



    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Hubo un problema para obtener los argumentos: {err}");
        helps();
        process::exit(1);
    });



    println!("Buscando por: {}", config.query);
    println!("En el archivo: {}", config.file_path);
    println!("-----------------------------------------\n");

    if let Err(e) = run(config) {
        eprintln!("Error en la aplicacion: {e}");
        helps();
        process::exit(1);
    }
    
}

/// En esta funcion esta la logica de leer lo que esta en el fichero y de mostrar su contenido.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.helps {
        helps();
        return Ok(());
    }
    //println!("------El texto contenido------\n\n{contents}"); // Muestra el contenido del fichero que pasamos por parametro
    let results= if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    
    if config.line_count{
        let results_count=count(results.clone());
        for line in results_count{
            println!("{line}");
        }
        return Ok(());
    }


    for line in results{
        println!("{line}");
    }

    // dbg!(args); // Con esto podemos ver cuales son los argumenots pasados desde la consola
    Ok(())
}

fn helps()  {
    eprintln!("Opciones disponibles:");
    eprintln!("-i --ignore: Busqueda insensible a mayuscala/minuscula.");
    eprintln!("-c --count: Cuenta los match de linea.");
    eprintln!("-l --line: Indica la linea donde se encuentra.");

    
}

/// Creamos una estructura que nos ayuda a manejar la entrada de nuestro programa. 
struct Config{
    query: String,
    file_path: String,
    ignore_case: bool,
    line_count: bool,
    helps: bool,
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
        
        let ignore_case = Self::ignore_case();

        let mut config= Config{
            query:query,
            file_path:file_path,
            ignore_case:ignore_case,
            line_count:false,
            helps:false,
        };

        for i in args.iter(){
            if i=="-c" || i=="--count"{
                config.line_count=true;
            }
            if i=="-h" || i=="--help"{
                config.helps=true;
            }
            if i=="-i" || i=="--ignore"{
                config.ignore_case=true;
            }
            
        }


        Ok(config)
    }

    fn ignore_case() -> bool {
    // Esto se puede mejorar poniendolo en un vector y agregano los tipos ue se pueden
        let ignore_case_env = env::var("IGNORE_CASE").is_ok();

        ignore_case_env
    }    



}
