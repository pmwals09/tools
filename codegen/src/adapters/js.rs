use clap::{Args, ValueEnum};
use crate::adapters::shared::FileResponse;

#[derive(Args, Debug)]
pub struct JS {
    /// Generator to use
    #[arg(short, long, value_enum)]
    generator: JSGenerator,
    /// Filename and generated item name
    #[arg(short, long)]
    name: String,
    /// Any additional properties relevant to the generated item
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    properties: Vec<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum JSGenerator {
    Class,
    ReactComponent,
}

pub fn main(config: JS) -> Vec<FileResponse> {
    return match config.generator {
        JSGenerator::Class => generate_class(config),
        JSGenerator::ReactComponent => generate_react_component(config),
    };
}

fn generate_class(config: JS) -> Vec<FileResponse> {
    println!("Generating class...");
    let mut files = vec![];

    // Ensure that the class name is at least uppercase
    let formatted_name = capitalize(&config.name);


    // Class declaration
    let mut c = format!("class {} {{\n", &formatted_name);

    // Constructor method
    c += "  constructor({ ";
    for prop in &config.properties {
        c += format!("{}, ", prop).as_str();
    }

    // Drop the comma-space from the last one
    c.pop();
    c.pop();
    c += " }) {\n";
    for prop in &config.properties {
        c += format!("    this._{} = {};\n", prop, prop).as_str();
    }

    // End constructor method
    c += "  }\n\n";

    // Getters and setters
    for prop in &config.properties {
        let formatted_prop = capitalize(&prop);
        // Getter
        c += format!("  get {}() {{\n", prop).as_str();
        c += format!("    return this._{};\n", prop).as_str();

        // End getter
        c += "  }\n\n";

        // Setter
        c += format!("  set {}(new{}) {{\n", prop, &formatted_prop).as_str();
        c += format!("    this._{} = new{};\n", prop, formatted_prop).as_str();
        // End setter
        c += "  }\n\n";
    }

    c.pop();

    // End class declaration
    c += "}";

    files.push(FileResponse::new(c, formatted_name + ".js"));

    files
}

fn generate_react_component(config: JS) -> Vec<FileResponse> {
    println!("Generating react-component...");
    println!("{:?}", config);
    let files = vec![];
    files
}

fn capitalize(s: &str) -> String {
    let mut new_s = String::new();
    let mut cs = s.chars();
    let first = cs.next().expect("Should have at least one character");
    new_s.push(first.to_ascii_uppercase());
    while let Some(c) = cs.next() {
        new_s.push(c);
    }
    new_s
}
