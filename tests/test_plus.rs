#![allow(deprecated)]       // warning: use of deprecated function `rust_study::depends::plus::add`
extern crate rust_study;
pub use rust_study::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let res = super::depends::plus::add(2, 2);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_macros_vector() {
        super::macros::vector::example();
    }

    #[test]
    fn test_macros_typeof() {
        super::macros::type_of::example();
    }
}