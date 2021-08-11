use std::collections::VecDeque;
use crate::{Parsable, ParseError};
use std::fmt::Debug;

#[derive(Debug)]
enum ParseStreamReference<'a, T: Sized + Clone, U: Iterator<Item=T> + 'a> {
	Forked(&'a mut ParseStream<'a, T, U>),
	Iter(&'a mut U)
}

impl<'a, T: Sized + Clone, U: Iterator<Item=T> + 'a> Iterator for ParseStreamReference<'a, T, U> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Self::Forked(forked) => forked.next(),
			Self::Iter(iter) => iter.next()
		}
	}
}

#[derive(Debug)]
pub struct ParseStream<'a, T: Sized + Clone, U: Iterator<Item=T> + 'a> {
	inner: ParseStreamReference<'a, T, U>,
	buffer: VecDeque<T>,
	buffer_position: usize,
	storing: bool
}

fn test() {
	let test_slice = vec![1, 2, 3, 4];
	let mut test_slice_iter = test_slice.into_iter();
	let mut test_parse_stream_base = ParseStream::from(&mut test_slice_iter);
	{
		let mut test_parse_stream_forked = test_parse_stream_base.fork();
		println!("{}", test_parse_stream_forked.next().unwrap());
	}
	// where compiles well ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
	test2(&mut test_parse_stream_base)
}

fn test2<'a>(stream: &'a mut ParseStream<'a, i32, impl Iterator<Item=i32> + 'a>) {
	{
		let mut forked = stream.fork();
		test2(&mut forked);
	}
	println!("{}", stream.next().unwrap());
}

impl<'a, T: Sized + Clone, U: Iterator<Item=T> + 'a> ParseStream<'a, T, U> {

	pub fn fork<'c: 'a>(&'c mut self) -> ParseStream<'c, T, U> {
		ParseStream {
			inner: ParseStreamReference::Forked(self),
			buffer: Default::default(),
			buffer_position: 0,
			storing: false
		}
	}

	pub fn set_rewind_point(&mut self) {
		for _ in 0..self.buffer_position {
			self.buffer.pop_front().unwrap();
		}
		self.buffer_position = 0;
		self.storing = true;
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
}

impl<'a, T: Sized + Clone, U: Iterator<Item=T> + 'a> Iterator for ParseStream<'a, T, U> {
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


impl<'a, T: Sized + Clone, U: Iterator<Item=T> + 'a> From<&'a mut U> for ParseStream<'a, T, U> {
	fn from(inner: &'a mut U) -> Self {
		Self {
			inner: ParseStreamReference::Iter(inner),
			buffer: Default::default(),
			buffer_position: Default::default(),
			storing: Default::default(),
		}
	}
}

/*
pub trait ParseTo<T: Sized + Copy, U: Parsable<T>> {
	fn try_parse_to(&mut self, position: usize) -> Result<(U, usize), ParseError<T>>;
}

impl<'a, T: Sized + Copy, U: Parsable<T>, V: Iterator<Item=T> + 'a> ParseTo<T, U> for ParseStream<'a, T, V> {
	fn try_parse_to(&mut self, position: usize) -> Result<(U, usize), ParseError<T>> {
		U::try_parse(self, position)
	}
}


*/