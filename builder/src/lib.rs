use proc_macro::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let b_name = Ident::new(
        format!("{}Builder", name).as_str(),
        Span::call_site().into(),
    );

    let members = match &input.data {
        syn::Data::Struct(data) => data.fields.members(),
        _ => unimplemented!(),
    };

    let b_fields = builder_fields(&input.data);
    let b_setters = builder_setters(&input.data);

    let expanded = quote! {
        impl #name {
            fn builder() -> #b_name {
                #b_name {
                    #(#members: None,)*
                }
            }
        }

        pub struct #b_name {
            #(#b_fields)*
        }

        impl #b_name {
            #(#b_setters)*
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn builder_fields(data: &Data) -> impl Iterator<Item = TokenStream> + '_ {
    match data {
        syn::Data::Struct(data) => data.fields.iter().map(|f| {
            let ident = &f.ident.clone().expect("field has no identifier");
            let ty = &f.ty;
            quote! {
                #ident: Option<#ty>,
            }
        }),
        _ => unimplemented!(),
    }
}

fn builder_setters(data: &Data) -> impl Iterator<Item = TokenStream> + '_ {
    match data {
        syn::Data::Struct(data) => data.fields.iter().map(|f| {
            let ident = &f.ident.clone().expect("field has no identifier");
            let ty = &f.ty;
            quote! {
                fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        }),
        _ => unimplemented!(),
    }
}
