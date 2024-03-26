use std::io::Write;

fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let n = token.next::<usize>();
    if n == 1 {
        writeln!(out, "1").unwrap();
    } else if n == 2 || n == 3 {
        writeln!(out, "NO SOLUTION").unwrap();
    } else {
        for i in 0..(n >> 1) {
            write!(out, "{} ", (i << 1) + 2).unwrap();
        }
        for i in 0..(n >> 1) {
            write!(out, "{} ", (i << 1) + 1).unwrap();
        }
        if n & 1 == 1 {
            write!(out, "{n} ").unwrap();
        }
        writeln!(out, "").unwrap();
    }
}

pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
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
