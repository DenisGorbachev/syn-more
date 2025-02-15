use syn::Path;

pub fn default_path() -> Path {
    Path {
        leading_colon: None,
        segments: Default::default(),
    }
}
