use crate::{Parsable, ParseError};
use std::io::Cursor;

pub struct Space {
	spaces: Vec<char>
}

impl Parsable<char> for Space {
	fn try_parse(buffer: &mut Cursor<&[char]>) -> Result<Self, ParseError<char>> {
		let mut spaces = Vec::new();
		while match buffer.next() {
			Some(char) => match char {
				' ' | '\t' | '\r' | '\n' => {
					spaces.push(char)
				},
				_ => false
			},
			None => false
		} {}
		if spaces.is_empty() {
			Err(ParseError::new([
				' ', '\t', '\r', '\n'
			].into(), buffer.position()))
		} else {
			Self {
				spaces
			}
		}
	}

	fn write(&self, buffer: &mut Vec<char>) {
		buffer.extend_from_slice(&self.spaces);
	}
}

pub type MaySpace = Option<Space>;