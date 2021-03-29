pub use crate::{Parsable, ParseError};
use std::io::{Cursor, Seek};

trait Auto<T: Clone> {
	fn try_parse(buffer: &mut Cursor<&[T]>) -> Result<Self, ParseError<T>>;

    fn try_parse_rewind_when_fail(buffer: &mut Cursor<&[T]>)  -> Result<Self, ParseError<T>> {
        let buffer_clone = buffer.clone();
        match Self::try_parse(buffer) {
            Ok(parsed) => Ok(parsed),
            Err(error) => {
                std::mem::replace(buffer, buffer_clone);
                Err(error)
            }
        }
    }

    fn write(&self, buffer: &mut Vec<T>);
}

impl<T: Clone, U: Parsable<T>> Auto<T> for Option<U> {
	fn try_parse(buffer: &mut Cursor<&[T]>) -> Result<Self, ParseError<T>> {
		Ok(match U::try_parse_rewind_when_fail(buffer) {
			Ok(parsed) => Some(parsed),
			Err(_) => None
		})
	}

	fn write(&self, buffer: &mut Vec<T>) {
		if let Some(inner) = self {
			inner.write(buffer)
		}
	}
}

impl<T: Clone, U: Parsable<T>> Auto<T> for Vec<U> {
	fn try_parse(buffer: &mut Cursor<&[T]>) -> Result<Self, ParseError<T>> {
		let mut new = Vec::new();
		let original = buffer.stream_len();
		while let Ok(parsed) = U::try_parse_rewind_when_fail(buffer) {
			new.push(parsed)
		}
		Ok(new)
	}

	fn write(&self, buffer: &mut Vec<T>) {
		for x in self {
			x.write(buffer)
		}
	}
}

impl<T: Clone, U1: Parsable<T>, U2: Parsable<T>> Auto<T> for (U1, U2) {
	fn try_parse(buffer: &mut Cursor<&[T]>) -> Result<Self, ParseError<T>> {
		let u1 = match U1::try_parse(buffer) {
			Ok(parsed) => parsed,
			Err(e) => return Err(e)
		};

		let u2 = match U2::try_parse(buffer) {
			Ok(parsed) => parsed,
			Err(e) => return Err(e)
		};
		Ok((u1, u2))
	}

	fn write(&self, buffer: &mut Vec<T>) {
		self.0.write(buffer);
		self.1.write(buffer);
	}
}
