//! for parsing spaces(blank)
//! TODO `MayNotSpace` and `MayNotSpaced`
use autoparse::{ExpectedValue, Parsable, ParseError, ParseStream, ParseStreamReference, Writable};
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
	fn try_parse_no_rewind<'a>(stream: &mut ParseStream<'a, char, impl ParseStreamReference<char> + ?Sized + 'a>, position: usize) -> Result<(Self, usize), ParseError<char>> {
		let mut spaces = Vec::new();
		while match { //TODO fix byte skipping
			stream.set_rewind_point();
			let mut c = ['\0'];
			stream.read(&mut c);
			c[0]
		} {
			c @ (' ' | '\t' | '\r' | '\n') => {
				spaces.push(c);
				true
			},
			_ => false
		} {}
		
		stream.rewind(1);
		if spaces.is_empty() {
			Err(ParseError::new([
							ExpectedValue::String("TAB, SPACE, CR, LF".to_string())
			].into(), position))
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
