use darling::{Error, FromField};
use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident, Index, Member, Variant, parse_macro_input};

fn field_to_member(index: usize, field: &Field) -> Member {
  match &field.ident {
    Some(ident) => Member::Named(ident.clone()),
    None => Member::Unnamed(Index::from(index)),
  }
}

fn string_to_ident(name: impl AsRef<str>) -> Ident {
  Ident::new(name.as_ref(), Span::call_site())
}

#[derive(Debug, Default, FromField)]
#[darling(attributes(tree), default, and_then = Self::validate)]
struct FieldAttributes {
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
    if self.ignore && (self.unlabeled || self.label.is_some()) {
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

  let ccrate = match crate_name("tree-display") {
    Ok(FoundCrate::Itself) => quote!(crate),
    Ok(FoundCrate::Name(name)) => {
      let ident = Ident::new(&name, Span::call_site());
      quote!(::#ident)
    }
    Err(_) => quote!(::tree_display),
  };

  let body = match input.data {
    Data::Struct(data) => derive_struct(&ccrate, &type_ident, data.fields),
    Data::Enum(data) => derive_enum(&ccrate, data.variants.into_iter().collect()),
    _ => {
      return Error::custom("TreeDisplay cannot be derived for unions")
        .write_errors()
        .into();
    }
  };

  quote! {
    impl #impl_generics #ccrate::TreeDisplay for #type_ident #type_generics #where_clause {
      fn tree(&self) -> #ccrate::Tree {
        use #ccrate::format::{Member, TypeName};
        #body
      }
    }
  }
  .into()
}

fn derive_struct(ccrate: &TokenStream2, type_ident: &Ident, fields: Fields) -> TokenStream2 {
  let type_name = type_ident.to_string();

  let is_empty_type = matches!(&fields, Fields::Unit)
    || matches!(&fields, Fields::Unnamed(unnamed) if unnamed.unnamed.is_empty())
    || matches!(&fields, Fields::Named(named) if named.named.is_empty());

  if is_empty_type {
    return quote!(#ccrate::Tree::leaf(TypeName::new(#type_name)));
  }

  let is_newtype = matches!(fields, Fields::Unnamed(_)) && fields.len() == 1;

  if is_newtype {
    let field = fields.iter().next().expect("there is exactly one element");
    let member = field_to_member(0, field);
    return quote!(self.#member.tree());
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
      Ok(process_field(quote!(self.#member), member, attrs))
    })
    .collect::<darling::Result<Vec<_>>>()
  {
    Ok(handlers) => handlers,
    Err(err) => return err.write_errors().into(),
  };

  quote! {
    let mut subtrees = ::std::vec::Vec::<#ccrate::Tree>::new();
    #(#field_handlers)*
    #ccrate::Tree::new(TypeName::new(#type_name), subtrees)
  }
}

fn derive_enum(ccrate: &TokenStream2, variants: Vec<Variant>) -> TokenStream2 {
  let mut arms = Vec::new();
  for variant in variants {
    let variant_ident = variant.ident;
    let variant_name = variant_ident.to_string();

    match variant.fields {
      Fields::Unit => {
        arms.push(quote! {
          Self::#variant_ident => #ccrate::Tree::leaf(TypeName::new(#variant_name))
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
            let ident = if attrs.ignore {
              &string_to_ident("_")
            } else {
              &bindings[idx]
            };
            Ok(process_field(
              quote!(#ident),
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
            let mut subtrees = ::std::vec::Vec::<#ccrate::Tree>::new();

            #(#handlers)*

            #ccrate::Tree::new(TypeName::new(#variant_name), subtrees)
          }
        });
      }

      Fields::Unnamed(fields) => {
        let is_newtype = fields.unnamed.len() == 1;
        if is_newtype {
          // For newtype variants, forward directly without wrapping
          let ident = string_to_ident("__field0");
          arms.push(quote! {
            Self::#variant_ident( #ident ) => {
              #ident.tree()
            }
          });
          continue;
        }

        let bindings = (0..fields.unnamed.len())
          .map(|i| string_to_ident(format!("__field{i}")))
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
            let mut subtrees = ::std::vec::Vec::<#ccrate::Tree>::new();

            #(#handlers)*

            #ccrate::Tree::new(TypeName::new(#variant_name), subtrees)
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
    return quote!();
  }

  let label = match (attributes.unlabeled, attributes.label, &member) {
    (true, _, _) => None,
    (false, Some(label), _) => Some(label),
    (false, _, Member::Named(ident)) => Some(ident.to_string()),
    (false, _, Member::Unnamed(index)) => Some(format!(".{}", index.index.to_string())),
  };

  let labeled = match label {
    Some(l) => quote! { .labeled(Member::new(#l)) },
    None => quote! {},
  };
  quote! {
    subtrees.push(#access.tree()#labeled);
  }
}
