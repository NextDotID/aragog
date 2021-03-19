use crate::parse_operation::{OperationValue, ParseOperation};
use proc_macro2::Span;
use syn::{spanned::Spanned, Attribute, Field, Meta, NestedMeta, Path};

pub trait ParseAttribute: Sized {
    type AttributeOperation: ParseOperation;

    fn init(path: &Path, field: Option<&Field>) -> Option<Self>;

    fn field(&self) -> Option<String>;

    fn add_operation(&mut self, span: Span, operation: Self::AttributeOperation);

    fn validate(&self, span: Span) -> bool;

    fn parse_operation(&self, meta: &Meta) -> Option<Self::AttributeOperation> {
        let operation = match meta {
            Meta::NameValue(named_value) => Self::AttributeOperation::parse(
                &named_value.path,
                Some(OperationValue::Lit(named_value.lit.clone())),
                self.field(),
            )?,
            Meta::List(list) => {
                if list.nested.len() > 1 {
                    emit_error!(
                        list.span(),
                        "Wrong format, only one value per operation allowed"
                    );
                    return None;
                }
                let nested_meta = match list.nested.first() {
                    Some(m) => m,
                    None => {
                        emit_error!(list.span(), "Wrong format, empty parenthesis");
                        return None;
                    }
                };
                let value = OperationValue::parse(nested_meta)?;
                Self::AttributeOperation::parse(&list.path, Some(value), self.field())?
            }
            Meta::Path(path) => Self::AttributeOperation::parse(path, None, self.field())?,
        };
        Some(operation)
    }

    fn parse_attribute(attr: &Attribute, field: Option<&Field>, container: &mut Vec<Self>) {
        let mut cmd = match Self::init(&attr.path, field) {
            None => return,
            Some(c) => c,
        };
        match attr.parse_meta() {
            Ok(meta) => match meta {
                Meta::List(list) => {
                    for nest in list.nested.iter() {
                        match nest {
                            NestedMeta::Meta(meta) => {
                                if let Some(op) = cmd.parse_operation(meta) {
                                    cmd.add_operation(meta.span(), op)
                                }
                            }
                            NestedMeta::Lit(_) => {
                                emit_error!(nest.span(), "Expected meta item, not a Rust Literal")
                            }
                        }
                    }
                    if cmd.validate(list.span()) {
                        container.push(cmd);
                    }
                }
                _ => {
                    emit_error!(meta.span(), "Expected a meta list. Add valid operations")
                }
            },
            Err(error) => emit_error!(
                error.span(),
                format!("Failed to parse attribute: {}", error)
            ),
        }
    }
}
