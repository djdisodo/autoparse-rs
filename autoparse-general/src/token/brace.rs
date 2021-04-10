use crate::{MayNotSpaced, token};
use autoparse::Parsable;
use autoparse_derive::*;
use dede::*;

token!(BraceOpen, "{");

token!(BraceClose, "}");

#[derive(Parsable, Writable, Debug, Clone, Deref, DerefMut)]
#[autoparse_for(char)]
pub struct Braced<T: Parsable<char>> {
	pub open: MayNotSpaced<BraceOpen>,
	#[deref]
	pub inner: MayNotSpaced<T>,
	pub close: BraceClose
}
