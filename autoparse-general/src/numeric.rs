use autoparse::{Parsable, Writable, ParseError, ParseStream};
use crate::token;
use autoparse_derive::*;

#[derive(Clone, Debug)]
pub struct UnsignedInteger {
	pub literal: Vec<char>
}

impl Parsable<char> for UnsignedInteger {
	fn try_parse_no_rewind(stream: &mut impl ParseStream<char>) -> Result<(Self, usize), ParseError<char>> {
		let mut literal = vec![];
		let mut read = 0;
		let mut reader = ['\0'];
		let mut stream_fork = stream.clone();
		while {
			read += stream.read(&mut reader);
			reader[0].is_numeric()
		} {
			literal.push(reader[0]);
			stream_fork = stream.clone()
		}
		*stream = stream_fork;
		read -= 1;
		if !literal.is_empty() {
			Ok((Self {
				literal
			}, read))	
		} else {
			//TODO expected numeric
			Err(ParseError::new(vec![vec!['0']], 0))
		}
	}
}

impl Writable<char> for UnsignedInteger {
	fn write(&self, stream: &mut Vec<char>) {
		stream.extend_from_slice(&self.literal)
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
