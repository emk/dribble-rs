extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::cmp::min;
use std::io::{self, Read};

/// Wrap an implementation of `Read`, and return bytes in small,
/// random-sized chunks when `read` is called.  This is slow, and no
/// attempt has been made to optimize it for performance.
pub struct DribbleReader<R: Read> {
    source: R,
    buffer: Vec<u8>,
    used: usize,
    available: usize
}

impl<R: Read> DribbleReader<R> {
    /// Create a new `DribbleReader`.  The `read` function will only return
    /// 0 if `source.read` returns 0.
    pub fn new(source: R) -> Self {
        DribbleReader{source: source, buffer: vec![0; 64],
                      used: 0, available: 0}
    }
}

impl<R: Read> Read for DribbleReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        assert!(self.used <= self.available);
        if self.used == self.available {
            self.used = 0;
            self.available = 0;
            self.available = try!(self.source.read(&mut self.buffer));
        }
        if self.available == 0 {
            Ok(0)
        } else {
            let mut rng = rand::thread_rng();
            let bytes = min(buf.len(),
                            min(Range::new(1, 5).ind_sample(&mut rng),
                                self.available -  self.used));
            for i in 0..bytes {
                buf[i] = self.buffer[self.used+i];
            }
            self.used += bytes;
            Ok(bytes)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn test_dribble_reader() {
        let input = b"This is my test data";
        let mut cursor = Cursor::new(input as &[u8]);
        let mut dribble = DribbleReader::new(&mut cursor);
        let mut output = vec!();
        dribble.read_to_end(&mut output).unwrap();
        assert_eq!(input as &[u8], &output as &[u8]);
    }
}
