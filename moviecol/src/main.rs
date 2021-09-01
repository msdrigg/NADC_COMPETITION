use std::io::{self, BufRead, Write};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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

fn get_calculations(stack_size: usize, queries: Vec<usize>) -> Vec<usize> {
    queries
        .par_iter()
        .map(|val| val * val * val * val)
        .collect()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut reader = stdin.lock();
    let mut out = stdout.lock();
    let test_cases = *get_next_test_case_queries(&mut reader).get(0).unwrap();
    for _ in 0..test_cases {
        let n_and_m = get_next_test_case_queries(&mut reader);
        let n = *n_and_m.get(0).unwrap();
        let _ = *n_and_m.get(1).unwrap();

        let queries = get_next_test_case_queries(&mut reader);

        let calculations = get_calculations(n, queries);

        let mut output = Vec::<u8>::new();
        for calculation in calculations {
            let _ = write!(output, "{} ", calculation);
        }

        *output.last_mut().unwrap() = b'\n';

        out.write(&output).expect("Stdout write error");
    }
}

fn get_next_test_case_queries(stdin: &mut io::StdinLock) -> Vec<usize> {
    let mut buffer = Vec::new();
    stdin.read_until(b'\n', &mut buffer).expect("Failed read");

    let ascii: std::str::SplitAsciiWhitespace<'static> = unsafe {
        let slice = std::str::from_utf8_unchecked(&buffer);
        std::mem::transmute(slice.split_ascii_whitespace())
    };

    ascii.map(|it| it.parse().unwrap()).collect()
}
