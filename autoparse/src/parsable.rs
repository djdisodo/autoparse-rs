use crate::ParseError;
use crate::ParseStream;
use crate::Writable;

pub trait Parsable<T: Copy + Sized>: Writable<T> {
	fn try_parse_no_rewind(stream: &mut ParseStream<T, impl Iterator<Item=T>>, position: usize) -> Result<(Self, usize), ParseError<T>>;

	fn try_parse(mut stream: &mut ParseStream<T, impl Iterator<Item=T>>, position: usize) -> Result<(Self, usize), ParseError<T>> {
		stream.set_rewind_point();
		match Self::try_parse_no_rewind(&mut ParseStream::from(&mut stream), position) {
			Ok(parsed) => {
				stream.rewind();
				stream.advance(parsed.1); //TODO rework parse_stream
				Ok(parsed)
			},
			Err(error) => {
				stream.rewind();
				Err(error)
			}
		}
	}
}

impl<T: Copy + Sized, U: Parsable<T>> Parsable<T> for Option<U> {
	fn try_parse_no_rewind(stream: &mut ParseStream<T, impl Iterator<Item=T>>, position: usize) -> Result<(Self, usize), ParseError<T>> {
		Ok(match U::try_parse(stream, position) {
			Ok((parsed, r)) => (Some(parsed), r),
			Err(_) => (None, 0)
		})
	}
}

impl<T: Copy + Sized, U: Parsable<T>> Parsable<T> for Vec<U> {
	fn try_parse_no_rewind(stream: &mut ParseStream<T, impl Iterator<Item=T>>, position: usize) -> Result<(Self, usize), ParseError<T>> {
		let mut new = Vec::new();
		let mut read = 0;
		while let Ok((parsed, r)) = U::try_parse(stream, position + read) {
			new.push(parsed);
			read += r;
		}
		Ok((new, read))
	}
}

impl<T: Copy + Sized, U1: Parsable<T>, U2: Parsable<T>> Parsable<T> for (U1, U2) {
	fn try_parse_no_rewind(stream: &mut ParseStream<T, impl Iterator<Item=T>>, position: usize) -> Result<(Self, usize), ParseError<T>> {
		let mut read = 0;
		let u1 = match U1::try_parse_no_rewind(stream, position + read) {
			Ok((parsed, r)) => {
				read += r;
				parsed
			},
			Err(e) => return Err(e)
		};

		let u2 = match U2::try_parse_no_rewind(stream, position + read) {
			Ok((parsed, r)) => {
				read += r;
				parsed
			},
			Err(e) => return Err(e)
		};
		Ok(((u1, u2), read))
	}
}

/// this is a flag trait that means parsing is lossy
pub trait Lossy {
	
}
