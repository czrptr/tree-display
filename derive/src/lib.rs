use darling::{Error, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, Member, Variant, parse_macro_input};

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

  let ident = input.ident;
  let ident_string = ident.to_string();
  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  let body = match input.data {
    Data::Struct(data) => derive_struct(&ident_string, data.fields),
    Data::Enum(data) => derive_enum(&ident_string, data.variants.into_iter().collect()),
    Data::Union(_) => {
      return Error::custom("TreeDisplay cannot be derived for unions")
        .write_errors()
        .into();
    }
  };

  quote! {
    impl #impl_generics crate::tree_display::TreeDisplay for #ident #ty_generics #where_clause {
      fn tree(&self) -> crate::tree_display::TreeNode {
        #body
      }
    }
  }
  .into()
}

fn derive_struct(type_name_string: &str, fields: Fields) -> proc_macro2::TokenStream {
  let is_empty_type = matches!(&fields, Fields::Unit)
    || matches!(&fields, Fields::Unnamed(unnamed) if unnamed.unnamed.is_empty());

  if is_empty_type {
    return quote! {
      crate::tree_display::TreeNode {
        label: ::std::string::String::from(#type_name_string),
        fields: ::std::vec::Vec::new(),
        children: ::std::vec::Vec::new(),
      }
    };
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
    let mut fields = ::std::vec::Vec::<crate::tree_display::Field>::new();
    let mut children = ::std::vec::Vec::<crate::tree_display::TreeNode>::new();

    #(#field_handlers)*

    crate::tree_display::TreeNode {
      label: ::std::string::String::from(#type_name_string),
      fields,
      children,
    }
  }
}

fn derive_enum(_type_name_string: &str, _variants: Vec<Variant>) -> proc_macro2::TokenStream {
  quote! {}
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
      children.push(crate::tree_display::TreeNode {
        label: ::std::format!("{}{}", #member_string, node.label),
        fields: node.fields,
        children: node.children,
      });
    }
  } else {
    quote! {
      fields.push(crate::tree_display::Field {
        name: ::std::string::String::from(#member_string),
        value: ::std::format!("{:?}", self.#member),
      });
    }
  }
}
