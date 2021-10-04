use autoparse::{Parsable, Writable, ParseError, ParseStream, ParseStreamReference};
use crate::token;
use autoparse_derive::*;

#[derive(Clone, Debug)]
pub struct UnsignedInteger {
	pub chars: Vec<char>
}

impl Parsable<char> for UnsignedInteger {
	fn try_parse_no_rewind<'a>(stream: &mut ParseStream<'a, char, impl ParseStreamReference<char> + ?Sized + 'a>, position: usize) -> Result<(Self, usize), ParseError<char>> {
		let mut literal = vec![];
		let mut read = 0;
		let mut reader = ['\0'];
		stream.set_rewind_point();
		while {
			read += stream.read(&mut reader);
			reader[0].is_numeric()
		} {
			literal.push(reader[0]);
		}
		stream.rewind(1);
		read -= 1;
		if !literal.is_empty() {
			Ok((Self {
				chars: literal
			}, read))	
		} else {
			//TODO expected numeric
			Err(ParseError::new(vec![vec!['0']], position))
		}
	}
}

impl Writable<char> for UnsignedInteger {
	fn write(&self, stream: &mut Vec<char>) {
		stream.extend_from_slice(&self.chars)
	}
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub struct UnsignedFloat {
	pub integer: UnsignedInteger,
	pub point: token::Dot,
	pub decimal: UnsignedInteger
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub enum Unsigned {
	Float(UnsignedFloat),
	Integer(UnsignedInteger)
}


#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub struct SignedInteger {
	pub sign: Option<token::Dash>,
	pub integer: UnsignedInteger
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub struct SignedFloat {
	pub sign: Option<token::Dash>,
	pub integer: UnsignedFloat
}

#[derive(Clone, Debug, Writable, Parsable)]
#[autoparse_for(char)]
pub enum Signed {
	Float(SignedFloat),
	Integer(SignedInteger)
}
