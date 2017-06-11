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


pub struct Reader(fs::File);


impl Reader {
    pub fn wrap_file(f: fs::File) -> io::Result<Reader> {
        Ok(Reader(f))
    }
}


// Can't find newtype derivation yet, so boilerplate:
impl io::Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}
