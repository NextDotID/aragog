use proc_macro2::Span;
use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Field, Ident, Path};

use crate::derives::validate::operation::Operation;
use crate::parse_attribute::ParseAttribute;
use crate::to_tokenstream::ToTokenStream;
use crate::toolbox::expect_field_name;

#[derive(Clone)]
pub enum ValidateCommandType {
    Validate,
    ValidateField { field: String },
    ValidateFieldEach { field: String },
}

#[derive(Clone)]
pub struct ValidateCommand {
    operations: Vec<Operation>,
    command_type: ValidateCommandType,
}

impl ParseAttribute for ValidateCommand {
    type AttributeOperation = Operation;

    fn init(path: &Path, field: Option<&Field>) -> Option<Self> {
        let ident = path.get_ident()?;
        let command_type = match ident.to_string().as_str() {
            "validate" => match field {
                Some(f) => ValidateCommandType::ValidateField {
                    field: f.ident.as_ref().unwrap().to_string(),
                },
                None => ValidateCommandType::Validate,
            },
            "validate_each" => ValidateCommandType::ValidateFieldEach {
                field: expect_field_name(path.span(), field)?,
            },
            _ => return None,
        };
        Some(Self {
            command_type,
            operations: vec![],
        })
    }

    fn field(&self) -> Option<String> {
        match &self.command_type {
            ValidateCommandType::Validate => None,
            ValidateCommandType::ValidateField { field } => Some(field.clone()),
            ValidateCommandType::ValidateFieldEach { field } => Some(field.clone()),
        }
    }

    fn add_operation(&mut self, _span: Span, operation: Self::AttributeOperation) {
        self.operations.push(operation);
    }

    fn validate(&self, span: Span) -> bool {
        if self.operations.is_empty() {
            emit_error!(span, "Validation attribute requires at least one operation");
            false
        } else {
            true
        }
    }
}

impl ValidateCommand {
    fn field_ident(field: &String) -> Ident {
        Ident::new(field.as_str(), Span::call_site())
    }

    fn field_each_token() -> TokenStream {
        let ident = Ident::new("iterator", Span::call_site());
        let res = quote! {
            #ident
        };
        res.into()
    }
}

impl ToTokenStream for ValidateCommand {
    fn token_stream(self) -> TokenStream {
        let mut quote = quote! {};

        let custom_token = match &self.command_type {
            ValidateCommandType::ValidateFieldEach { .. } => Some(Self::field_each_token()),
            _ => None,
        };
        for operation in self.operations.into_iter() {
            let operation_quote = operation.token_stream(custom_token.clone());
            quote = quote! {
               #quote
               #operation_quote
            }
        }
        if let ValidateCommandType::ValidateFieldEach { field } = self.command_type {
            let field_ident = Self::field_ident(&field);
            quote = quote! {
               for iterator in self.#field_ident.iter() {
                    #quote
               }
            };
        }
        quote.into()
    }
}
