fn main() {
    let mut token = Tokenizer::new();
    let n = token.next::<usize>();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(token.next::<u32>());
    }
    v.sort_unstable();

    let mid = v.len() >> 1;
    let median = v[mid] as i64;
    let mut cost = 0i64;
    for item in &v[..mid] {
        cost += median - *item as i64;
    }
    for item in &v[mid..] {
        cost += *item as i64 - median;
    }

    println!("{cost}");
}

struct Tokenizer {
    buf: Vec<String>,
    i: usize,
}

impl Tokenizer {
    pub fn new() -> Self {
        return Tokenizer {
            buf: Vec::<String>::new(),
            i: 0,
        };
    }

    fn read_line(&mut self) {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        self.buf = s.split_whitespace().map(str::to_string).collect();
        self.i = 0;
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        while self.i == self.buf.len() {
            self.read_line();
        }
        let t = self.buf[self.i].parse().unwrap();
        self.i += 1;
        return t;
    }

    #[allow(dead_code)]
    pub fn next_line(&self) -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        return s;
    }
}
