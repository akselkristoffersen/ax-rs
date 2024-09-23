pub struct PacketIterator<'a, T, F> where 
    F: Fn(&[T]) -> usize {
    buffer: &'a [T],
    get_size_func: F,
    size: usize,
}

impl<'a, T, F> PacketIterator<'a, T, F> where 
    F: Fn(&[T]) -> usize {
    pub fn new(buffer: &'a [T], protocol: F) -> PacketIterator<T, F> {
        PacketIterator{
            buffer: buffer,
            get_size_func: protocol,
            size: 0,
        }
    }
}

impl<'a, T, F> Iterator for PacketIterator<'a, T, F> where
    F: Fn(&[T]) -> usize {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.size >= self.buffer.len() {
            return None
        }
        self.buffer = &self.buffer[self.size..];
        self.size = (self.get_size_func)(self.buffer);
        if self.size == 0 || self.size > self.buffer.len() {
            return None
        }
        Some(&self.buffer[0..self.size])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let protocol = |slice: &[i32]| -> usize {
            if slice.is_empty() {
                return 0
            }
            slice[0] as usize
        };

        {
            let buffer = vec![2, 0, 4, 1, 0, 0, 3, 0, 8];
            let mut it = PacketIterator::new(&buffer, protocol);
            assert_eq!(it.next(), Some(&buffer[0..2]));
            assert_eq!(it.next(), Some(&buffer[2..6]));
            assert_eq!(it.next(), Some(&buffer[6..9]));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }

        {
            let buffer = vec![2, 0, 4, 1, 0, 0, 3, 0, 8, 3];
            let mut it = PacketIterator::new(&buffer, protocol);
            assert_eq!(it.next(), Some(&buffer[0..2]));
            assert_eq!(it.next(), Some(&buffer[2..6]));
            assert_eq!(it.next(), Some(&buffer[6..9]));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
        
        {
            let buffer = vec![2, 0, 4, 1, 0, 0, 3, 0, 8, 3, 0];
            let mut it = PacketIterator::new(&buffer, protocol);
            assert_eq!(it.next(), Some(&buffer[0..2]));
            assert_eq!(it.next(), Some(&buffer[2..6]));
            assert_eq!(it.next(), Some(&buffer[6..9]));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }

        {
            let buffer = vec![1, 0, 4, 1, 0, 0, 3, 0, 8, 3, 0];
            let mut it = PacketIterator::new(&buffer, protocol);
            assert_eq!(it.next(), Some(&buffer[0..1]));
            assert_eq!(it.next(), None);
            assert_eq!(it.next(), None);
        }
    }
}
