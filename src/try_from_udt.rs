use common::get_struct_fields;
use quote;
use syn;

pub fn impl_try_from_udt(ast: &syn::DeriveInput) -> quote::Tokens {
  let name = &ast.ident;
  let fields = get_struct_fields(ast);
  quote! {
      impl cdrs_tokio::frame::TryFromUdt for #name {
        fn try_from_udt(cdrs: cdrs_tokio::types::udt::Udt) -> cdrs_tokio::Result<Self> {
        use cdrs_tokio::frame::TryFromUdt;
        use cdrs_tokio::types::from_cdrs::FromCdrsByName;

          Ok(#name {
            #(#fields),*
          })
        }
      }
  }
}
