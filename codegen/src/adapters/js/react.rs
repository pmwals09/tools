use crate::adapters::shared::FileResponse;
use crate::capitalize;
use crate::{ReactComponent, ReactSupport};

pub fn generate_react_component(config: ReactComponent) -> Vec<FileResponse> {
    println!("Generating react-component...");
    let mut files = vec![];

    let component_name = capitalize(&config.name);

    // Generate React component
    let mut c = String::new();
    c += "import React from \"react\";\n";

    if let Some(support_list) = config.file_include {
        for item in support_list {
            match item {
                ReactSupport::Style => {
                    let file_name = component_name.clone() + ".scss";
                    c += format!("import \"./{}\";\n", file_name).as_str();
                    files.push(FileResponse::new(String::new(), file_name));
                },
                ReactSupport::Config => {
                    let file_name = component_name.clone() + ".config.js";
                    c += format!("import * as config from \"./{}\";\n", file_name).as_str();
                    files.push(FileResponse::new(String::new(), file_name));
                },
                ReactSupport::Service => {
                    let file_name = component_name.clone() + ".service.js";
                    c += format!("import * as service from \"./{}\";\n", file_name).as_str();
                    files.push(FileResponse::new(String::new(), file_name));
                },
                ReactSupport::Test => files.push(FileResponse::new(String::new(), component_name.clone() + ".test.js"))
            }
        }
    }
    c += "\n";
    c += format!("function {}(props) {{\n", &component_name).as_str();
    c += "  return (\n";
    c += "    <div>\n";
    c += "      <pre>\n";
    c += "        <code>\n";
    c += format!("          I am the {} component\n", &component_name).as_str();
    c += "        </code>\n";
    c += "      </pre>\n";
    c += "    </div>\n";
    c += "  );\n";
    c += "}\n\n";
    c += format!("export default {};", &component_name).as_str();
    files.push(FileResponse::new(c, component_name + ".jsx"));

    files
}

