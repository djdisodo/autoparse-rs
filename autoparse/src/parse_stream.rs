use std::cmp::min;
use crate::{Parsable, ParseError};

pub trait ParseStream<T: Sized + Clone>: Clone {
	fn has_remaining(&self) -> bool;
	
	fn read(&mut self, other: &mut [T]) -> usize;

	fn advance(&mut self, amount: usize);
	
	fn take(&mut self, amount: usize) -> Self;

	fn get_span(&self) -> usize;
}

#[derive(Debug, Clone, Copy)]
pub struct SimpleParseStream<'a, T: Sized + Copy> {
	inner: &'a [T],
	position: usize
}

impl<'a, T: Sized + Copy> SimpleParseStream<'a, T> {
	pub fn new(inner: &'a [T]) -> Self {
		Self {
			inner,
			position: 0
		}
	}
}

impl<'a, T: Sized + Copy> ParseStream<T> for SimpleParseStream<'a, T> {
	fn has_remaining(&self) -> bool {
		self.inner.len() > self.position
	}

	fn read(&mut self, other: &mut [T]) -> usize  {
		let len = min(self.inner.len() - self.position, other.len());
		(&mut other[..len]).copy_from_slice(&(*self.inner)[self.position..][..len]);
		self.advance(len);
		len
	}

	fn advance(&mut self, amount: usize) {
		self.position = min(self.position + amount, self.inner.len());
	}

	fn take(&mut self, amount: usize) -> Self {
		let take = *self;
		self.advance(amount);
		take
	}

	fn get_span(&self) -> usize {
		self.position
	}
}

pub trait ParseTo<T: Sized + Clone, U: Parsable<T>> {
	fn try_parse_to(&mut self) -> Result<(U, usize), ParseError<T>>;
}

impl<T: Sized + Clone, U: ParseStream<T>, V: Parsable<T>> ParseTo<T, V> for U {
	fn try_parse_to(&mut self) -> Result<(V, usize), ParseError<T>> {
		V::try_parse(self)
	}
}
