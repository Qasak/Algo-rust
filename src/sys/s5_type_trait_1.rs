
use std::fmt;
use std::fmt::Formatter;
use std::io::{BufWriter, Write};
use std::net::TcpStream;

// #[derive(Debug)]
struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

// 实现 Debug trait，打印字符串
impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // 把 buf 添加到 BufBuilder 的尾部
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // 由于是在内存中操作，所以不需要 flush
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use std::io::Write;
    use crate::sys::s5_type_trait_1::BufBuilder;

    #[test]
    fn write_test() {
        let mut buf = BufBuilder::new();
        buf.write_all(b"write something").unwrap();
        println!("{:?}", buf);
    }
}

