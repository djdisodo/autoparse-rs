use std::cmp::min;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::intrinsics::transmute;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct ParseStreamInitial<'a, T: Sized + Clone, U: Iterator<Item=T> + ?Sized + 'a> {
	inner: &'a mut U,
	buffer: VecDeque<T>,
	buffer_position: usize,
	storing: bool
}
impl<'a, T: Sized + Clone, U: Iterator<Item=T> + ?Sized + 'a> Iterator for ParseStreamInitial<'a, T, U> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let res = if let Some(next) = self.buffer.get(self.buffer_position) {
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
		};
		return res;
	}
}

pub trait ParseStreamReference<T>: Iterator<Item=T> {
	fn rewind(&mut self, amount: usize);
}

impl<'a, T: Sized + Clone, U: Iterator<Item=T> + ?Sized + 'a> ParseStreamReference<T> for ParseStreamInitial<'a, T, U> {
	fn rewind(&mut self, amount: usize) {
		self.buffer_position -= min(self.buffer_position, amount);
	}
}




#[derive(Debug)]
pub struct ParseStream<'a, T: Sized + Clone, U: ParseStreamReference<T> + ?Sized + 'a> {
	inner: &'a mut U,
	buffer_position: usize,
	__d: PhantomData<T>
}

/*
fn test() {
	let test_slice = vec![1, 2, 3, 4];
	let mut test_slice_iter = test_slice.into_iter();
	let mut test_parse_stream_base = ParseStream::from(&mut test_slice_iter);
	{
		let mut test_parse_stream_forked = test_parse_stream_base.fork();
		println!("{}", test_parse_stream_forked.next().unwrap());
		drop(test_parse_stream_forked);
	}
	// where compiles well ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
	test2(&mut test_parse_stream_base)
}

fn test2<'a>(stream: &'a mut ParseStream<'a, i32, impl Iterator<Item=i32> + ?Sized + 'a>) {
	{
		let mut forked = stream.fork();
		test2(&mut forked);
	}
	println!("{}", stream.next().unwrap());
}
*/

impl<'a, T: Sized + Clone, U: ParseStreamReference<T> + ?Sized + 'a> ParseStreamReference<T> for ParseStream<'a, T, U> {
	fn rewind(&mut self, mut amount: usize) {
		amount = min(self.buffer_position, amount);
		self.buffer_position -= min(self.buffer_position, amount);
		self.inner.rewind(amount);
	}
}

impl<'a, T: Sized + Clone, U: ParseStreamReference<T> + ?Sized + 'a> ParseStream<'a, T, U> {

	pub fn fork<'b>(&'b mut self) -> ParseStream<'b, T, dyn ParseStreamReference<T>> {
		ParseStream {
			inner: self as &mut dyn ParseStreamReference<T>,
			buffer_position: 0,
			__d: Default::default()
		}
	}

	pub fn rewind_all(&mut self) {
		self.rewind(self.buffer_position);
	}

	pub fn set_rewind_point(&mut self) {
		self.buffer_position = 0;
	}

	pub fn read(&mut self, other: &mut [T]) -> usize  {
		for i in 0..other.len() {
			if let Some(next) = self.next() {
				//println!("read:{}", unsafe { transmute::<_, &char>(&next) });
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

impl<'a, T: Sized + Clone, U: ParseStreamReference<T> + ?Sized + 'a> Iterator for ParseStream<'a, T, U> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let result = self.inner.next();
		if result.is_some() {
			self.buffer_position += 1;
		}
		return result;
	}
}


impl<'a, T: Sized + Clone, U: Iterator<Item=T> + Sized + 'a> From<&'a mut U> for ParseStreamInitial<'a, T, U> {
	fn from(inner: &'a mut U) -> Self {
		Self {
			inner,
			buffer: Default::default(),
			buffer_position: Default::default(),
			storing: true,
		}
	}
}

impl<'a, T: Sized + Clone, U: ParseStreamReference<T> + Sized + 'a> From<&'a mut U> for ParseStream<'a, T, U> {
	fn from(inner: &'a mut U) -> Self {
		Self {
			inner,
			buffer_position: 0,
			__d: Default::default()
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