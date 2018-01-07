mod bridge;
mod err;
pub use bridge::*;
pub use err::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
