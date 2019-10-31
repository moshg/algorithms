/// Extension of io module.
pub mod io_ext {
    use std::mem;
    use std::io::BufRead;

    pub struct Reader<R> {
        buf: String,
        inner: R,
    }

    impl<R> Reader<R> {
        #[inline]
        pub fn new(inner: R) -> Self {
            Reader { buf: String::new(), inner: inner }
        }

        #[inline]
        pub fn into_inner(self) -> R {
            self.inner
        }
    }

    impl<R: BufRead> Reader<R> {
        #[allow(deprecated)]
        #[inline]
        pub fn read_line(&mut self) -> &str {
            self.buf.clear();
            self.inner.read_line(&mut self.buf).unwrap_or_else(|e| panic!("{}", e));
            self.buf.trim_right()
        }

        pub fn read_lines(&mut self, n: usize) -> Lines<R> {
            Lines { reader: self, n: n }
        }
    }

    pub struct Lines<'a, R> {
        reader: &'a mut Reader<R>,
        n: usize,
    }

    impl<'a, R: BufRead> Iterator for Lines<'a, R> {
        type Item = &'a str;

        fn next(&mut self) -> Option<&'a str> {
            if self.n > 0 {
                self.n -= 1;
                unsafe { Some(mem::transmute::<_, &'a str>(self.reader.read_line())) }
            } else {
                None
            }
        }
    }
}

/// Module supporting to parse strings.
pub mod parse {
    use std::str::FromStr;
    use std::borrow::Borrow;

    pub trait ParseNext: Iterator {
        fn parse_next<F: FromStr>(&mut self) -> F;
    }

    impl<'a, S: Borrow<str>, I: Iterator<Item=S>> ParseNext for I {
        fn parse_next<F: FromStr>(&mut self) -> F {
            if let Some(s) = self.next() {
                match s.borrow().parse() {
                    Ok(x) => x,
                    Err(_) => panic!("provided string cannot be parsed")
                }
            } else {
                panic!("iterator has no next element")
            }
        }
    }
}