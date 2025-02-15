use syn::{Ident, TypeParam};

pub fn default_type_param(ident: Ident) -> TypeParam {
    TypeParam {
        attrs: vec![],
        ident,
        colon_token: None,
        bounds: Default::default(),
        eq_token: None,
        default: None,
    }
}
