use darling::Error;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Index, Member, Meta, parse_macro_input};

#[derive(Debug, Default)]
struct FieldAttributes {
  child: bool,
  ignore: bool,
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
          if path.is_ident("child") {
            result.child = true;
          } else if path.is_ident("ignore") {
            result.ignore = true;
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
        fn tree(&self) -> TreeNode {
          TreeNode {
            label: ::std::string::String::from(#type_name_string),
            children: ::std::vec::Vec::new(),
          }
        }
      }
    }
    .into();
  }

  let field_handlers = match fields {
    Fields::Named(named_fields) => named_fields
      .named
      .iter()
      .map(|field| {
        process_field(
          Member::Named(
            field
              .clone()
              .ident
              .expect("named fields are have an identifier"),
          ),
          FieldAttributes::from(field),
        )
      })
      .collect::<Vec<_>>(),
    Fields::Unnamed(unnamed_fields) => unnamed_fields
      .unnamed
      .iter()
      .enumerate()
      .map(|(idx, field)| {
        process_field(
          Member::Unnamed(Index::from(idx)),
          FieldAttributes::from(field),
        )
      })
      .collect::<Vec<_>>(),
    Fields::Unit => vec![],
  };

  quote! {
    impl #impl_generics TreeDisplay for #type_name #ttype_generics #where_clause {
      fn tree(&self) -> TreeNode {
        let mut properties = ::std::vec::Vec::<TreeNode>::new();
        let mut children = ::std::vec::Vec::<TreeNode>::new();

        #(#field_handlers)*

        TreeNode {
          label: ::std::string::String::from(#type_name_string),
          children: properties.into_iter().chain(children.into_iter()).collect(),
        }
      }
    }
  }
  .into()
}

fn process_field(member: Member, attributes: FieldAttributes) -> proc_macro2::TokenStream {
  if attributes.ignore {
    return quote! {};
  }

  let member_string = match &member {
    Member::Named(ident) => ident.to_string(),
    Member::Unnamed(index) => ::std::format!(".{}", index.index),
  };

  if attributes.child {
    quote! {
      let node = self.#member.tree();
      children.push(TreeNode {
        label: ::std::format!("{}: {}", #member_string, node.label),
        children: node.children,
      });
    }
  } else {
    quote! {
      properties.push(TreeNode {
        label: ::std::format!("{}: {:?}", #member_string, self.#member),
        children: ::std::vec::Vec::<TreeNode>::new(),
      });
    }
  }
}
