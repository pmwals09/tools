#[derive(Debug)]
pub struct FileResponse {
    pub contents: String,
    pub name: String
}

impl FileResponse {
    pub fn new(contents: String, name: String) -> Self {
        Self {
            contents,
            name
        }
    }
}
