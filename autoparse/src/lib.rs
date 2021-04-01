pub mod general;

//pub mod token;

mod parsable;
mod parse_error;

pub use parsable::Parsable;
pub use parse_error::ParseError;

//macro hack
use crate as autoparse;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
