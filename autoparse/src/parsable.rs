use crate::ParseError;
use crate::ParseStream;
use crate::Writable;

pub trait Parsable<T: Copy + Sized>: Writable<T> {
	fn try_parse_no_rewind<'a>(stream: &'a mut ParseStream<'a, T, impl Iterator<Item=T> + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>>;
	fn try_parse<'a>(mut stream: &'a mut ParseStream<'a, T, impl Iterator<Item=T> + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>> {
		stream.set_rewind_point();
		match {
			let mut forked = stream.fork();
			Self::try_parse_no_rewind(&mut forked, position)}
		{
			Ok(parsed) => {
				stream.rewind();
				stream.advance(parsed.1); //TODO rework parse_stream
				stream.unset_rewind_point();
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
	fn try_parse_no_rewind<'a>(stream: &'a mut ParseStream<'a, T, impl Iterator<Item=T> + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>> {
		Ok(match U::try_parse(stream, position) {
			Ok((parsed, r)) => (Some(parsed), r),
			Err(_) => (None, 0)
		})
	}
}

impl<T: Copy + Sized, U: Parsable<T>> Parsable<T> for Vec<U> {
	fn try_parse_no_rewind<'a>(stream: &'a mut ParseStream<'a, T, impl Iterator<Item=T> + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>> {
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
	fn try_parse_no_rewind<'a>(stream: &'a mut ParseStream<'a, T, impl Iterator<Item=T> + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>> {
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