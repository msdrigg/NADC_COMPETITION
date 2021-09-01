use std::io::{self, BufRead};
/// Same API as Scanner but nearly twice as fast, using horribly unsafe dark arts
/// **REQUIRES** Rust 1.34 or higher
pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// This function should be marked unsafe, but noone has time for that in a
    /// programming contest. Use at your own risk!
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
    let reader = io::stdin();
    let mut reader = reader.lock();
    let mut line1_buffer = String::new();
    reader.read_line(&mut line1_buffer).unwrap();
    let line1 = line1_buffer
        .split_ascii_whitespace()
        .map(|val| val.parse().unwrap())
        .collect::<Vec<usize>>();

    let n = line1[0];
    let m = line1[1];
    let k = line1[2];

    let points: Vec<(usize, usize)> = Vec::with_capacity(k);
}
