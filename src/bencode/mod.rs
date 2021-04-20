use std::collections::BTreeMap;

use encode::EncodeBencode;
use parse::{parse_any, ParseResult};

mod parse;
mod encode;

pub type BInt = usize;
pub type BStr = Vec<u8>;
pub type BVec = Vec<Benc>;
pub type BMap = BTreeMap<String, Benc>;

#[derive(Debug, Eq, PartialEq)]
pub enum Benc { Int(BInt), Str(BStr), Vec(BVec), Map(BMap) }

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

#[macro_export]
macro_rules! unwrap { ($benc:expr, $variant:ident) => {
    if let Benc::$variant(value) = $benc { value } else {
        panic!("unwrap macro called with invalid variant.")
    }
} }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting_and_getting_values() {
        let bint: Benc = Benc::Int(10);
        assert_eq!(10, unwrap!(bint, Int));

        let bstr: Benc = Benc::Str(b"foo".to_vec());
        assert_eq!(b"foo".to_vec(), unwrap!(bstr, Str));
    }

    #[test]
    #[should_panic]
    fn bad_value_unwrap_bint() {
        unwrap!(Benc::Int(10), Str);
    }

    #[test]
    #[should_panic]
    fn bad_value_unwrap_bstr() {
        // even though the number is a valid ASCII number this still isn't valid
        unwrap!(Benc::Str(b"10".to_vec()), Int);
    }

    #[test]
    fn setting_and_getting_collections() {
        let bv1: Benc = Benc::Int(1);
        let bv2: Benc = Benc::Str(b"foo".to_vec());
        let bvec: Benc = Benc::Vec(vec![bv1, bv2]);
        let unwrapped_bvec = unwrap!(bvec, Vec);
        assert_eq!(&1, unwrap!(unwrapped_bvec.get(0).unwrap(), Int));
        assert_eq!(&b"foo".to_vec(), unwrap!(unwrapped_bvec.get(1).unwrap(), Str));

        let mut map: BTreeMap<String, Benc> = BTreeMap::new();
        map.insert("foo".to_string(), Benc::Int(9001));
        map.insert("bar".to_string(), Benc::Str(b"its over".to_vec()));

        let bmap: Benc = Benc::Map(map);
        let unwrapped_map = unwrap!(bmap, Map);
        assert_eq!(&9001, unwrap!(unwrapped_map.get("foo").unwrap(), Int));
        assert_eq!(&b"its over".to_vec(), unwrap!(unwrapped_map.get("bar").unwrap(), Str));
    }
}
