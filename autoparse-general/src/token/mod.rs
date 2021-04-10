mod brace;

pub use brace::*;

#[macro_export]
macro_rules! token {
	($id:ident, $raw:expr) => {
		#[derive(Clone, Debug)]
		pub struct $id;

		impl $id {
			pub const TOKEN: &'static str = $raw;
		}

		impl autoparse::Parsable<char> for $id {
			fn try_parse_no_rewind(stream: &mut impl autoparse::ParseStream<char>) -> Result<(Self, usize), autoparse::ParseError<char>> {
				let token: Vec<char> = Self::TOKEN.chars().collect();
				let mut check = vec![0 as char; token.len()];
				autoparse::ParseStream::<char>::read(stream, &mut check);
				if check == token {
					Ok((Self, Self::TOKEN.len()))
				} else {
					Err(autoparse::ParseError::new([token].into(), 0))
				}
			}
		}

		impl autoparse::Writable<char> for $id {
			fn write(&self, stream: &mut Vec<char>) {
				let token: Vec<char> = Self::TOKEN.chars().collect();
				stream.extend_from_slice(&token)
			}
		}
	}
}

token!(Comma, ",");
token!(Colon, ":");
token!(Dot, ".");


