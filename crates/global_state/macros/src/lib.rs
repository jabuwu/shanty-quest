use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(GlobalState)]
pub fn derive_global_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let mut quotes = vec![];
    if let Data::Enum(enm) = input.data {
        for variant in enm.variants.iter() {
            let variant_ident = &variant.ident;
            quotes.push(quote! {
                app.add_system_set(bevy::ecs::schedule::SystemSet::on_exit(#ident::#variant_ident).with_system(global_state::cleanup_entities));
            });
        }
    }
    TokenStream::from(quote! {
        impl GlobalState for #ident {
            fn init_global_state(app: &mut bevy::app::App) {
                app.add_state(#ident::default());
                #(#quotes)*
            }
        }
    })
}
