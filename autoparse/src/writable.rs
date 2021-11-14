pub trait Writable<T: Clone + Sized>: Sized + Clone{
	fn write(&self, stream: &mut Vec<T>);
}

impl<T: Clone + Sized, U: Writable<T>> Writable<T> for Option<U> {
	fn write(&self, stream: &mut Vec<T>) {
		if let Some(inner) = self {
			inner.write(stream)
		}
	}
}

impl<T: Clone + Sized, U: Writable<T>> Writable<T> for Vec<U> {
	fn write(&self, stream: &mut Vec<T>) {
		for x in self {
			x.write(stream)
		}
	}
}

impl<T: Clone + Sized, U1: Writable<T>, U2: Writable<T>> Writable<T> for (U1, U2) {
	fn write(&self, stream: &mut Vec<T>) {
		self.0.write(stream);
		self.1.write(stream);
	}
}

impl<T: Clone + Sized, U: Writable<T>> Writable<T> for &U {
	fn write(&self, stream: &mut Vec<T>) {
		(*self).write(stream)
	}
}

impl<T: Clone + Sized, U: Writable<T>> Writable<T> for Box<U> {
	fn write(&self, stream: &mut Vec<T>) {
		(**self).write(stream)
	}
}
