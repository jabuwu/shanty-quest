use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(AppState)]
pub fn derive_app_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let mut quotes = vec![];
    if let Data::Enum(enm) = input.data {
        for variant in enm.variants.iter() {
            let variant_ident = &variant.ident;
            quotes.push(quote! {
                app.add_system_set(bevy::ecs::schedule::SystemSet::on_exit(#ident::#variant_ident).with_system(app_state::cleanup_entities));
            });
        }
    }
    TokenStream::from(quote! {
        impl AppState for #ident {
            fn init_app_state(app: &mut bevy::app::App) {
                app.add_state(#ident::default());
                #(#quotes)*
            }
        }
    })
}
