use darling::{Error, FromField};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident, Index, Member, Variant, parse_macro_input};

fn field_to_member(index: usize, field: &Field) -> Member {
  match &field.ident {
    Some(ident) => Member::Named(ident.clone()),
    None => Member::Unnamed(Index::from(index)),
  }
}

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
  let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
  let type_ident = input.ident;

  let body = match input.data {
    Data::Struct(data) => derive_struct(&type_ident, data.fields),
    Data::Enum(data) => derive_enum(data.variants.into_iter().collect()),
    _ => {
      return Error::custom("TreeDisplay cannot be derived for unions")
        .write_errors()
        .into();
    }
  };

  quote! {
    impl #impl_generics crate::tree_display::TreeDisplay for #type_ident #type_generics #where_clause {
      fn tree(&self) -> crate::tree_display::TreeNode {
        #body
      }
    }
  }
  .into()
}

fn derive_struct(type_ident: &Ident, fields: Fields) -> TokenStream2 {
  let is_empty_type = matches!(&fields, Fields::Unit)
    || matches!(&fields, Fields::Unnamed(unnamed) if unnamed.unnamed.is_empty())
    || matches!(&fields, Fields::Named(named) if named.named.is_empty());

  let type_name = type_ident.to_string();

  if is_empty_type {
    return quote! {
      crate::tree_display::TreeNode::leaf(#type_name)
    };
  }

  let fields = match fields {
    Fields::Unit => unreachable!(),
    Fields::Named(named_fields) => named_fields.named,
    Fields::Unnamed(unnamed_fields) => unnamed_fields.unnamed,
  };

  let field_handlers = match fields
    .iter()
    .enumerate()
    .map(|(idx, field)| {
      let member = field_to_member(idx, field);
      let attrs = FieldAttributes::from_field(field)?;
      Ok(process_field(quote! {self.#member}, member, attrs))
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

    crate::tree_display::TreeNode::new(#type_name, fields, children)
  }
}

fn derive_enum(variants: Vec<Variant>) -> TokenStream2 {
  let mut arms = Vec::new();
  for variant in variants {
    let variant_ident = variant.ident;
    let variant_name = variant_ident.to_string();

    match variant.fields {
      Fields::Unit => {
        arms.push(quote! {
          Self::#variant_ident => crate::tree_display::TreeNode::leaf(#variant_name)
        });
      }

      Fields::Named(fields) => {
        let bindings = fields
          .named
          .iter()
          .map(|f| f.ident.clone().unwrap())
          .collect::<Vec<_>>();

        let handlers = match fields
          .named
          .iter()
          .enumerate()
          .map(|(idx, field)| {
            let attrs = FieldAttributes::from_field(field)?;
            let ident = &bindings[idx];
            Ok(process_field(
              quote! {#ident},
              Member::Named(ident.clone()),
              attrs,
            ))
          })
          .collect::<darling::Result<Vec<_>>>()
        {
          Ok(x) => x,
          Err(err) => return err.write_errors(),
        };

        arms.push(quote! {
          Self::#variant_ident{ #( #bindings ),* } => {
            let mut fields = ::std::vec::Vec::<crate::tree_display::Field>::new();
            let mut children = ::std::vec::Vec::<crate::tree_display::TreeNode>::new();

            #(#handlers)*

            crate::tree_display::TreeNode::new(#variant_name, fields, children)
          }
        });
      }

      Fields::Unnamed(fields) => {
        let bindings = (0..fields.unnamed.len())
          .map(|i| Ident::new(&format!("__field{i}"), Span::call_site()))
          .collect::<Vec<_>>();

        let handlers = match fields
          .unnamed
          .iter()
          .enumerate()
          .map(|(idx, field)| {
            let attrs = FieldAttributes::from_field(field)?;
            let ident = &bindings[idx];
            Ok(process_field(
              quote! {#ident},
              Member::Unnamed(Index::from(idx)),
              attrs,
            ))
          })
          .collect::<darling::Result<Vec<_>>>()
        {
          Ok(x) => x,
          Err(err) => return err.write_errors(),
        };

        arms.push(quote! {
          Self::#variant_ident( #( #bindings ),* ) => {
            let mut fields = ::std::vec::Vec::<crate::tree_display::Field>::new();
            let mut children = ::std::vec::Vec::<crate::tree_display::TreeNode>::new();

            #(#handlers)*

            crate::tree_display::TreeNode::new(#variant_name, fields, children)
          }
        });
      }
    }
  }

  quote! {
    match self {
      #(#arms),*
    }
  }
}

fn process_field(
  access: TokenStream2,
  member: Member,
  attributes: FieldAttributes,
) -> TokenStream2 {
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
      let node = #access.tree();
      children.push(crate::tree_display::TreeNode {
        label: ::std::format!("{}{}", #member_string, node.label),
        fields: node.fields.into_iter().map(|f| crate::tree_display::Field {
          name: ::std::format!("{}: ", f.name),
          value: f.value,
        }).collect(),
        children: node.children,
      });
    }
  } else {
    quote! {
      fields.push(crate::tree_display::Field {
        name: ::std::string::String::from(#member_string),
        value: ::std::format!("{:?}", #access),
      });
    }
  }
}
