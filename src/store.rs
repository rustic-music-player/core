use library::Library;
use error::RusticError;

pub trait LibraryStore {
    fn store(&mut self, library: &Library) -> Result<(), RusticError>;
    fn load(&self) -> Result<Library, RusticError>;
}