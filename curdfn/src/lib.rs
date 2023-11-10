
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
mod curd;
use curd::Context;


#[proc_macro_derive(CURD)]
pub fn curd_fn(input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as DeriveInput);
    let context = Context::new(input);
    context.generate().into()
    // let struct_name ;
    // let field_name ;
    // let ty;
    // let add_field_name;


    // quote!(
    //     impl #struct_name {
    //         pub fn #add_field_name(&mut self,value: #ty){
    //             self.#field_name.push(value)
    //         }
    //     }
    // );
}
