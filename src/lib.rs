//! The `dribble` library helps you test implementations of the traits
//! `std::io::Read` and `std::io::Write` by passing data to them in small,
//! random-sized chunks.  This allows you to stress-test the code you run
//! near buffer boundaries.

extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::cmp::min;
use std::io::{self, Read, Write};

/// Wrap an implementation of `Read`, and return bytes in small,
/// random-sized chunks when `read` is called.
///
/// ```
/// use std::io::{Cursor, Read};
/// use dribble::DribbleReader;
///
/// let input = b"This is my test data";
/// let mut cursor = Cursor::new(input as &[u8]);
/// let mut dribble = DribbleReader::new(&mut cursor);
/// let mut output = vec!();
/// dribble.read_to_end(&mut output).unwrap();
/// assert_eq!(input as &[u8], &output as &[u8]);
/// ```
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

        // If we're out of buffered data, try to read some.
        if self.used == self.available {
            self.used = 0;
            self.available = 0;
            self.available = try!(self.source.read(&mut self.buffer));
        }

        if self.available == 0 {
            // We tried to read bytes and didn't get any, so pass it
            // through.
            Ok(0)
        } else {
            // Decide how many bytes to copy, limiting it to the smallest
            // of (1) the destination space available, (2) a random number
            // between 1 and 4 inclusive, and (3) the number of bytes
            // currently buffered.
            let mut rng = rand::thread_rng();
            let bytes = min(buf.len(),
                            min(Range::new(1, 5).ind_sample(&mut rng),
                                self.available -  self.used));
            assert!(1 <= bytes && bytes <= 4);

            // Copy bytes, mark them as used, and return.
            for i in 0..bytes { buf[i] = self.buffer[self.used+i]; }
            self.used += bytes;
            Ok(bytes)
        }
    }
}

/// Wrap an implementation of `Write`, and pass through bytes in small,
/// random-sized chunks when `write` is called.
///
/// ```
/// use std::io::Write;
/// use dribble::DribbleWriter;
///
/// let input = b"This is my test data";
/// let mut output = vec!();
/// {
///     let mut dribble = DribbleWriter::new(&mut output);
///     dribble.write(input).unwrap();
/// }
/// assert_eq!(input as &[u8], &output as &[u8]);        
/// ```
pub struct DribbleWriter<W: Write> {
    dest: W
}

impl<W: Write> DribbleWriter<W> {
    /// Create a new `DribbleWriter`.
    pub fn new(dest: W) -> Self {
        DribbleWriter{dest: dest}
    }
}

impl<W: Write> Write for DribbleWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut rng = rand::thread_rng();
        let mut written = 0;
        while written < buf.len() {
            // Decide how many bytes to write, not exceeding the number
            // available.
            let bytes = min(buf.len() - written,
                            Range::new(0, 5).ind_sample(&mut rng));

            // Write the bytes.
            try!(self.dest.write(&buf[written..written+bytes]));
            written += bytes;
        }
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.dest.flush()
    }
}
