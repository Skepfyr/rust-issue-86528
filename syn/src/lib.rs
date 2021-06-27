#![allow(unused_variables, unused_mut, clippy::new_without_default)]
use proc_macro2::Literal;

pub struct LitInt {}

pub fn foo() -> LitInt {
    let token = "".parse().ok().map(Literal::f64_unsuffixed).unwrap();

    LitInt {}
}
