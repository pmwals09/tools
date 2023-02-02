mod adapters;
use crate::adapters::js::{self, *};
use clap::{Parser, Subcommand};

/// Generalized code generation
#[derive(Parser, Debug)]
struct Cli {
    /// Language adapter to use
    #[command(subcommand)]
    lang: Language,
    /// The path to the output file or folder
    #[arg(short, long)]
    out: Option<std::path::PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Language {
    /// Utilizes the JS adapter
    JS(JS),
}

fn main() {
    let args = Cli::parse();

    let files_to_write = match args.lang {
        Language::JS(config) => js::main(config),
    };

    for file in files_to_write {
        match args.out {
            Some(ref path) => {
                let mut full_path = path.clone();
                full_path.push(file.name);

                std::fs::write(full_path, file.contents).expect("Error writing file contents.");
            },
            None => println!("{}", file.contents),
        }
    }
}
