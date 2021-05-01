use super::parse::ParseBenc;
use super::{BInt, BMap, BStr, BVec, Benc};

pub trait EncodeBencode<T: ParseBenc<T>> {
    fn as_bytes(&self) -> Vec<u8>;
}

impl EncodeBencode<BInt> for BInt {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push('i' as u8);
        bytes.append(&mut self.to_string().as_bytes().to_vec());
        bytes.push('e' as u8);
        bytes
    }
}

impl EncodeBencode<BStr> for BStr {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut self.len().to_string().into_bytes());
        bytes.push(':' as u8);
        bytes.append(&mut self.clone());
        bytes
    }
}

impl EncodeBencode<BVec> for BVec {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push('l' as u8);
        self.into_iter()
            .for_each(|e| bytes.append(&mut e.as_bytes()));
        bytes.push('e' as u8);
        bytes
    }
}

impl EncodeBencode<BMap> for BMap {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push('d' as u8);

        self.into_iter().for_each(|(k, v)| {
            bytes.append(&mut Benc::Str(k.clone().into_bytes()).as_bytes());
            bytes.append(&mut v.as_bytes());
        });
        bytes.push('e' as u8);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::super::Benc;
    use super::*;

    #[test]
    fn test_encode_bint() {
        assert_eq!(b"i10e".to_vec(), (10 as BInt).as_bytes());
        assert_eq!(b"i0e".to_vec(), (0 as BInt).as_bytes());
        assert_eq!(b"i12345e".to_vec(), (12345 as BInt).as_bytes());
    }

    #[test]
    fn test_encode_bstr() {
        assert_eq!(
            vec![51, 58, 1, 2, 3],
            (vec![1 as u8, 2, 3] as BStr).as_bytes()
        );
        assert_eq!(b"3:foo".to_vec(), (b"foo".to_vec() as BStr).as_bytes());
        assert_eq!(b"0:".to_vec(), (b"".to_vec() as BStr).as_bytes());
        assert_eq!(
            b"10:abcdefghij".to_vec(),
            (b"abcdefghij".to_vec() as BStr).as_bytes()
        );
    }

    #[test]
    fn test_encode_bvec() {
        assert_eq!(b"li10ee".to_vec(), (vec![Benc::Int(10)] as BVec).as_bytes());
        assert_eq!(
            b"li1ei2ei3ei4ei5ee".to_vec(),
            (vec![
                Benc::Int(1),
                Benc::Int(2),
                Benc::Int(3),
                Benc::Int(4),
                Benc::Int(5),
            ] as BVec)
                .as_bytes()
        );
        assert_eq!(
            b"li1e3:foo3:bari5ee".to_vec(),
            (vec![
                Benc::Int(1),
                Benc::Str(b"foo".to_vec()),
                Benc::Str(b"bar".to_vec()),
                Benc::Int(5),
            ] as BVec)
                .as_bytes()
        );
        assert_eq!(b"le".to_vec(), (vec!() as BVec).as_bytes());
    }

    #[test]
    fn test_encode_bmap() {
        let mut bmap = BMap::new();
        assert_eq!(b"de".to_vec(), bmap.as_bytes());

        bmap.insert("foo".to_string(), Benc::Int(10));
        assert_eq!(b"d3:fooi10ee".to_vec(), bmap.as_bytes());
        // note that it should be sorting so that bar is before foo, even though foo was inserted
        // first.
        bmap.insert("bar".to_string(), Benc::Int(20));
        assert_eq!(b"d3:bari20e3:fooi10ee".to_vec(), bmap.as_bytes());

        let mut sorted = BMap::new();
        sorted.insert("a".to_string(), Benc::Int(1));
        sorted.insert("b".to_string(), Benc::Int(2));
        assert_eq!(b"d1:ai1e1:bi2ee".to_vec(), sorted.as_bytes());
    }
}
