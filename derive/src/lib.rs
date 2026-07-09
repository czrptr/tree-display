//! Procedural macro for deriving `TreeDisplay`.
//!
//! This crate provides the `#[derive(TreeDisplay)]` macro, which generates
//! tree representations of structs and enums. It should not be used directly;
//! use the `tree-display` crate instead.
//!
//! # Field Attributes
//!
//! The following attributes can be applied to struct or enum variant fields:
//!
//! - `#[tree(map)]` - Apply a custom mapper from `Context`
//! - `#[tree(ignore)]` - Exclude the field from the tree
//! - `#[tree(label = "...")]` - Override the field's display label
//! - `#[tree(unlabeled)]` - Display the field without a label

use darling::{Error, FromField};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident, Index, Member, Variant};

// ──── API ───────────────────────────────────────────────────────────────────────────────────────

/// Derives `TreeDisplay` for structs and enums.
///
/// The following attributes can be applied to struct or enum variant fields:
///
/// - `#[tree(map)]` - Apply a custom mapper from `Context`
/// - `#[tree(ignore)]` - Exclude the field from the tree
/// - `#[tree(label = "...")]` - Override the field's display label
/// - `#[tree(unlabeled)]` - Display the field without a label
#[proc_macro_derive(TreeDisplay, attributes(tree))]
pub fn derive_tree_display(tokens: TokenStream) -> TokenStream {
  derive(tokens)
}

// ──── Impl ──────────────────────────────────────────────────────────────────────────────────────

/// Converts a field index to a [`Member`] (named or unnamed).
fn field_to_member(index: usize, field: &Field) -> Member {
  match &field.ident {
    Some(ident) => Member::Named(ident.clone()),
    None => Member::Unnamed(Index::from(index)),
  }
}

/// Creates an [`Ident`] from a string.
fn string_to_ident(name: impl AsRef<str>) -> Ident {
  Ident::new(name.as_ref(), Span::call_site())
}

/// Attributes that can be applied to fields with `#[tree(...)]`.
#[derive(Debug, Default, FromField)]
#[darling(attributes(tree), default, and_then = Self::validate)]
struct FieldAttributes {
  /// Apply a custom mapper from [`Context`]
  map: bool,
  /// Exclude this field from the tree
  ignore: bool,
  /// Display the field without a label
  unlabeled: bool,
  /// Override the field's display label
  label: Option<String>,
}

impl FieldAttributes {
  /// Validates that attributes are not used in contradictory ways.
  fn validate(self) -> darling::Result<Self> {
    if self.unlabeled && self.label.is_some() {
      return Err(darling::Error::custom(
        "`unlabeled` and `label` cannot be used together",
      ));
    }
    if self.ignore && (self.map || self.unlabeled || self.label.is_some()) {
      return Err(darling::Error::custom(
        "`ignore` cannot be combined with any other attribute",
      ));
    }
    Ok(self)
  }
}

/// Entry point for the derive macro.
fn derive(tokens: TokenStream) -> TokenStream {
  let input = parse_macro_input!(tokens as DeriveInput);
  let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
  let type_ident = input.ident;

  // `proc-macro-crate` looks at CARGO_MANIFEST_DIR to find the calling
  // crate's Cargo.toml. During a doctest, that still points at
  // tree-display's own manifest (whose package name is "tree-display"),
  // so `crate_name` incorrectly reports `FoundCrate::Itself` even though
  // the doctest is compiled as a separate binary that depends on
  // `tree_display` as an *external* crate. Detect that case via the env
  // var rustdoc sets while compiling doctests, and fall back to the
  // external path in that situation.
  let is_doctest = std::env::var_os("UNSTABLE_RUSTDOC_TEST_PATH").is_some();

  let ccrate = match crate_name("tree-display") {
    Ok(FoundCrate::Itself) if !is_doctest => quote!(crate),
    Ok(FoundCrate::Itself) => quote!(::tree_display),
    Ok(FoundCrate::Name(name)) => {
      let ident = Ident::new(&name, Span::call_site());
      quote!(::#ident)
    }
    Err(_) => quote!(::tree_display),
  };

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
    impl #impl_generics #ccrate::TreeDisplay for #type_ident #type_generics #where_clause {
      fn tree(&self, context: &#ccrate::context::Context) -> #ccrate::Tree {
        use #ccrate::{format::{Member, TypeName}, Tree};
        use ::std::any::TypeId;

        #[allow(dead_code)] // might not be used if no fields are mapped
        const fn type_of<T: ?Sized + 'static>(_: &T) -> TypeId {
          TypeId::of::<T>()
        }

        #body
      }
    }
  }
  .into()
}

/// Generates the tree body for a struct.
fn derive_struct(type_ident: &Ident, fields: Fields) -> TokenStream2 {
  let type_name = type_ident.to_string();

  let is_empty_type = matches!(&fields, Fields::Unit)
    || matches!(&fields, Fields::Unnamed(unnamed) if unnamed.unnamed.is_empty())
    || matches!(&fields, Fields::Named(named) if named.named.is_empty());

  if is_empty_type {
    return quote!(Tree::leaf(TypeName::new(#type_name)));
  }

  let is_newtype = matches!(fields, Fields::Unnamed(_)) && fields.len() == 1;
  if is_newtype {
    let field = fields.iter().next().expect("there is exactly one element");
    let member = field_to_member(0, field);
    let attrs = match FieldAttributes::from_field(field) {
      Ok(attrs) => attrs,
      Err(err) => return err.write_errors().into(),
    };

    if attrs.map {
      return quote! {
        if let Some(mapper) = context.mappers.get(&type_of(&self.#member)) {
          Tree::leaf(mapper(&self.#member))
        }
        else
        {
          self.#member.tree(context)
        }
      };
    }
    return quote!(self.#member.tree(context));
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
    let mut subtrees = ::std::vec::Vec::<Tree>::new();
    #(#field_handlers)*
    Tree::new(TypeName::new(#type_name), subtrees)
  }
}

/// Generates the tree body for an enum.
fn derive_enum(variants: Vec<Variant>) -> TokenStream2 {
  let mut arms = Vec::new();
  for variant in variants {
    let variant_ident = variant.ident;
    let variant_name = variant_ident.to_string();

    match variant.fields {
      Fields::Unit => {
        arms.push(quote! {
          Self::#variant_ident => Tree::leaf(TypeName::new(#variant_name))
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
            let mut subtrees = ::std::vec::Vec::<Tree>::new();

            #(#handlers)*

            Tree::new(TypeName::new(#variant_name), subtrees)
          }
        });
      }

      Fields::Unnamed(fields) => {
        let is_newtype = fields.unnamed.len() == 1;
        if is_newtype {
          // For newtype variants, forward directly without wrapping
          let ident = string_to_ident("__field0");
          let field = fields
            .unnamed
            .first()
            .expect("there is exactly one element");

          let attrs = match FieldAttributes::from_field(field) {
            Ok(attrs) => attrs,
            Err(err) => return err.write_errors().into(),
          };

          if attrs.map {
            arms.push(quote! {
              Self::#variant_ident( #ident ) => {
                if let Some(mapper) = context.mappers.get(&type_of(&#ident)) {
                  Tree::leaf(mapper(&#ident))
                }
                else
                {
                  #ident.tree(context)
                }
              }
            });
            continue;
          }

          arms.push(quote! {
            Self::#variant_ident( #ident ) => {
              #ident.tree(context)
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
            let mut subtrees = ::std::vec::Vec::<Tree>::new();

            #(#handlers)*

            Tree::new(TypeName::new(#variant_name), subtrees)
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

/// Processes a single field and generates code to add it to the tree.
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

  if attributes.map {
    return quote! {
      if let Some(mapper) = context.mappers.get(&type_of(&#access)) {
        let mapped = mapper(&#access);
        subtrees.push(Tree::leaf(mapped)#labeled);
      }
      else
      {
        subtrees.push(#access.tree(context)#labeled);
      }
    };
  }

  quote! {
    subtrees.push(#access.tree(context)#labeled);
  }
}
