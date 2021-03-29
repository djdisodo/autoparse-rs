use crate::{Parsable, ParseError};
use std::io::{Cursor, Read};

/// plz use zero sized type!
/// if you're not sure use macro
trait Token<T> {
	const TOKEN: &'static [T];
}

impl<T, U: Token<T>> Parsable<T> for U {
	fn try_parse(buffer: &mut Cursor<&[T]>) -> Result<Self, ParseError<T>> {
		let mut check = [0; U::TOKEN.len()];
		buffer.read(&mut check);
		if check == U::TOKEN {
			Ok(Default::default())
		} else {
			Err(ParseError::new([Self::TOKEN.into()].into(), 0))
		}
	}

	fn write(&self, buffer: &mut Vec<T>) {
		buffer.extend_from_slice(U::TOKEN)
	}
}
