use std::marker::PhantomData;

use autoparse::{Parsable, ParseError, ParseStream, Writable};
use super::{MaySpaced, MayNotSpaced, MaySpace, MayNotSpace};
use dede::*;
use std::fmt::Debug;
#[derive(Debug, Clone, Deref, DerefMut, Default)]
pub struct Punchuated<V: Writable<char>, D: Writable<char> + Default> {
	#[deref]
	elements: Vec<V>,
	delimiter: PhantomData<D>
}

impl <V: Parsable<char> + Debug, D: Parsable<char> + Default + Debug> Parsable<char> for Punchuated<V, D> {
	fn try_parse_no_rewind(stream: &mut ParseStream<char, impl Iterator<Item=char>>, position: usize) -> Result<(Self, usize), ParseError<char>> {
		let mut elements: Vec<V> = vec![];
		let mut read = 0;
		if let Ok((Some(value), r)) = Option::<MayNotSpaced<V>>::try_parse(stream, position) {
			elements.push(value.inner);
			read += r;
		}

		while let Ok(((_delimiter, value), r)) = <(MaySpaced<D>, MayNotSpaced<V>)>::try_parse(stream, position) {
			read += r;
			elements.push(value.inner);
		}

		Ok((Self {
			elements,
			delimiter: Default::default()
		}, read))
	}
}

impl <V: Writable<char>, D: Writable<char> + Default> Writable<char> for Punchuated<V, D> {
	fn write(&self, stream: &mut Vec<char>) {
		let mut iter = self.iter();
		iter.next().write(stream);
		MayNotSpace::default().write(stream);

		for item in iter {
			D::default().write(stream);
			MaySpace::default().write(stream);
			MayNotSpace::default().write(stream);
			item.write(stream);
		}
	}
}

