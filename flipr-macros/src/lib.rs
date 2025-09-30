//! Procedural macros for converting Rust functions to operation descriptions.
//!
//! This crate provides macros that transform regular Rust functions into
//! data structures that can be executed on different backends.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Transform a Rust function into an image processing operation.
///
/// This macro converts a function that processes pixels into an operation
/// description that can be executed on different backends (CPU, GPU).
///
/// # Example
///
/// ```ignore
/// use flipr_macros::image_op;
///
/// #[image_op]
/// fn brighten(pixel: u8, amount: f64) -> u8 {
///     (pixel as f64 * amount).min(255.0) as u8
/// }
/// ```
#[proc_macro_attribute]
pub fn image_op(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let vis = &input.vis;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let block = &input.block;

    // Generate both the original function and an operation builder
    let expanded = quote! {
        #vis fn #fn_name(#inputs) #output {
            #block
        }

        /// Operation builder for the function.
        #[allow(non_camel_case_types)]
        pub struct #fn_name;

        impl #fn_name {
            /// Get the name of this operation.
            pub fn name() -> &'static str {
                #fn_name_str
            }
        }
    };

    TokenStream::from(expanded)
}

/// Mark a function as GPU-compatible.
///
/// This attribute indicates that the function can be compiled to GPU kernels.
///
/// # Example
///
/// ```ignore
/// use flipr_macros::gpu_compatible;
///
/// #[gpu_compatible]
/// fn add_pixels(a: u8, b: u8) -> u8 {
///     a.saturating_add(b)
/// }
/// ```
#[proc_macro_attribute]
pub fn gpu_compatible(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let vis = &input.vis;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let block = &input.block;

    // For now, just pass through the function with metadata
    let expanded = quote! {
        #[doc = "This function is GPU-compatible"]
        #vis fn #fn_name(#inputs) #output {
            #block
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for creating operation descriptions from structs.
///
/// This macro generates the necessary trait implementations for converting
/// a struct into an executable operation.
///
/// # Example
///
/// ```ignore
/// use flipr_macros::Operation;
///
/// #[derive(Operation)]
/// struct Blur {
///     radius: f64,
/// }
/// ```
#[proc_macro_derive(Operation)]
pub fn derive_operation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            /// Create a new operation instance.
            pub fn new() -> Self {
                Self::default()
            }
        }
    };

    TokenStream::from(expanded)
}
