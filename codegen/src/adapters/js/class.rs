use crate::{capitalize, Class};
use crate::adapters::shared::FileResponse;

pub fn generate_class(config: Class) -> Vec<FileResponse> {
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

