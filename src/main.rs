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

use int64grep::{count, search, search_case_insensitive, search_line_count};
use std::error::Error;
use clap::Parser;

use std::fs;
use std::process;


/// Creamos una estructura que nos ayuda a manejar la entrada de nuestro programa.
/// Configuration parsed from command-line arguments.
///
/// Fields correspond to CLI options and positional arguments.
#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Config {
    query: String,
    file_path: String,
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
    #[arg(short, long, default_value_t = false)]
    line_count: bool,
    #[arg(short, long, default_value_t = false)]
    count: bool,
}

fn main() {
    let args = Config::parse(); // En esta linea hacemos un collect de los argumentos pasados desde la consola.

    run(args).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
}

/// En esta funcion esta la logica de leer lo que esta en el fichero y de mostrar su contenido.
/// Run the application with a parsed `Config`.
///
/// Reads the file, performs the chosen search and prints formatted results according to the
/// selected options in `config`.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

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
