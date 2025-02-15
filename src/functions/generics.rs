use syn::punctuated::Punctuated;
use syn::{GenericParam, Generics};

pub fn get_generics_from_params(params: impl IntoIterator<Item = GenericParam>) -> Generics {
    let params = Punctuated::from_iter(params);
    if params.first().is_some() {
        // with lt_token / gt_token
        Generics {
            lt_token: Some(Default::default()),
            params,
            gt_token: Some(Default::default()),
            where_clause: None,
        }
    } else {
        // without lt_token / gt_token
        Generics {
            lt_token: None,
            params,
            gt_token: None,
            where_clause: None,
        }
    }
}
