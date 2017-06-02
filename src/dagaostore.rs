use std::path::{Path, PathBuf};
use hashstore::{HashInserter, HashStore};


pub struct DagaoStore {
    indexdir: PathBuf,
    hstore: HashStore,
}

impl DagaoStore {
    pub fn create(indexdir: &Path, hashstore: HashStore) -> io::Result<DagaoStore> {
        match fs::create_dir(indexdir) {
            Ok(()) => {}
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::AlreadyExists => {
                        // Fine, no problem.
                    }
                    _ => return Err(e),
                }
            }
        }
        DagaoStore::open(indexdir, hashstore)
    }

    pub fn open(indexdir: &Path, hashstore: HashStore) -> io::Result<DagaoStore> {
        match fs::read_dir(dir) {
            Err(e) => Err(e),
            Ok(_) => {
                Ok(DagaoStore {
                    indexdir: indexdir.to_owned(),
                    hstore: hashstore,
                })
            }
        }
    }

    pub fn open_leafnode_inserter(&self) -> io::Result<HashInserter> {
        self.hstore.open_inserter()
    }

    pub fn open_leafnode_reader(&self, hash: &Hash) -> io::Result<fs::File> {
        self.hstore.open_reader(hash)
    }

    pub fn open_linknode_inserter(&self) -> io::Result<LinkNodeInserter> {
        let hins = try!(self.hstore.open_inserter());
        Ok(LinkNodeInserter::init(hins))
    }

    pub fn open_linknode_reader(&self, hash: &Hash) -> io::Result<LinkNodeReader> {
        let f = try!(self.hstore.open_reader(hash));
        LinkNodeReader::wrap_file(f)
    }

    pub fn has_hash(&self, hash: &Hash) -> io::Result<bool> {
        use std::io::ErrorKind;

        match self.open_reader(hash) {
            Ok(_) => Ok(true),
            Err(ref e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }
}


pub struct LinkNodeInserter<'a> {
    hins: HashInserter<'a>,
}

impl<'a> LinkNodeInserter<'a> {
    fn init(hins: HashInserter<'a>) -> LinkNodeInserter<'a> {
        LinkNodeInserter { hins: hins }
    }
}


pub struct LinkNodeReader {
    f: fs::File,
}

impl LinkNodeReader {
    fn wrap_file(mut f: fs::file) -> io::Result<LinkNodeReader> {
        let header = "dagao 0\n".as_bytes();
        let mut buf = &[0u8; header.len()];
        let n = try!(f.read(&mut buf));
        if n == header.len() && header == buf {
            Ok(LinkNodeReader { f: f })
        } else {
            use std::io::{Error, ErrorKind};
            Err(Error::new(ErrorKind::InvalidData, "header unrecognized"))
        }
    }
}


#[cfg(test)]
mod tests {
    // FIXME
}
