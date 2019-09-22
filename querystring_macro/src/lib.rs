extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(query_string_builder)]
pub fn struct_to_querystring(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);

    let expanded = quote! {
        impl BaseQueryOptions for #name {
            fn get_querystring(self) -> String {
                let query_options = vec![#((stringify!(#field_name), self.#field_name.map(|f| f.to_string()))),*];
                build_querystring(query_options) 
            }
        }
    };

    expanded.into()
}