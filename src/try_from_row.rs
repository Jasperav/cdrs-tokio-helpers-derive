use common::{struct_fields, tokenize_fields, get_ident, has_attr};
use quote;
use syn;

pub fn impl_try_from_row(ast: &syn::DeriveInput) -> quote::Tokens {
  let name = &ast.ident;
  let fields = struct_fields(ast).clone();
  let (mapped, non_mapped): (Vec<syn::Field>, Vec<syn::Field>) = fields
      .into_iter()
      .partition(|r| has_attr(r, "json_mapped"));
  let mut fields = tokenize_fields(&non_mapped);

  for mapped in mapped {
    let name = mapped.ident.unwrap();
    let ty = get_ident(mapped.ty);

    let string = quote! {
          let val = &String::from_cdrs_r(&cdrs, stringify!(#name))?;
      };

    let serde = quote! {
          serde_json::from_str(&val).map_err(|e| cdrs_tokio::Error::General(format!("Failed to transform type {}", stringify!(#name))))?
      };

    let ts = if &ty.to_string() == "Option" {
      quote! {
          #name: {
              #string
              if val.is_empty() {
                  None
              } else {
                  Some(#serde)
              }
          }
      }
    } else {
      quote! {
          #name: {
              #string
              #serde
          }
      }
    };

    fields.push(ts);
  }

  quote! {
      impl cdrs_tokio::types::prelude::TryFromRow for #name {
        fn try_from_row(cdrs: cdrs_tokio::types::rows::Row) -> cdrs_tokio::Result<Self> {
            use cdrs_tokio::frame::TryFromUdt;
            use cdrs_tokio::types::from_cdrs::FromCdrsByName;
            use cdrs_tokio::types::AsRustType;

          Ok(#name {
            #(#fields),*
          })
        }
      }
  }
}
