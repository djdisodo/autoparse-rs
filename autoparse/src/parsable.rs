use crate::ParseError;

pub trait Parsable<T: Clone + Sized>: Sized {
	fn try_parse(buffer: &mut &[T]) -> Result<(Self, usize), ParseError<T>>;

	fn try_parse_rewind_when_fail(buffer: &mut &[T])  -> Result<(Self, usize), ParseError<T>> {
		let buffer_clone = buffer.clone();
		match Self::try_parse(buffer) {
			Ok(parsed) => Ok(parsed),
			Err(error) => {
				*buffer = buffer_clone;
				Err(error)
			}
		}
	}

	fn write(&self, buffer: &mut Vec<T>);
}

impl<T: Clone + Sized, U: Parsable<T>> Parsable<T> for Option<U> {
	fn try_parse(buffer: &mut &[T]) -> Result<(Self, usize), ParseError<T>> {
		Ok(match U::try_parse_rewind_when_fail(buffer) {
			Ok((parsed, r)) => (Some(parsed), r),
			Err(_) => (None, 0)
		})
	}

	fn write(&self, buffer: &mut Vec<T>) {
		if let Some(inner) = self {
			inner.write(buffer)
		}
	}
}

impl<T: Clone + Sized, U: Parsable<T>> Parsable<T> for Vec<U> {
	fn try_parse(buffer: &mut &[T]) -> Result<(Self, usize), ParseError<T>> {
		let mut new = Vec::new();
		let mut read = 0;
		while let Ok((parsed, r)) = U::try_parse_rewind_when_fail(buffer) {
			new.push(parsed);
			read += r;
		}
		Ok((new, read))
	}

	fn write(&self, buffer: &mut Vec<T>) {
		for x in self {
			x.write(buffer)
		}
	}
}

impl<T: Clone + Sized, U1: Parsable<T>, U2: Parsable<T>> Parsable<T> for (U1, U2) {
	fn try_parse(buffer: &mut &[T]) -> Result<(Self, usize), ParseError<T>> {
		let mut read = 0;
		let u1 = match U1::try_parse(buffer) {
			Ok((parsed, r)) => {
				read += r;
				parsed
			},
			Err(e) => return Err(e)
		};

		let u2 = match U2::try_parse(buffer) {
			Ok((parsed, r)) => {
				read += r;
				parsed
			},
			Err(e) => return Err(e.advance(read))
		};
		Ok(((u1, u2), read))
	}

	fn write(&self, buffer: &mut Vec<T>) {
		self.0.write(buffer);
		self.1.write(buffer);
	}
}
