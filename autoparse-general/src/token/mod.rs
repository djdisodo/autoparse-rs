mod bracket;
mod brace;

pub use bracket::*;
pub use brace::*;

#[macro_export]
macro_rules! token {
	($id:ident, $raw:expr) => {
		#[derive(Clone, Debug, Default)]
		pub struct $id;

		impl $id {
			pub const TOKEN: &'static str = $raw;
		}

		impl autoparse::Parsable<char> for $id {
			fn try_parse_no_rewind<'a>(stream: &mut autoparse::ParseStream<'a, char, impl autoparse::ParseStreamReference<char> + ?Sized + 'a>, position: usize) -> Result<(Self, usize), autoparse::ParseError<char>> {
				let token: Vec<char> = Self::TOKEN.chars().collect();
				let mut check = vec![0 as char; token.len()];
				stream.read(&mut check);
				if check == token {
					Ok((Self, Self::TOKEN.len()))
				} else {
					Err(autoparse::ParseError::new([autoparse::ExpectedValue::String(Self::TOKEN.to_string())].into(), position))
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

token!(Bang, "!");
token!(Colon, ":");
token!(Comma, ",");
token!(Dash, "-");
token!(Dot, ".");
token!(SnailQuote, "\"");
