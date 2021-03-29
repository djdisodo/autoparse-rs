
#[derive(Clone)]
pub struct ParseError<T> {
	pub expected: Vec<Vec<T>>,
	pub at: usize
}

impl<T> ParseError<T> {
	pub fn new(expected: Vec<Vec<T>>, at: usize) -> Self {
		Self {
			expected,
			at
		}
	}

	pub fn advance(mut self, amount: usize) -> Self {
		self.at += amount;
		self
	}
}
