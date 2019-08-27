// HACK: This sequence needs to be repeated in each project.
//       See https://github.com/rust-lang/cargo/issues/5034
// For clippy lints see: https://rust-lang.github.io/rust-clippy/master
// For rustc lints see: https://doc.rust-lang.org/rustc/lints/index.html
#![warn(
    // Enable sets of warnings
    clippy::all,
    clippy::pedantic,
    // TODO: clippy::cargo,
    rust_2018_idioms,
    future_incompatible,
    unused,

    // Additional unused warnings (not included in `unused`)
    unused_lifetimes,
    unused_qualifications,
    unused_results,

    // Additional misc. warnings
    anonymous_parameters,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    // TODO: missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // TODO: unreachable_pub,
    unsafe_code,
    variant_size_differences
)]
#![cfg_attr(feature = "std", warn(
    // TODO: missing_debug_implementations,
))]

extern crate proc_macro;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn hex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_lib::hex(input.into()).into()
}

#[proc_macro_hack]
pub fn u256h(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_lib::u256h(input.into()).into()
}

#[proc_macro_hack]
pub fn field_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_lib::field_element(input.into()).into()
}
