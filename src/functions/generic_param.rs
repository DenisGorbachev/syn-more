use crate::default_type_param;
use standard_traits::Get;
use std::iter::{empty, once, Once};
use syn::{GenericArgument, GenericParam, Lifetime, Path, PathArguments, PathSegment, Type, TypePath, TypeReference};

pub fn get_iter_generic_param_from_ref_trait_path(path: &Path) -> impl Iterator<Item = GenericParam> + '_ {
    path.segments
        .iter()
        .flat_map(get_generic_params_from_ref_path_segment)
}

pub fn get_generic_params_from_ref_path_segment(segment: &PathSegment) -> Box<dyn Iterator<Item = GenericParam> + '_> {
    match &segment.arguments {
        PathArguments::None => Box::new(empty::<GenericParam>()),
        PathArguments::AngleBracketed(generic_args) => Box::new(
            generic_args
                .args
                .iter()
                .flat_map(get_generic_param_from_ref_generic_argument),
        ),
        PathArguments::Parenthesized(_) => {
            todo!()
        }
    }
}

pub fn get_generic_param_from_ref_generic_argument(arg: &GenericArgument) -> Box<dyn Iterator<Item = GenericParam>> {
    match arg {
        GenericArgument::Lifetime(lifetime) => Box::new(get_iter_generic_param_from_ref_lifetime(lifetime)),
        GenericArgument::Type(ty) => get_generic_param_from_ref_type(ty),
        _ => Box::new(empty()),
    }
}

pub fn get_generic_param_from_ref_type(ty: &Type) -> Box<dyn Iterator<Item = GenericParam>> {
    match ty {
        Type::Path(type_path) => Box::new(get_generic_param_from_ref_type_path(type_path).into_iter()),
        Type::Reference(type_reference) => get_generic_param_from_ref_type_reference(type_reference),
        _ => Box::new(empty()),
    }
}

pub fn get_generic_param_from_ref_type_reference(type_reference: &TypeReference) -> Box<dyn Iterator<Item = GenericParam>> {
    let elem_iter = get_generic_param_from_ref_type(type_reference.elem.as_ref());
    match &type_reference.lifetime {
        None => Box::new(elem_iter),
        Some(lifetime) => Box::new(get_iter_generic_param_from_ref_lifetime(lifetime).chain(elem_iter)),
    }
}

fn get_iter_generic_param_from_ref_lifetime(lifetime: &Lifetime) -> Once<GenericParam> {
    once(lifetime.clone().get::<GenericParam>())
}

pub fn get_generic_param_from_ref_type_path(type_path: &TypePath) -> Option<GenericParam> {
    let ident = type_path.path.get_ident()?;
    if is_generic_ident_str(&ident.to_string()) {
        Some(GenericParam::Type(default_type_param(ident.clone())))
    } else {
        None
    }
}

pub fn is_generic_ident_str(str: &str) -> bool {
    str.len() == 1 && str.chars().all(|c| c.is_uppercase())
}
