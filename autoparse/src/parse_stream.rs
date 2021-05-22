use std::{cmp::min, collections::VecDeque};
use crate::{Parsable, ParseError};

pub struct ParseStream<T: Sized + Clone, U: Iterator<Item=T>> {
	inner: U,
	buffer: VecDeque<T>,
	buffer_position: usize,
	buffering: bool
}

impl<T: Sized + Copy, U: Iterator<Item=T>> ParseStream<T, U> {

	fn set_rewind_point(&mut self) {
		if self.buffering == true {
			for _ in 0..buffer_position {
				self.buffer.pop_front().unwrap();
			}
			self.buffer_position = 0;
		} else {
			self.buffering = true;
			self.buffer.clear();
		}
	}

	fn unset_rewind_point(&mut self) {
		for _ in 0..buffer_position {
			self.buffer.pop_front().unwrap();
		}
		self.buffer_position = 0;
		self.buffering = false;
	}

	fn rewind(&mut self) {
		self.buffer_position = 0;
		self.buffering = false;
	}

	fn next(&mut self) -> Option<T> {
		return if self.buffering {
			if let Some(next) = self.buffer.get(self.buffer_position) {
				self.buffer_position += 1;
				next
			} else {
				if let Some(next) = self.inner.next() {
					self.buffer.push_back(next.clone());
					self.buffer_position += 1;
					next
				} else {
					None
				}
			}
		} else {
			self.buffer_position += 1;
			self.buffer.next()
		}
	}

	fn read(&mut self, other: &mut [T]) -> usize  {
	}

	fn advance(&mut self, amount: usize) {
	 
	}

	fn take(&mut self, amount: usize) -> Self {
		let take = *self;
		self.advance(amount);
		take
	}
}

pub trait ParseTo<T: Sized + Clone, U: Parsable<T>> {
	fn try_parse_to(&mut self, position: usize) -> Result<(U, usize), ParseError<T>>;
}

impl<T: Sized + Clone, U: ParseStream<T>, V: Parsable<T>> ParseTo<T, V> for U {
	fn try_parse_to(&mut self, position: usize) -> Result<(V, usize), ParseError<T>> {
		V::try_parse(self, position)
	}
}
