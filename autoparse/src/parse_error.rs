use std::ops::{Deref, DerefMut};


#[derive(Clone, Debug, Default)]
pub struct ParseError<T> {
	pub expectations: Vec<Expectation<T>> //(expected, at)
}

#[derive(Clone, Debug)]
pub struct Expectation<T> {
	pub expected: Vec<ExpectedValue<T>>,
	pub at: usize
}

pub enum ExpectedValue<T> {
	Owned(T),
	String(String)
}

impl<T> ParseError<T> {
	pub fn new(expected: Vec<ExpectedValue<T>>, at: usize) -> Self {
		Self {
			expectations: vec![Expectation {
				expected,
				at
			}]
		}
	}

	pub fn advance(mut self, amount: usize) -> Self {
		for exception in &mut self.expectations {
			exception.advance(amount);
		}
		self
	}
}

impl<T> Deref for ParseError<T> {
	type Target = Vec<Expectation<T>>;

	fn deref(&self) -> &Self::Target {
		&self.expectations
	}
}

impl<T> DerefMut for ParseError<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.expectations
	}
}

impl<T> Expectation<T> {
	pub fn new(expected: Vec<ExpectedValue<T>>, at: usize) -> Self {
		Self {
			expected,
			at
		}
	}

	pub fn advance(&mut self, amount: usize) {
		self.at += amount;
	}
}
