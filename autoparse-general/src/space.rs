//! for parsing spaces(blank)
//! TODO `MayNotSpace` and `MayNotSpaced`
use autoparse::{Parsable, ParseError, ParseStream, Writable};
use autoparse_derive::*;
use dede::*;


/// parses space like `' '`, `'\t'`, `'\r'`, `'\n'`
#[derive(Debug, Clone)]
pub struct Space {
	pub spaces: Vec<char>
}

impl Default for Space {
	fn default() -> Self {
		Self {
			spaces: vec![' ']
		}
	}
}

impl Parsable<char> for Space {
	fn try_parse_no_rewind(stream: &mut impl ParseStream<char>) -> Result<(Self, usize), ParseError<char>> {
		let mut spaces = Vec::new();
		while match {
			let mut c = ['\0'; 1];
			stream.read(&mut c);
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
}

impl Writable<char> for Space {
	fn write(&self, stream: &mut Vec<char>) {
		stream.extend_from_slice(&self.spaces);
	}
}

/// optional space
pub type MaySpace = Option<Space>;

pub type MayNotSpace = MaySpace;

/// tailing space for `T`
#[derive(Parsable, Writable, Debug, Clone, Deref, DerefMut)]
#[autoparse_for(char)]
pub struct Spaced<T: Parsable<char>> {
	#[deref]
	pub inner: T,
	pub space: Space
}

impl<T: Parsable<char>> From<T> for Spaced<T> {
	fn from(inner: T) -> Self {
		Self {
			inner,
			space: Default::default()
		}
	}
}


/// tailing optional space for `T`
#[derive(Parsable, Writable, Debug, Clone, Deref, DerefMut)]
#[autoparse_for(char)]
pub struct MaySpaced<T: Parsable<char>> {
	#[deref]
	pub inner: T,
	pub space: MaySpace
}

impl<T: Parsable<char>> From<T> for MaySpaced<T> {
	fn from(inner: T) -> Self {
		Self {
			inner,
			space: Default::default()
		}
	}
}

pub type MayNotSpaced<T> = MaySpaced<T>;
