use std::{io, fs};
use std::path::{Path, PathBuf};
use hashstore::{Hash, HashInserter, HashStore};


pub struct DagaoStore {
    indexdir: PathBuf,
    hstore: HashStore,
}

impl DagaoStore {
    pub fn create_std(basedir: &Path) -> io::Result<DagaoStore> {
        try!(ensure_dir_exists(basedir));
        DagaoStore::create(basedir.join("index").as_path(),
                           try!(HashStore::create(basedir.join("store").as_path())))
    }

    pub fn create(indexdir: &Path, hashstore: HashStore) -> io::Result<DagaoStore> {
        try!(ensure_dir_exists(indexdir));
        DagaoStore::open(indexdir, hashstore)
    }

    pub fn open_std(basedir: &Path) -> io::Result<DagaoStore> {
        DagaoStore::open(basedir.join("index").as_path(),
                         try!(HashStore::open(basedir.join("store").as_path())))
    }

    pub fn open(indexdir: &Path, hashstore: HashStore) -> io::Result<DagaoStore> {
        match fs::read_dir(indexdir) {
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
        LinkNodeInserter::init(hins)
    }

    pub fn open_linknode_reader(&self, hash: &Hash) -> io::Result<LinkNodeReader> {
        let f = try!(self.hstore.open_reader(hash));
        LinkNodeReader::wrap_file(f)
    }

    pub fn has_hash(&self, hash: &Hash) -> io::Result<bool> {
        self.hstore.has_hash(hash)
    }
}


const LINK_NODE_HEADER: &'static [u8] = b"dagao 0\n";
const LINK_NODE_HEADER_LEN: usize = 8; // Is there a better way?


pub struct LinkNodeInserter<'a> {
    hins: HashInserter<'a>,
}

impl<'a> LinkNodeInserter<'a> {
    fn init(mut hins: HashInserter<'a>) -> io::Result<LinkNodeInserter<'a>> {
        use std::io::Write;

        let n = try!(hins.write(LINK_NODE_HEADER));
        if n == LINK_NODE_HEADER.len() {
            Ok(LinkNodeInserter { hins: hins })
        } else {
            // FIXME Handle 0 < n < LINK_NODE_HEADER.len().
            use std::io::{Error, ErrorKind};
            Err(Error::new(ErrorKind::WriteZero,
                           "could not write header in single write"))
        }
    }
}


pub struct LinkNodeReader {
    f: fs::File,
}

impl LinkNodeReader {
    fn wrap_file(mut f: fs::File) -> io::Result<LinkNodeReader> {
        use std::borrow::BorrowMut;
        use std::io::Read;

        let mut buf = [0u8; LINK_NODE_HEADER_LEN];
        let n = try!(f.read(buf.borrow_mut()));
        if n == LINK_NODE_HEADER.len() && buf == LINK_NODE_HEADER {
            Ok(LinkNodeReader { f: f })
        } else {
            use std::io::{Error, ErrorKind};
            Err(Error::new(ErrorKind::InvalidData, "header unrecognized"))
        }
    }
}


fn ensure_dir_exists(p: &Path) -> io::Result<()> {
    match fs::create_dir(p) {
        Ok(()) => { Ok(()) }
        Err(e) => {
            match e.kind() {
                io::ErrorKind::AlreadyExists => {
                    // Fine, no problem.
                    Ok(())
                }
                _ => Err(e),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    tests_with_fs! {
        create_new_dir |path: &Path| {
            use std::{fs, io};
            use DagaoStore;

            let exists_as_dir = |p: &Path| {
                match fs::metadata(p) {
                    Ok(md) => {
                        md.is_dir()
                    }
                    Err(e) => {
                        assert_eq!(e.kind(), io::ErrorKind::NotFound);
                        false
                    }
                }
            };

            assert!(!exists_as_dir(path));

            res_unwrap!(DagaoStore::create_std(path));

            assert!(exists_as_dir(path));
            assert!(exists_as_dir(path.join("index").as_path()));
            assert!(exists_as_dir(path.join("store").as_path()));
        };

        open_non_existent_dir |path: &Path| {
            use std::io;
            use hashstore::HashStore;

            let res = HashStore::open(path);

            assert!(res.is_err());
            assert!(res.err().unwrap().kind() == io::ErrorKind::NotFound);
        }

        /*
        insert_empty_linknode |path: &Path| {
            use std::fs;
            use std::io::Read;
            use hashstore::{HashStore, EMPTY_HASH};
            use dagaostore::{DagaoStore};

            let expectedhash = "";

            let ds = res_unwrap!(DagaoStore::create_std(path));

            let ins = res_unwrap!(ds.open_leafnode_inserter());
            let hash = res_unwrap!(ins.commit());
            assert_eq!(expectedhash, hash.encoded());

            let mut pb = path.to_path_buf();
            pb.push("nodes");
            pb.push(expectedhash);
            assert!(res_unwrap!(fs::metadata(pb)).is_file());

            let mut lnr = res_unwrap!(ds.open_linknode_reader(&hash));
            assert_eq!(None, res_unwrap!(lnr.next()));

            assert!(res_unwrap!(ds.has_hash(&hash)));
        }
        */
    }
}
