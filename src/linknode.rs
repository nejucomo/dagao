use std::{io, fs};
use std::io::Read;
use hashstore::HashInserter;
use {Reference, RefType};


const LINK_NODE_HEADER: &'static [u8] = b"dagao 0\n";
const LINK_NODE_HEADER_LEN: usize = 8; // Is there a better way?


pub struct Inserter<'a> {
    hins: HashInserter<'a>,
}

impl<'a> Inserter<'a> {
    pub fn wrap_hash_inserter(mut hins: HashInserter<'a>) -> io::Result<Inserter<'a>> {
        use std::io::Write;

        let n = try!(hins.write(LINK_NODE_HEADER));
        if n == LINK_NODE_HEADER.len() {
            Ok(Inserter { hins: hins })
        } else {
            // FIXME Handle 0 < n < LINK_NODE_HEADER.len().
            use std::io::{Error, ErrorKind};
            Err(Error::new(ErrorKind::WriteZero,
                           "could not write header in single write"))
        }
    }

    pub fn commit(self) -> io::Result<Reference> {
        let hash = try!(self.hins.commit());
        Ok(Reference {
            reftype: RefType::Link,
            hash: hash,
        })
    }
}


pub struct Reader<R> {
    f: R,
}

pub type FileReader = Reader<fs::File>;

impl<R: Read> Reader<R> {
    pub fn wrap_read(mut f: R) -> io::Result<Reader<R>> {
        use std::borrow::BorrowMut;
        use ioutil::read_full;

        let mut buf = [0u8; LINK_NODE_HEADER_LEN];
        try!(read_full(&mut f, buf.borrow_mut()));
        if buf == LINK_NODE_HEADER {
            Ok(Reader { f: f })
        } else {
            use std::io::{Error, ErrorKind};
            Err(Error::new(ErrorKind::InvalidData, "header unrecognized"))
        }
    }

    pub fn read_next(&mut self) -> io::Result<Option<Reference>> {
        Reference::read_from_file(&mut self.f)
    }
}


#[cfg(test)]
mod tests {}
