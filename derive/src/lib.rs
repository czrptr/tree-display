use darling::{Error, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, Member, parse_macro_input};

#[derive(Debug, Default, FromField)]
#[darling(attributes(tree), default, and_then = Self::validate)]
struct FieldAttributes {
  child: bool,
  ignore: bool,
  unlabeled: bool,
  label: Option<String>,
}

impl FieldAttributes {
  fn validate(self) -> darling::Result<Self> {
    if self.unlabeled && self.label.is_some() {
      return Err(darling::Error::custom(
        "`unlabeled` and `label` cannot be used together",
      ));
    }
    if self.ignore && (self.child || self.unlabeled || self.label.is_some()) {
      return Err(darling::Error::custom(
        "`ignore` cannot be combined with any other attribute",
      ));
    }
    Ok(self)
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

  let fields = match fields {
    Fields::Named(named_fields) => named_fields.named,
    Fields::Unnamed(unnamed_fields) => unnamed_fields.unnamed,
    Fields::Unit => unreachable!(),
  };

  let field_handlers = match fields
    .iter()
    .enumerate()
    .map(|(idx, field)| {
      let member = match &field.ident {
        Some(ident) => Member::Named(ident.clone()),
        None => Member::Unnamed(Index::from(idx)),
      };
      let attrs = FieldAttributes::from_field(field)?;
      Ok(process_field(member, attrs))
    })
    .collect::<darling::Result<Vec<_>>>()
  {
    Ok(handlers) => handlers,
    Err(err) => return err.write_errors().into(),
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

  let member_string = match (attributes.unlabeled, attributes.label, &member) {
    (true, _, _) => "".into(),
    (false, Some(label), _) => ::std::format!("{}: ", label),
    (false, _, Member::Named(ident)) => ::std::format!("{}: ", ident.to_string()),
    (false, _, Member::Unnamed(index)) => ::std::format!(".{}: ", index.index),
  };

  if attributes.child {
    quote! {
      let node = self.#member.tree();
      children.push(TreeNode {
        label: ::std::format!("{}{}", #member_string, node.label),
        children: node.children,
      });
    }
  } else {
    quote! {
      properties.push(TreeNode {
        label: ::std::format!("{}{:?}", #member_string, self.#member),
        children: ::std::vec::Vec::<TreeNode>::new(),
      });
    }
  }
}
