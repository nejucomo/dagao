use std::{io, fs};
use std::path::{Path, PathBuf};
use hashstore::HashStore;
use {Reference, RefType};
use datanode;
use linknode;


pub struct DagaoStore {
    #[allow(dead_code)]
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

    pub fn open_datanode_inserter(&self) -> io::Result<datanode::Inserter> {
        let hins = try!(self.hstore.open_inserter());
        datanode::Inserter::wrap_hash_inserter(hins)
    }

    pub fn open_datanode_reader(&self, r: &Reference) -> io::Result<datanode::FileReader> {
        try!(require_reftype(RefType::Data, r));
        datanode::Reader::wrap_read(try!(self.hstore.open_reader(&r.hash)))
    }

    pub fn open_linknode_inserter(&self) -> io::Result<linknode::Inserter> {
        let hins = try!(self.hstore.open_inserter());
        linknode::Inserter::wrap_hash_inserter(hins)
    }

    pub fn open_linknode_reader(&self, r: &Reference) -> io::Result<linknode::FileReader> {
        try!(require_reftype(RefType::Link, r));
        let f = try!(self.hstore.open_reader(&r.hash));
        linknode::Reader::wrap_read(f)
    }

    pub fn has_ref(&self, r: &Reference) -> io::Result<bool> {
        self.hstore.has_hash(&r.hash)
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


fn require_reftype(rt: RefType, r: &Reference) -> io::Result<()> {
    if r.reftype == rt {
        Ok(())
    } else {
        use std::io::{Error, ErrorKind};

        Err(Error::new(ErrorKind::InvalidInput,
                       format!("Expected {:?} reference, found {:?}.", rt, r)))
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
        };

        insert_empty_datanode |path: &Path| {
            use std::fs;
            use std::io::Read;
            use hashstore::{EMPTY_HASH};
            use {DagaoStore, RefType};

            let ds = res_unwrap!(DagaoStore::create_std(path));
            let ins = res_unwrap!(ds.open_datanode_inserter());
            let href = res_unwrap!(ins.commit());
            println!("href {:?}, EMPTY_HASH {:?}", href, EMPTY_HASH);
            assert_eq!(RefType::Data, href.reftype);
            assert_eq!(EMPTY_HASH, href.hash.encoded());

            let mut pb = path.to_path_buf();
            pb.push("store");
            pb.push(EMPTY_HASH);
            println!("pb {:?}", pb);
            assert!(res_unwrap!(fs::metadata(pb)).is_file());

            let mut f = res_unwrap!(ds.open_datanode_reader(&href));
            let mut contents = String::new();
            let readlen = res_unwrap!(f.read_to_string(&mut contents));
            assert_eq!(0, readlen);
            assert_eq!("", contents);

            assert!(res_unwrap!(ds.has_ref(&href)));
        };

        insert_empty_linknode |path: &Path| {
            use std::fs;
            use {DagaoStore, RefType};

            const EMPTY_LINKNODE: &'static str = "PJJ13v6y0SzJ88yEvzH5ng7qAYURX3omv4NFJYG35fQ";

            let ds = res_unwrap!(DagaoStore::create_std(path));
            let ins = res_unwrap!(ds.open_linknode_inserter());
            let href = res_unwrap!(ins.commit());
            println!("href {:?}", href);
            assert_eq!(RefType::Link, href.reftype);
            assert_eq!(EMPTY_LINKNODE, href.hash.encoded());

            let mut pb = path.to_path_buf();
            pb.push("store");
            pb.push(EMPTY_LINKNODE);
            println!("pb {:?}", pb);
            assert!(res_unwrap!(fs::metadata(pb)).is_file());

            let mut lnr = res_unwrap!(ds.open_linknode_reader(&href));
            let link = res_unwrap!(lnr.read_next());
            assert_eq!(None, link);

            assert!(res_unwrap!(ds.has_ref(&href)));
        }
    }
}
