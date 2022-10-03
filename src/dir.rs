#[derive(Debug)]
pub struct Dir {
    pub name: String,
    pub is_folder: bool,
    pub is_open: bool,
}

impl Dir {
    pub fn new(name: String, is_folder: bool, is_open: bool) -> Self {
        Dir {
            name,
            is_folder,
            is_open,
        }
    }
}
