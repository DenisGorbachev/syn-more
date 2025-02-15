use syn::{parse_str, Attribute, ItemStruct};

/// `Attribute` does not implement `Parse`, so we have to work around this
pub fn parse_attribute(attr: &str) -> Option<Attribute> {
    let dummy_type = format!("{attr} struct Dummy;");
    let dummy_item: ItemStruct = parse_str(&dummy_type).expect("Parsing an attribute must always succeed");
    dummy_item.attrs.into_iter().next()
}
