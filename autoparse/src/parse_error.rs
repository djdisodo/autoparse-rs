use arrayvec::ArrayVec;

#[derive(Clone)]
pub struct ParseError<T> {
	pub expected: ArrayVec<ArrayVec<T, 10>, 1>, //as it's usually small
	pub at: u64
}

impl<T> ParseError<T> {
	pub fn new(expected: ArrayVec<ArrayVec<T, 10>, 1>, at: u64) -> Self {
		Self {
			expected,
			at
		}
	}
}