use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, parse_macro_input, token, Data, DeriveInput, LitStr, Result};

struct Item {
    _paren_token: token::Paren,
    literal: LitStr,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Item {
            _paren_token: parenthesized!(content in input),
            literal: content.parse()?,
        })
    }
}

#[proc_macro_derive(AssetStruct, attributes(asset))]
pub fn derive_asset_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut load_quotes = vec![];
    let mut loaded_quotes = vec![];
    match input.data {
        Data::Struct(a) => {
            for field in a.fields.iter() {
                let field_ident = field.ident.clone().unwrap();
                let mut found = false;
                for attr in field.attrs.iter() {
                    if found {
                        panic!("multiple asset attributes for {}", field_ident.to_string());
                    }
                    found = true;
                    let tokens: TokenStream = attr.tokens.clone().into();
                    let item: Item = parse_macro_input!(tokens as Item);
                    let path = item.literal.value();
                    load_quotes.push(quote! {
                        self.#field_ident = asset_server.load(#path);
                    });
                    loaded_quotes.push(quote! {
                        self.#field_ident.clone_untyped()
                    });
                }
            }
        }
        _ => {}
    }
    let expanded = quote! {
        impl AssetStruct for #name {
            fn load_assets(&mut self, asset_server: &Res<AssetServer>) {
                #(#load_quotes)*
            }
            fn load_state(&self, asset_server: &Res<AssetServer>) -> bevy::asset::LoadState {
                let mut assets: Vec<HandleUntyped> = vec![
                    #(#loaded_quotes,)*
                ];
                asset_server.get_group_load_state(assets.iter().map(|h| h.id))
            }
        }
    };
    TokenStream::from(expanded)
}
