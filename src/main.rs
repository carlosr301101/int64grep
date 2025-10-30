//! `int64grep` — a tiny CLI that demonstrates the search helpers from this crate.
//!
//! Usage: `int64grep  <query> <file> [OPTIONS]`
//!
//! Options:
//! - `-i`, `--ignore` — case-insensitive search
//! - `-c`, `--count` — print numbered matches (relative numbering)
//! - `-l`, `--line-count` — print original file line numbers with matches exclude[-c]
//!
//! The binary uses functions exported by the crate for the core search logic, keeping `main.rs`
//! focused on argument parsing and output formatting.

use int64grep::{count, no_parameter, search, search_case_insensitive, search_line_count};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // En esta linea hacemos un collect de los argumentos pasados desde la consola.

    if args.len() < 3 {
        eprintln!("{}", no_parameter());
        process::exit(1);
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
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
/// Run the application with a parsed `Config`.
///
/// Reads the file, performs the chosen search and prints formatted results according to the
/// selected options in `config`.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.helps {
        helps();
        return Ok(());
    }

    // ------- This is like a break point , Idk how to match the typos on the functions like search_line_count and search_case and search
    if config.line_count {
        let results_count = search_line_count(&config.query, &contents);
        for line in results_count {
            println!("{line}");
        }
        return Ok(());
    }

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if config.count {
        let results_count = count(results.clone());
        for line in results_count {
            println!("{line}");
        }
        return Ok(());
    }

    for line in results {
        println!("{line}");
    }

    // dbg!(args); // Con esto podemos ver cuales son los argumenots pasados desde la consola
    Ok(())
}

/// Print a short help message describing available command-line options.
fn helps() {
    eprintln!("\nUso: int64grep <query> <file> [OPTIONS] \n");
    eprintln!("Opciones disponibles:");
    eprintln!("-i --ignore: Busqueda insensible a mayuscala/minuscula.");
    eprintln!("-c --count: Cuenta los match de linea.");
    eprintln!("-l --line-count: Indica la linea donde se encuentra. Exclude [-c]\n");

}
/// Creamos una estructura que nos ayuda a manejar la entrada de nuestro programa.
/// Configuration parsed from command-line arguments.
///
/// Fields correspond to CLI options and positional arguments.
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    line_count: bool,
    helps: bool,
    count: bool,
}

///Esta funcion hace el parse de los argumentos enviados por consola al programa.
///Recibe un vec[String] con los argumentos pasados por consla para poder devolver el `query` y el `file_path` en el sgt formato: (query, file_path)
impl Config {
    /// Build a `Config` from program arguments.
    ///
    /// This function expects at least two positional arguments (`query` and `file_path`) and
    /// scans for optional flags. Returns `Err` if required arguments are missing.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        
        if args.len() < 3 {
            eprintln!("{}", no_parameter());
            process::exit(1);
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = Self::ignore_case();

        let mut config = Config {
            query,
            file_path,
            ignore_case,
            line_count: false,
            helps: false,
            count: false,
        };

        for i in args.iter() {
            if i == "-c" || i == "--count" {
                config.count = true;
            }
            if i == "-h" || i == "--help" {
                config.helps = true;
            }
            if i == "-i" || i == "--ignore" {
                config.ignore_case = true;
            }
            if i == "-l" || i == "--line-count" {
                config.line_count = true;
            }
        }

        Ok(config)
    }

    fn ignore_case() -> bool {
        // Esto se puede mejorar poniendolo en un vector y agregano los tipos ue se pueden
        env::var("IGNORE_CASE").is_ok()

    }
}
