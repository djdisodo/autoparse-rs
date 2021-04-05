mod parsable;
mod parse_error;

pub use parsable::Parsable;
pub use parse_error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
