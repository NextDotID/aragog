use syn::spanned::Spanned;
use syn::{Attribute, Lit, Meta, Path};

pub struct CollectionNameAttribute(pub Lit);

impl CollectionNameAttribute {
    fn correct_path(path: &Path) -> Option<()> {
        let ident = path.get_ident()?;
        if "collection_name" == ident.to_string().as_str() {
            Some(())
        } else {
            None
        }
    }

    pub fn parse_attribute(attr: &Attribute) -> Option<Self> {
        Self::correct_path(&attr.path)?;
        match attr.parse_meta() {
            Ok(meta) => match meta {
                Meta::NameValue(named_value) => {
                    return Some(Self(named_value.lit));
                }
                _ => {
                    emit_error!(
                        meta.span(),
                        "Expected Named Value, add a correct collection name"
                    );
                }
            },
            Err(error) => emit_error!(
                error.span(),
                format!("Failed to parse attribute: {}", error)
            ),
        }
        None
    }
}
