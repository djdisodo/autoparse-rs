use crate::{MayNotSpaced, token};
use autoparse::Parsable;
use autoparse_derive::*;
use dede::*;

token!(BracketOpen, "[");

token!(BracketClose, "]");

#[derive(Parsable, Writable, Debug, Clone, Deref, DerefMut)]
#[autoparse_for(char)]
pub struct Bracketed<T: Parsable<char>> {
	pub open: MayNotSpaced<BracketOpen>,
	#[deref]
	pub inner: MayNotSpaced<T>,
	pub close: BracketClose
}
