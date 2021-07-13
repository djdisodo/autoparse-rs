use std::collections::VecDeque;
use crate::{Parsable, ParseError};
use std::vec::IntoIter;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct ParseStream<T: Sized + Clone, U: Iterator<Item=T>> {
	inner: U,
	buffer: VecDeque<T>,
	buffer_position: usize,
	storing: bool
}

impl<T: Sized + Clone, U: Iterator<Item=T>> ParseStream<T, U> {

	pub fn set_rewind_point(&mut self) {
		if self.storing == true {
			for _ in 0..self.buffer_position {
				self.buffer.pop_front().unwrap();
			}
			self.buffer_position = 0;
		} else {
			self.storing = true;
			self.buffer.clear();
		}
	}

	pub fn unset_rewind_point(&mut self) {
		for _ in 0..self.buffer_position {
			self.buffer.pop_front().unwrap();
		}
		self.buffer_position = 0;
		self.storing = false;
	}

	pub fn rewind(&mut self) {
		self.buffer_position = 0;
		self.storing = false;
	}

	pub fn read(&mut self, other: &mut [T]) -> usize  {
		for i in 0..other.len() {
			if let Some(next) = self.next() {
				other[i] = next;
			} else {
				return i;
			}
		}
		return other.len();
	}

	pub fn advance(&mut self, amount: usize) {
		for _ in 0..amount {
			self.next();
		}
	}

	pub fn take(&mut self, amount: usize) -> ParseStream<T, IntoIter<T>> {
		let mut buffer = Vec::with_capacity(amount);
		for _ in 0..amount {
			if let Some(next) = self.next() {
				buffer.push(next);
			} else {
				break;
			}
		}
		ParseStream::from(buffer.into_iter())
	}
	
	pub fn into_inner(self) -> U {
		self.inner
	}
}

impl<T: Sized + Clone, U: Iterator<Item=T>> Iterator for ParseStream<T, U> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		return if let Some(next) = self.buffer.get(self.buffer_position) {
			self.buffer_position += 1;
			Some(next.clone())
		} else {
			if let Some(next) = self.inner.next() {
				if self.storing {
					self.buffer.push_back(next.clone());
					self.buffer_position += 1;
				}
				Some(next)
			} else {
				None
			}
		}	
	}
}


impl<T: Sized + Clone, U: Iterator<Item=T>> From<U> for ParseStream<T, U> {
	fn from(inner: U) -> Self {
		Self {
			inner,
			buffer: Default::default(),
			buffer_position: Default::default(),
			storing: Default::default()
		}
	}
}

pub trait ParseTo<T: Sized + Copy, U: Parsable<T>> {
	fn try_parse_to(&mut self, position: usize) -> Result<(U, usize), ParseError<T>>;
}

impl<T: Sized + Copy, U: Parsable<T>, V: Iterator<Item=T>> ParseTo<T, U> for ParseStream<T, V> {
	fn try_parse_to(&mut self, position: usize) -> Result<(U, usize), ParseError<T>> {
		U::try_parse(self, position)
	}
}
