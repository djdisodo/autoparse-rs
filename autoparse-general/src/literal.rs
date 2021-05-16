use autoparse::{Parsable, Writable, ParseError, ParseStream};
use dede::*;
use snailquote::*;
use crate::token;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Literal {
	#[deref]
	pub inner: String
}

impl Writable<char> for Literal {
	fn write(&self, stream: &mut Vec<char>) {
		let escaped: String = escape(&self.inner).into(); //TODO this type of escape won't be compatible
		for c in escaped.chars() {
			stream.push(c);
		}
	}
}

impl Parsable<char> for Literal {
	fn try_parse_no_rewind(stream: &mut impl ParseStream<char>, position: usize) -> Result<(Self, usize), ParseError<char>> {
		let mut buffer = vec![];
		let mut reader = ['\0'];
		let mut read = 1;
		
		if let Err(e) = token::SnailQuote::try_parse_no_rewind(stream, position) {
			return Err(e);
		}
		while {
			read += stream.read(&mut reader);
			reader[0] != '\"'
		} {
			buffer.push(reader[0]);
			if reader[0] == '\\' {
				read += stream.read(&mut reader);
				buffer.push(reader[0]);
			}
		}

		let collected: String = buffer.into_iter().collect();

		let unescaped = unescape(&collected).unwrap(); //TODO handle error

		Ok((Self {
			inner: unescaped
		}, read))
	}
}
