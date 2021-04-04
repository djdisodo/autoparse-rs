use std::ops::{Deref, DerefMut};


#[derive(Clone, Debug, Default)]
pub struct ParseError<T> {
	pub expections: Vec<Expection<T>> //(expected, at)
}

#[derive(Clone, Debug)]
pub struct Expection<T> {
	pub expected: Vec<Vec<T>>,
	pub at: usize
}

impl<T> ParseError<T> {
	pub fn new(expected: Vec<Vec<T>>, at: usize) -> Self {
		Self {
			expections: vec![Expection {
				expected,
				at
			}]
		}
	}

	pub fn advance(mut self, amount: usize) -> Self {
		for exception in &mut self.expections {
			exception.advance(amount);
		}
		self
	}
}

impl<T> Deref for ParseError<T> {
	type Target = Vec<Expection<T>>;

	fn deref(&self) -> &Self::Target {
		&self.expections
	}
}

impl<T> DerefMut for ParseError<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.expections
	}
}

impl<T> Expection<T> {
	pub fn new(expected: Vec<Vec<T>>, at: usize) -> Self {
		Self {
			expected,
			at
		}
	}

	pub fn advance(&mut self, amount: usize) {
		self.at += amount;
	}
}
