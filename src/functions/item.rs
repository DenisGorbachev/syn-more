use std::path::Path;

use crate::{FromPathFileError, SynFrom};
use syn::{Attribute, File, Ident, Item};

pub fn is_main_item(item: &Item) -> bool {
    matches!(item, Item::Enum(_) | Item::Impl(_) | Item::Struct(_) | Item::Trait(_) | Item::TraitAlias(_) | Item::Type(_) | Item::Union(_))
}

pub fn parse_main_item_from_path<P: AsRef<Path>>(path: P) -> Result<Option<Item>, FromPathFileError> {
    let mut file = File::syn_from(path.as_ref())?;
    let main_item_position = file.items.iter().position(is_main_item);
    let item = main_item_position.map(|position| file.items.remove(position));
    Ok(item)
}

pub fn get_main_item_mut(file: &mut File) -> Option<&mut Item> {
    let main_item_position = file.items.iter().position(is_main_item);
    main_item_position.and_then(|position| file.items.get_mut(position))
}

pub fn get_attrs_mut(item: &mut Item) -> Option<&mut Vec<Attribute>> {
    match item {
        Item::Struct(item) => Some(&mut item.attrs),
        Item::Enum(item) => Some(&mut item.attrs),
        Item::Fn(item) => Some(&mut item.attrs),
        Item::Mod(item) => Some(&mut item.attrs),
        Item::Const(item) => Some(&mut item.attrs),
        Item::ExternCrate(item) => Some(&mut item.attrs),
        Item::ForeignMod(item) => Some(&mut item.attrs),
        Item::Impl(item) => Some(&mut item.attrs),
        Item::Macro(item) => Some(&mut item.attrs),
        Item::Static(item) => Some(&mut item.attrs),
        Item::Trait(item) => Some(&mut item.attrs),
        Item::TraitAlias(item) => Some(&mut item.attrs),
        Item::Type(item) => Some(&mut item.attrs),
        Item::Union(item) => Some(&mut item.attrs),
        Item::Use(item) => Some(&mut item.attrs),
        _ => None,
    }
}

pub fn get_struct_or_enum_attrs_mut(item: &mut Item) -> Option<&mut Vec<Attribute>> {
    match item {
        Item::Struct(item) => Some(&mut item.attrs),
        Item::Enum(item) => Some(&mut item.attrs),
        _ => None,
    }
}

pub fn maybe_ref_ident_for_ref_item(item: &Item) -> Option<&Ident> {
    match item {
        Item::Const(item) => Some(&item.ident),
        Item::Enum(item) => Some(&item.ident),
        Item::ExternCrate(item) => Some(&item.ident),
        Item::Fn(item) => Some(&item.sig.ident),
        Item::ForeignMod(_item) => None,
        Item::Impl(_item) => None,
        Item::Macro(item) => item.ident.as_ref(),
        Item::Mod(item) => Some(&item.ident),
        Item::Static(item) => Some(&item.ident),
        Item::Struct(item) => Some(&item.ident),
        Item::Trait(item) => Some(&item.ident),
        Item::TraitAlias(item) => Some(&item.ident),
        Item::Type(item) => Some(&item.ident),
        Item::Union(item) => Some(&item.ident),
        Item::Use(_item) => None,
        Item::Verbatim(_item) => None,
        _ => todo!(),
    }
}

pub fn maybe_ident_for_item(item: Item) -> Option<Ident> {
    match item {
        Item::Const(item) => Some(item.ident),
        Item::Enum(item) => Some(item.ident),
        Item::ExternCrate(item) => Some(item.ident),
        Item::Fn(item) => Some(item.sig.ident),
        Item::ForeignMod(_item) => None,
        Item::Impl(_item) => None,
        Item::Macro(item) => item.ident,
        Item::Mod(item) => Some(item.ident),
        Item::Static(item) => Some(item.ident),
        Item::Struct(item) => Some(item.ident),
        Item::Trait(item) => Some(item.ident),
        Item::TraitAlias(item) => Some(item.ident),
        Item::Type(item) => Some(item.ident),
        Item::Union(item) => Some(item.ident),
        Item::Use(_item) => None,
        Item::Verbatim(_item) => None,
        _ => todo!(),
    }
}
