use crate::SynFrom;
use derive_more::{Error, From};
use fmt_derive::Display;
use std::fs::read_to_string;
use std::path::Path;
use syn::{parse_file, File, Item};

impl SynFrom<&Path> for File {
    type Output = Result<Self, FromPathFileError>;

    fn syn_from(path: &Path) -> Self::Output {
        let content = read_to_string(path)?;
        let file = parse_file(&content)?;
        Ok(file)
    }
}

#[derive(Error, From, Display, Debug)]
pub enum FromPathFileError {
    TheSynError(syn::Error),
    TheStdIoError(std::io::Error),
}

impl SynFrom<Item> for File {
    type Output = Self;

    fn syn_from(item: Item) -> Self::Output {
        Self {
            shebang: None,
            attrs: vec![],
            items: vec![item],
        }
    }
}
