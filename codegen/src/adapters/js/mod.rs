mod class;
mod shared;

pub use crate::adapters::js::shared::*;

use crate::adapters::shared::FileResponse;
use crate::adapters::js::class::generate_class;


pub fn main(config: JS) -> Vec<FileResponse> {
    return match config.generator {
        JSGenerator::Class(class_config) => generate_class(class_config),
        JSGenerator::ReactComponent(react_config) => generate_react_component(react_config), 
    };
}


fn generate_react_component(config: ReactComponent) -> Vec<FileResponse> {
    println!("Generating react-component...");
    println!("{:?}", config);
    let files = vec![];

    // Generate React component
    // Generate scss file
    // Generate config file
    // Generate service file
    // Generate test file

    files
}

