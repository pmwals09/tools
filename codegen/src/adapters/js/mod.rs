mod class;
mod shared;
mod react;

pub use crate::adapters::js::shared::*;

use crate::adapters::shared::FileResponse;
use crate::adapters::js::class::generate_class;
use crate::adapters::js::react::generate_react_component;


pub fn main(config: JS) -> Vec<FileResponse> {
    return match config.generator {
        JSGenerator::Class(class_config) => generate_class(class_config),
        JSGenerator::ReactComponent(react_config) => generate_react_component(react_config), 
    };
}
