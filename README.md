The `dribble` library helps you test implementations of the Rust traits
`std::io::Read` and `std::io::Write` by passing data to them in small,
random-sized chunks.  This allows you to stress-test the code you run near
buffer boundaries.

```rust
extern crate dribble;
```

### Reading data in tiny chunks

```rust
use std::io::{Cursor, Read};
use dribble::DribbleReader;

let input = b"This is my test data";
let mut cursor = Cursor::new(input as &[u8]);
let mut dribble = DribbleReader::new(&mut cursor);
let mut output = vec!();
dribble.read_to_end(&mut output).unwrap();

assert_eq!(input as &[u8], &output as &[u8]);
```

### Writing data in tiny chunks

```rust
use std::io::Write;
use dribble::DribbleWriter;

let input = b"This is my test data";
let mut output = vec!();
{
    let mut dribble = DribbleWriter::new(&mut output);
    dribble.write(input).unwrap();
}

assert_eq!(input as &[u8], &output as &[u8]);
```
