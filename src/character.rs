use syn;
use quote::{quote, ToTokens, TokenStreamExt};
use proc_macro;


#[proc_macro_derive(MyDebug)]
pub fn my_debug_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct or enum
    let name = &input.ident;

    // Generate the implementation of the Debug trait
    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "Debug: {}", stringify!(#name))
            }
        }
    };

    // Convert the generated code back into a TokenStream
    TokenStream::from(expanded)
}

pub trait Character{

    fn health(&self) ->  &f32;
    fn health_mut(&mut self) -> &mut f32;
    fn damage(&mut self, damage : f32) {
        *self.health_mut() -= damage;
    }
}

