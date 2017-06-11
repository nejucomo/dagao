use std::io;
use std::io::Read;
use hashstore::{HASH_BYTES, Hash};


#[derive(PartialEq, Debug)]
pub struct Reference {
    pub reftype: RefType,
    pub hash: Hash,
}


const REFERENCE_BYTES: usize = HASH_BYTES + 1;


impl Reference {
    pub fn read_from_file<R: Read>(f: &mut R) -> io::Result<Option<Reference>> {
        use std::borrow::BorrowMut;
        use ioutil::read_full;
        use hashstore::{HASH_BYTES, Hash};

        let mut buf = [0u8; REFERENCE_BYTES];
        if try!(read_full(f, buf.borrow_mut())) {
            let reftype = try!(RefType::decode_byte(buf[0]));
            let mut hbuf = [0u8; HASH_BYTES];

            // BUG: Can we remove this copy?
            for i in 0..hbuf.len() {
                hbuf[i] = buf[i + 1];
            }

            let hash = Hash::wrap_bytes(hbuf);

            Ok(Some(Reference {
                reftype: reftype,
                hash: hash,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn decode(s: &str) -> Result<Reference, ReferenceDecodeError> {
        use hashstore::b64::{FromB64, FromBase64Error};

        let encoded_chars = ((REFERENCE_BYTES as f64) * 4f64 / 3f64).ceil() as usize;

        if s.len() == encoded_chars {
            match s.from_b64() {
                Ok(bvec) => {
                    assert_eq!(REFERENCE_BYTES, bvec.len());
                    let rt = try!(RefType::decode_byte(bvec[0]));
                    let mut hbuf = [0; HASH_BYTES];
                    for i in 0..HASH_BYTES {
                        hbuf[i] = bvec[i + 1];
                    }
                    let hash = Hash::wrap_bytes(hbuf);
                    Ok(Reference {
                        reftype: rt,
                        hash: hash,
                    })
                }
                Err(FromBase64Error::InvalidBase64Byte(b, i)) => {
                    Err(ReferenceDecodeError::InvalidBase64Byte(b, i))
                }
                Err(e) => {
                    unreachable!(
                        "Length precondition check inconsistency; error {:?}; input {:?}",
                        e,
                        s,
                    )
                }
            }
        } else {
            Err(ReferenceDecodeError::InvalidLength(s.len()))
        }
    }

    pub fn encoded(&self) -> String {
        use hashstore::b64::ToB64;

        let mut buf = [0; REFERENCE_BYTES];
        buf[0] = self.reftype.to_byte();
        let hbuf = self.hash.peek_bytes();
        for i in 0..HASH_BYTES {
            buf[i + 1] = hbuf[i];
        }
        buf.to_b64()
    }
}


#[derive(PartialEq, Debug)]
pub enum RefType {
    Data,
    Link,
}


impl RefType {
    fn decode_byte(b: u8) -> Result<RefType, ReferenceDecodeError> {
        match b {
            0 => Ok(RefType::Data),
            1 => Ok(RefType::Link),
            _ => Err(ReferenceDecodeError::InvalidRefType(b)),
        }
    }

    fn to_byte(&self) -> u8 {
        match *self {
            RefType::Data => 0,
            RefType::Link => 1,
        }
    }
}


#[derive(Debug)]
pub enum ReferenceDecodeError {
    InvalidRefType(u8),
    InvalidLength(usize),
    InvalidBase64Byte(u8, usize),
}


impl From<ReferenceDecodeError> for io::Error {
    fn from(rde: ReferenceDecodeError) -> Self {
        use std::io::ErrorKind;
        use self::ReferenceDecodeError as RDE;

        let errmsg =
            format!("invalid Reference: bad {}",
                    match rde {
                        RDE::InvalidRefType(b) => format!("RefType byte {}", b),
                        RDE::InvalidLength(l) => format!("length {}, expected {}", l, l),
                        RDE::InvalidBase64Byte(b, i) => format!("base64 char {} at index {}", b, i),
                    });

        io::Error::new(ErrorKind::InvalidData, errmsg)
    }
}
