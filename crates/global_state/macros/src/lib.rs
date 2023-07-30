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
                app.add_systems(bevy::ecs::schedule::OnExit(#ident::#variant_ident), global_state::cleanup_entities);
                app.add_systems(bevy::ecs::schedule::OnExit(#ident::#variant_ident), global_state::reset_state_time::<#ident>);
                app.add_systems(Update, global_state::update_state_time::<#ident>.run_if(in_state(#ident::#variant_ident)));
            });
        }
    }
    TokenStream::from(quote! {
        impl GlobalState for #ident {
            fn init_global_state(app: &mut bevy::app::App) {
                app.add_state::<#ident>();
                app.init_resource::<global_state::StateTime<#ident>>();
                #(#quotes)*
            }
        }
    })
}
