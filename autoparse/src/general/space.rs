use crate::{Parsable, ParseError};
use slicebuffer::Buf;

pub struct Space {
	spaces: Vec<char>
}

impl Parsable<char> for Space {
	fn try_parse(buffer: &mut &[char]) -> Result<(Self, usize), ParseError<char>> {
		let mut spaces = Vec::new();
		while match {
			let mut c = ['\0'; 1];
			buffer.read(&mut c);
			c[0]
		} {
			
			c @ (' ' | '\t' | '\r' | '\n') => {
				spaces.push(c);
				true
			},
			_ => false
		} {}
		if spaces.is_empty() {
			Err(ParseError::new([
								vec![' '], vec!['\t'], vec!['\r'], vec!['\n']
			].into(), 0))
		} else {
			let read = spaces.len();
			Ok((Self {
				spaces
			},read))
		}
	}

	fn write(&self, buffer: &mut Vec<char>) {
		buffer.extend_from_slice(&self.spaces);
	}
}

pub type MaySpace = Option<Space>;
/*
#[derive(autoparse_derive::Parsable)]
#[parse(char)]
pub struct MaySpaced<T: Parsable<char>> {
	head: MaySpace,
	inner: T,
	tail: MaySpace
}*/

#[derive(autoparse_derive::Parsable)]
#[parse(char)]
pub struct MaySpaced<T: Parsable<char>>(MaySpace, T, MaySpace);
