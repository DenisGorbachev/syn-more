use syn::{ItemUse, UseTree, Visibility};

pub fn new_item_use(tree: UseTree) -> ItemUse {
    ItemUse {
        attrs: vec![],
        vis: Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree,
        semi_token: Default::default(),
    }
}
