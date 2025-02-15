use proc_macro2::Ident;
use standard_traits::Of;
use syn::{UsePath, UseTree};

pub fn get_use_tree_with_crate(use_tree: UseTree) -> UseTree {
    UseTree::Path(UsePath {
        ident: Ident::of("crate"),
        colon2_token: Default::default(),
        tree: Box::new(use_tree),
    })
}
