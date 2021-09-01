use std::io;

use place::m3::UnsafeScanner;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    place::m3::method3(&mut scan, &mut out);
    // place::m1::method1();
}
