use std::cmp::min;

pub trait Buf<T: Sized + Copy> {

    fn has_remaining(&self) -> bool;
    
    fn read(&mut self, other: &mut [T]) -> usize;

    fn advance(&mut self, amount: usize);
    
    fn take(&mut self, amount: usize) -> Self;
}

impl<T: Sized + Copy> Buf<T> for &[T] {

    fn has_remaining(&self) -> bool {
        self.len() > 0
    }

    fn read(&mut self, other: &mut [T]) -> usize  {
        let len = min(self.len(), other.len());
        (&mut other[..len]).copy_from_slice(&(*self)[..len]);
        self.advance(len);
        len
    }

    fn advance(&mut self, amount: usize) {
        *self = &(*self)[amount..];
    }

    fn take(&mut self, amount: usize) -> Self {
        let take = &(*self)[..amount];
        self.advance(amount);
        take
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
