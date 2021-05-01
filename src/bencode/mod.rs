use std::collections::BTreeMap;

use encode::EncodeBencode;
use parse::{parse_any, ParseResult};

mod encode;
mod parse;

pub type BInt = usize;
pub type BStr = Vec<u8>;
pub type BVec = Vec<Benc>;
pub type BMap = BTreeMap<String, Benc>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Benc {
    Int(BInt),
    Str(BStr),
    Vec(BVec),
    Map(BMap),
}

impl Benc {
    pub fn parse(bytes: &[u8]) -> ParseResult<Benc> {
        let res = parse_any(bytes);
        if let Ok((val, _)) = res {
            Ok(val)
        } else {
            Err(res.unwrap_err())
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Benc::Int(v) => (*v).as_bytes(),
            Benc::Str(v) => (*v).as_bytes(),
            Benc::Vec(v) => (*v).as_bytes(),
            Benc::Map(v) => (*v).as_bytes(),
        }
    }
}

impl Benc {
    pub fn to_usize(&self) -> usize {
        if let Benc::Int(v) = self {
            return v.clone();
        }
        panic!("bad call to to_usize()");
    }
    pub fn to_string(&self) -> String {
        if let Benc::Str(v) = self {
            return String::from_utf8(v.to_vec())
                .expect("This was parsed when it was written, this should never panic here.");
        }
        panic!("bad call to to_string()");
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        if let Benc::Str(v) = self {
            return v.to_vec();
        }
        panic!("bad call to to_bytes()");
    }
    pub fn to_vec(&self) -> Vec<Benc> {
        if let Benc::Vec(v) = self {
            return v.to_vec();
        }
        panic!("bad call to to_vec()");
    }
    pub fn to_map(&self) -> BTreeMap<String, Benc> {
        if let Benc::Map(v) = self {
            return v.clone();
        }
        panic!("bad call to to_map()");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_and_getting_values() {
        let bint: Benc = Benc::Int(10);
        assert_eq!(10, bint.to_usize());

        let bstr: Benc = Benc::Str(b"foo".to_vec());
        assert_eq!(b"foo".to_vec(), bstr.to_bytes());
    }

    #[test]
    #[should_panic]
    fn bad_value_unwrap_bint() {
        Benc::Int(10).to_string();
    }

    #[test]
    #[should_panic]
    fn bad_value_unwrap_bstr() {
        // even though the number is a valid ASCII number this still isn't valid
        Benc::Str(b"10".to_vec()).to_usize();
    }

    #[test]
    fn setting_and_getting_collections() {
        let bv1: Benc = Benc::Int(1);
        let bv2: Benc = Benc::Str(b"foo".to_vec());
        let bvec: Benc = Benc::Vec(vec![bv1, bv2]);
        let unwrapped_bvec = bvec.to_vec();
        assert_eq!(1, unwrapped_bvec.get(0).unwrap().to_usize());
        assert_eq!("foo", unwrapped_bvec.get(1).unwrap().to_string());

        let mut map: BTreeMap<String, Benc> = BTreeMap::new();
        map.insert("foo".to_string(), Benc::Int(9001));
        map.insert("bar".to_string(), Benc::Str(b"its over".to_vec()));

        let bmap: Benc = Benc::Map(map);
        let unwrapped_map = bmap.to_map();
        assert_eq!(9001, unwrapped_map.get("foo").unwrap().to_usize());
        assert_eq!("its over", unwrapped_map.get("bar").unwrap().to_string());
    }
}
