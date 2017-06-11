use std::{io, fs};
use hashstore::HashInserter;
use {Reference, RefType};


pub struct Inserter<'a> {
    hins: HashInserter<'a>,
}


impl<'a> Inserter<'a> {
    pub fn wrap_hash_inserter(hins: HashInserter<'a>) -> io::Result<Inserter<'a>> {
        Ok(Inserter { hins: hins })
    }

    pub fn commit(self) -> io::Result<Reference> {
        let hash = try!(self.hins.commit());
        Ok(Reference {
            reftype: RefType::Data,
            hash: hash,
        })
    }
}


pub struct Reader<R>(R);

pub type FileReader = Reader<fs::File>;


impl<R> Reader<R> {
    pub fn wrap_read(f: R) -> io::Result<Reader<R>> {
        Ok(Reader(f))
    }
}


// Can't find newtype derivation yet, so boilerplate:
impl<R: io::Read> io::Read for Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}
