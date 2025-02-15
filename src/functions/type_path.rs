use syn::{Path, TypePath};

pub fn default_type_path(path: Path) -> TypePath {
    TypePath {
        qself: None,
        path,
    }
}
