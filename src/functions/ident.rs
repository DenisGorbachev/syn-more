use syn::{Ident, Path, PathSegment, TypePath};

pub fn into_type_path_for_ident(ident: Ident) -> TypePath {
    TypePath {
        qself: None,
        path: Path::from(PathSegment {
            ident,
            arguments: Default::default(),
        }),
    }
}
