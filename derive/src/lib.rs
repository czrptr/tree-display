use darling::Error;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Index, Meta, parse_macro_input};

#[derive(Debug, Default)]
struct FieldAttributes {
  ignore: bool,
  inline: bool,
}

impl From<&Field> for FieldAttributes {
  fn from(field: &Field) -> Self {
    let mut result = FieldAttributes::default();

    for attribute in &field.attrs {
      if !attribute.path().is_ident("tree") {
        continue;
      }

      let Ok(meta) = attribute.parse_args::<Meta>() else {
        continue;
      };

      match meta {
        Meta::Path(path) => {
          // #[tree(ignore)]
          if path.is_ident("ignore") {
            result.ignore = true;
          }
          // #[tree(inline)]
          else if path.is_ident("inline") {
            result.inline = true;
          }
        }
        _ => {}
      }
    }
    result
  }
}

#[proc_macro_derive(TreeDisplay, attributes(tree))]
pub fn derive_tree_display(tokens: TokenStream) -> TokenStream {
  let input = parse_macro_input!(tokens as DeriveInput);

  let type_name = input.ident;
  let type_name_string = type_name.to_string();
  let (impl_generics, ttype_generics, where_clause) = input.generics.split_for_impl();

  let fields = match input.data {
    Data::Struct(data) => data.fields,
    _ => {
      return Error::custom("TreeDisplay can only be derived for structs")
        .write_errors()
        .into();
    }
  };

  let is_empty_type = matches!(&fields, Fields::Unit)
    || matches!(&fields, Fields::Unnamed(unnamed) if unnamed.unnamed.is_empty());

  if is_empty_type {
    return quote! {
      impl #impl_generics TreeDisplay for #type_name #ttype_generics #where_clause {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{}", #type_name_string)?;
          Ok(())
        }
      }
    }
    .into();
  }

  let field_handlers = match fields {
    Fields::Named(named_fields) => named_fields
      .named
      .iter()
      .enumerate()
      .map(|(idx, field)| process_field(idx, field, FieldAttributes::from(field)))
      .collect::<Vec<_>>(),
    Fields::Unnamed(unnamed_fields) => unnamed_fields
      .unnamed
      .iter()
      .enumerate()
      .map(|(idx, field)| process_field(idx, field, FieldAttributes::from(field)))
      .collect::<Vec<_>>(),
    Fields::Unit => vec![],
  };

  quote! {
    impl #impl_generics TreeDisplay for #type_name #ttype_generics #where_clause {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut inline_parts = Vec::<String>::new();
        let mut child_parts = Vec::<String>::new();

        #(#field_handlers)*

        write!(f, "{}", #type_name_string)?;

        // Write inline fields
        if !inline_parts.is_empty() {
          write!(f, " ─ {}", inline_parts.join(", "))?;
        }

        // Write child fields
        if !child_parts.is_empty() {
          if let Some((last, rest)) = child_parts.split_last() {
            for child in rest {
              write!(f, "\n├─ {}", child)?;
            }
            write!(f, "\n└─ {}", last)?;
          }
        }

        Ok(())
      }
    }
  }
  .into()
}

fn process_field(
  field_index: usize,
  field: &Field,
  attributes: FieldAttributes,
) -> proc_macro2::TokenStream {
  if attributes.ignore {
    return quote! {};
  }

  if let Some(ident) = field.ident.as_ref() {
    let field_name = ident.to_string();

    if attributes.inline {
      quote! {
        inline_parts.push(::std::format!("{}: {:?}", #field_name, self.#ident));
      }
    } else {
      quote! {
        child_parts.push(::std::format!("{}: {:?}", #field_name, self.#ident));
      }
    }
  } else {
    let field_name = format!(".{:?}", field_index);
    let index = Index::from(field_index);

    if attributes.inline {
      quote! {
        inline_parts.push(::std::format!("{}: {:?}", #field_name, self.#index));
      }
    } else {
      quote! {
        child_parts.push(::std::format!("{}: {:?}", #field_name, self.#index));
      }
    }
  }
}
