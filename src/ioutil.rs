use std::io;


pub fn read_full<R: io::Read>(f: &mut R, buf: &mut [u8]) -> io::Result<bool> {
    let n = try!(f.read(buf));
    if n == buf.len() {
        Ok(true)
    } else if n == 0 {
        Ok(false)
    } else {
        use std::io::{Error, ErrorKind};
        Err(Error::new(ErrorKind::UnexpectedEof,
                       format!("partial read of {} bytes, expected {}", n, buf.len())))
    }
}
