pub struct PacketIterator<'a, T, F> where 
    F: Fn(&[T]) -> usize {
    buffer: &'a [T],
    get_size_func: F
}

impl<'a, T, F> PacketIterator<'a, T, F> where 
    F: Fn(&[T]) -> usize {
    pub fn new(buffer: &'a [T], protocol: F) -> PacketIterator<T, F> {
        PacketIterator{
            buffer: buffer,
            get_size_func: protocol
        }
    }
}

impl<'a, T, F> Iterator for PacketIterator<'a, T, F> where
    F: Fn(&[T]) -> usize {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            return None
        }
        let next_packet_size = (self.get_size_func)(self.buffer);
        if next_packet_size == 0 || next_packet_size > self.buffer.len() {
            return None
        }
        let next_packet = &self.buffer[0..next_packet_size];
        self.buffer = &self.buffer[next_packet_size..];
        Some(next_packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn protocol(buf: &[i32]) -> usize {
        if buf.is_empty() {
            return 0
        }
        buf[0] as usize
    }

    #[test]
    fn empty_buffer() {
        let buffer = vec![];
        let mut it = PacketIterator::new(&buffer, protocol);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn complete_packets() {
        let buffer = vec![2, 0, 4, 1, 0, 0, 3, 0, 8];
        let mut it = PacketIterator::new(&buffer, protocol);
        assert_eq!(it.next(), Some(&buffer[0..2]));
        assert_eq!(it.next(), Some(&buffer[2..6]));
        assert_eq!(it.next(), Some(&buffer[6..9]));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn complete_packets_with_lambda() {
        let protocol = |buf: &[i32]| -> usize {
            protocol(buf)
        };
        let buffer = vec![2, 0, 4, 1, 0, 0, 3, 0, 8];
        let mut it = PacketIterator::new(&buffer, protocol);
        assert_eq!(it.next(), Some(&buffer[0..2]));
        assert_eq!(it.next(), Some(&buffer[2..6]));
        assert_eq!(it.next(), Some(&buffer[6..9]));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn ignore_last_packet_which_is_incomplete1() {
        let buffer = vec![2, 0, 4, 1, 0, 0, 3, 0, 8, 3];
        let mut it = PacketIterator::new(&buffer, protocol);
        assert_eq!(it.next(), Some(&buffer[0..2]));
        assert_eq!(it.next(), Some(&buffer[2..6]));
        assert_eq!(it.next(), Some(&buffer[6..9]));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }
        
    #[test]
    fn ignore_last_packet_which_is_incomplete2() {
        let buffer = vec![2, 0, 4, 1, 0, 0, 3, 0, 8, 3, 0];
        let mut it = PacketIterator::new(&buffer, protocol);
        assert_eq!(it.next(), Some(&buffer[0..2]));
        assert_eq!(it.next(), Some(&buffer[2..6]));
        assert_eq!(it.next(), Some(&buffer[6..9]));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn second_packet_is_corrupt() {
        let buffer = vec![1, 0, 4, 1, 0, 0, 3, 0, 8, 3, 0];
        let mut it = PacketIterator::new(&buffer, protocol);
        assert_eq!(it.next(), Some(&buffer[0..1]));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }
    
}
