use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn graph_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse input as token tree
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let fields = &input.fields;

    // build a message
    let field_names: Vec<_> = fields
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();

    // so we want it to look like
    // code: 401
    // message: "Out Of Node Store"

    let display_parts = field_names.iter().map(|f| {
        let label = f.to_string();
        quote! { write!(f, "  {}: {:?}\n", #label, self.#f)?; }
    });

    quote! {
        // emit the original struct
        #input

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Error [{}]:\n", stringify!(#name))?;
                #(#display_parts)*
                Ok(())
            }
        }

        impl std::error::Error for #name {}
    }
    .into()
}
