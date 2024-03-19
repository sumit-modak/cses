fn main() {
    let mut token = Scanner::new(std::io::stdin().lock());
    let n = token.next::<usize>();
    let mut v = vec![0u32; n + 1];
    for i in 1..=n {
        let value = token.next::<usize>();
        v[value] = i as u32;
    }
    // count is taken as 1 for counting the first round
    let mut count = 1;
    for i in 1..n {
        if v[i] > v[i + 1] {
            count += 1;
        }
    }
    println!("{}", count);
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
