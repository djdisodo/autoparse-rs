use crate::{ParseError, ParseStreamReference};
use crate::ParseStream;
use crate::Writable;

pub trait Parsable<T: Clone + Sized>: Writable<T> {
	fn try_parse_no_rewind<'a>(stream: &mut ParseStream<'a, T, impl ParseStreamReference<T> + ?Sized + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>>;
	fn try_parse<'a>(stream: &mut ParseStream<'a, T, impl ParseStreamReference<T> + ?Sized + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>> {
		let mut forked = stream.fork();
		match {
			Self::try_parse_no_rewind(&mut forked, position)}
		{
			Ok(parsed) => {
				Ok(parsed)
			},
			Err(error) => {
				forked.rewind_all();
				Err(error)
			}
		}
	}
}

impl<T: Clone + Sized, U: Parsable<T>> Parsable<T> for Option<U> {
	fn try_parse_no_rewind<'a>(stream: &mut ParseStream<'a, T, impl ParseStreamReference<T> + ?Sized + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>> {
		Ok(match U::try_parse(stream, position) {
			Ok((parsed, r)) => (Some(parsed), r),
			Err(_) => (None, 0)
		})
	}
}

impl<T: Clone + Sized, U: Parsable<T>> Parsable<T> for Vec<U> {
	fn try_parse_no_rewind<'a>(stream: &mut ParseStream<'a, T, impl ParseStreamReference<T> + ?Sized + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>> {
		let mut new = Vec::new();
		let mut read = 0;
		while let Ok((parsed, r)) = U::try_parse(stream, position + read) {
			new.push(parsed);
			read += r;
		}
		Ok((new, read))
	}
}

impl<T: Clone + Sized, U1: Parsable<T>, U2: Parsable<T>> Parsable<T> for (U1, U2) {
	fn try_parse_no_rewind<'a>(stream: &mut ParseStream<'a, T, impl ParseStreamReference<T> + ?Sized + 'a>, position: usize) -> Result<(Self, usize), ParseError<T>> {
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