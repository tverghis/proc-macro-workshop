use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let builder_name = Ident::new(
        format!("{}Builder", name).as_str(),
        Span::call_site().into(),
    );

    let members = match &input.data {
        syn::Data::Struct(data) => data.fields.members(),
        _ => unimplemented!(),
    };

    let builder_fields = match &input.data {
        syn::Data::Struct(data) => data.fields.iter().map(|f| {
            let ident = &f.ident.clone().expect("field has no identifier");
            let ty = &f.ty;
            quote! {
                #ident: Option<#ty>,
            }
        }),
        _ => unimplemented!(),
    };

    let expanded = quote! {
        impl #name {
            fn builder() -> #builder_name {
                #builder_name {
                    #(#members: None,)*
                }
            }
        }

        pub struct #builder_name {
            #(#builder_fields)*
        }
    };

    proc_macro::TokenStream::from(expanded)
}
