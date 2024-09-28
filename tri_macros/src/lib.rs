extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{Expr, ItemFn, parse_macro_input};


/// times an event and adds it to the function tree,
/// takes a reference to a profiler static and a str name
/// ```
/// use triglyceride::{init_profiler, time_event};
/// 
/// init_profiler!(PROF, Settings::default());
/// 
/// #[time_event(PROF, "MAIN")]
/// fn main_update_function() {}
/// 
///
/// #[time_event(PROF, "INNER")]
/// fn inner_function() {}
/// ```
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


/// times an event without adding it to the function tree,
/// can be viewed in ``PerformanceProfiler::list_all_functions``
/// takes a reference to a profiler static and a str name
/// ```
/// use triglyceride::{init_profiler, time_lone_function};
///
/// init_profiler!(PROF, Settings::default());
///
/// fn main_update_function() {
///    repeated_function();
///    repeated_function();
/// }
///
/// #[time_lone_function(PROF, "REPEAT")]
/// fn repeated_function() {}
/// ```
#[proc_macro_attribute]
pub fn time_lone_function(attr: TokenStream, input: TokenStream) -> TokenStream {
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
            triglyceride::open_profiler(&#profiler, |mut p| p.start_time_function(#name));
            #fn_block
            triglyceride::open_profiler(&#profiler, |mut p| p.end_time_function(#name));
        }
    };

   TokenStream::from(expanded)
}