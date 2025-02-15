use syn::{ItemImpl, Type};

pub fn default_item_impl(self_ty: Box<Type>) -> ItemImpl {
    ItemImpl {
        attrs: vec![],
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: Default::default(),
        trait_: None,
        self_ty,
        brace_token: Default::default(),
        items: vec![],
    }
}
