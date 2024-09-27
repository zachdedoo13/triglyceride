extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{Expr, ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn time_event(attr: TokenStream, input: TokenStream) -> TokenStream {
   let input = parse_macro_input!(input as ItemFn);
   let attrs: Vec<Expr> = parse_macro_input!(attr with syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated).into_iter().collect();


   let profiler = attrs.get(0).unwrap();
   let name = attrs.get(1).unwrap();


   let fn_vis = &input.vis;
   let fn_attar = &input.attrs;
   let fn_sig = &input.sig;
   let fn_block = &input.block;

   let expanded = quote! {
        #(#fn_attar)*
        #fn_vis #fn_sig {
            triglyceride::open_profiler(&#profiler, |mut p| p.time_event_start(#name));
            #fn_block
            triglyceride::open_profiler(&#profiler, |mut p| p.time_event_end(#name));
        }
    };

   TokenStream::from(expanded)
}