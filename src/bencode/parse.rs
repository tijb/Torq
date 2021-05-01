use super::{BInt, BMap, BStr, BVec, Benc};

/// Implements the parsing of byte arrays into a Bencode Type (BInt, BMap, BStr, BVec)
pub trait ParseBenc<T> {
    /// attempts to parse and returns the result.
    fn parse(bytes: &[u8]) -> ParseResult<T>;
    /// attempts to parse, and returns a tuple containing the result, and the byte offset.
    fn parse_with_offset(bytes: &[u8]) -> ParseResult<(T, usize)>;
}

impl ParseBenc<BInt> for BInt {
    fn parse(bytes: &[u8]) -> ParseResult<Self> {
        match Self::parse_with_offset(bytes) {
            Ok((val, _)) => Ok(val),
            Err(e) => Err(e),
        }
    }
    fn parse_with_offset(bytes: &[u8]) -> ParseResult<(Self, usize)> {
        if bytes[0] != 'i' as u8 {
            return Err(ParseError::InvalidFirstByte);
        }
        match byte_index('e' as u8, bytes) {
            Some(e) => match parse_usize(&bytes[1..e]) {
                // +2 for 'i' and 'e'
                Ok(r) => Ok((r, bytes[1..e].len() + 2)),
                Err(e) => Err(e),
            },
            _ => Err(ParseError::MissingTerminator),
        }
    }
}

impl ParseBenc<BStr> for BStr {
    fn parse(bytes: &[u8]) -> ParseResult<Self> {
        match Self::parse_with_offset(bytes) {
            Ok((val, _)) => Ok(val),
            Err(e) => Err(e),
        }
    }
    fn parse_with_offset(bytes: &[u8]) -> ParseResult<(Self, usize)> {
        if !(48..58).contains(&bytes[0]) {
            return Err(ParseError::InvalidFirstByte);
        }
        let colon = match byte_index(':' as u8, bytes) {
            Some(colon) => colon,
            None => return Err(ParseError::InvalidByteString),
        };
        let len = match parse_usize(&bytes[0..colon]) {
            Ok(len) => len,
            Err(e) => return Err(e),
        };
        if colon + len >= bytes.len() {
            return Err(ParseError::ByteStringTooLong);
        }
        // +1 for the 'e'
        Ok((bytes[(colon + 1)..=(colon + len)].to_vec(), colon + len + 1))
    }
}

impl ParseBenc<BVec> for BVec {
    fn parse(bytes: &[u8]) -> ParseResult<Self> {
        match Self::parse_with_offset(bytes) {
            Ok((val, _)) => Ok(val),
            Err(e) => Err(e),
        }
    }
    fn parse_with_offset(bytes: &[u8]) -> ParseResult<(Self, usize)> {
        if bytes[0] != 'l' as u8 {
            return Err(ParseError::InvalidFirstByte);
        }
        // TODO: Revisit this and make it recursive/iterable without mut assignments.
        let mut i = 1;
        let mut vec = Vec::new();
        while bytes[i] != 'e' as u8 {
            match parse_any(&bytes[i..]) {
                Ok((val, offset)) => {
                    i += offset;
                    vec.push(val);
                }
                Err(e) => return Err(e),
            };
            // a value was successfully parsed, but it set us at the end of the byte array.
            // indicating that a list terminator (l...e) was missing.
            if i >= bytes.len() {
                return Err(ParseError::MissingTerminator);
            }
        }
        if bytes[bytes.len() - 1] != 'e' as u8 {
            return Err(ParseError::MissingTerminator);
        }
        // +1 for the 'e'
        Ok((vec, i + 1))
    }
}

impl ParseBenc<BMap> for BMap {
    fn parse(bytes: &[u8]) -> ParseResult<Self> {
        match Self::parse_with_offset(bytes) {
            Ok((val, _)) => Ok(val),
            Err(e) => Err(e),
        }
    }
    fn parse_with_offset(bytes: &[u8]) -> ParseResult<(Self, usize)> {
        if bytes[0] != 'd' as u8 {
            return Err(ParseError::InvalidFirstByte);
        }

        // TODO: Revisit this and make it recursive/iterable without mut assignments.
        let mut i = 1;
        let mut map: BMap = BMap::new();

        while bytes[i] != 'e' as u8 {
            let key = match BStr::parse_with_offset(&bytes[i..]) {
                Ok((val, offset)) => {
                    i += offset;
                    val
                }
                Err(e) => return Err(e),
            };
            if i >= bytes.len() {
                return Err(ParseError::InvalidBencodeDictionary);
            }
            let val = match parse_any(&bytes[i..]) {
                Ok((val, offset)) => {
                    i += offset;
                    val
                }
                Err(e) => return Err(e),
            };
            if i >= bytes.len() {
                return Err(ParseError::MissingTerminator);
            }
            map.insert(String::from_utf8(key).unwrap(), val);
        }
        // +1 for 'e'
        Ok((map, i + 1))
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingTerminator,
    InvalidFirstByte,
    InvalidAsciiInteger,
    InvalidUtf8String,
    InvalidByteString,
    ByteStringTooLong,
    InvalidBencodeDictionary,
}

/// Find the seek byte in the given bytes, or return None.
fn byte_index(seek: u8, bytes: &[u8]) -> Option<usize> {
    bytes
        .iter()
        .enumerate()
        .filter(|e| e.1 == &seek)
        .map(|e| e.0)
        .next()
}

/// Parse a byte slice into a String.
fn parse_string(bytes: &[u8]) -> ParseResult<String> {
    wrap_err(
        String::from_utf8(bytes.to_vec()),
        ParseError::InvalidUtf8String,
    )
}

/// Parse a byte slice into a usize. Assumes bytes are an ascii string of a valid usize value.
fn parse_usize(bytes: &[u8]) -> ParseResult<usize> {
    match parse_string(bytes) {
        Ok(s) => wrap_err(
            usize::from_str_radix(s.as_str(), 10),
            ParseError::InvalidAsciiInteger,
        ),
        Err(e) => Err(e),
    }
}

/// Rewrap a result of type Result<R, E> to Result<R, T>.
fn wrap_err<R, E, T>(res: Result<R, E>, err: T) -> Result<R, T> {
    match res {
        Ok(val) => Ok(val),
        _ => Err(err),
    }
}

/// Rewraps a ParseResult with a Bencode type into a result with a Benc instead so it can be
/// collected more easily.
#[macro_export]
macro_rules! rewrap_res {
    ($value:expr, $variant:ident) => {
        match $value {
            Ok((a, b)) => Ok((Benc::$variant(a), b)),
            Err(e) => Err(e),
        }
    };
}

// Attempts to parse the given bytes as any of the Bencode types, based on the ascii value of the
/// first byte.
pub fn parse_any(bytes: &[u8]) -> ParseResult<(Benc, usize)> {
    match bytes[0] {
        48..=57 => rewrap_res!(BStr::parse_with_offset(&bytes), Str),
        100 => rewrap_res!(BMap::parse_with_offset(&bytes), Map),
        105 => rewrap_res!(BInt::parse_with_offset(&bytes), Int),
        108 => rewrap_res!(BVec::parse_with_offset(&bytes), Vec),
        _ => Err(ParseError::InvalidFirstByte),
    }
}

#[cfg(test)]
mod tests {
    use crate::bencode::Benc;

    use super::{ParseError::*, *};

    #[test]
    fn bint_parse() {
        assert_eq!((10, 4), BInt::parse_with_offset(b"i10e").unwrap());
        assert_eq!((10, 6), BInt::parse_with_offset(b"i0010e").unwrap());
        assert_eq!((1234, 6), BInt::parse_with_offset(b"i1234e").unwrap());
        assert_eq!(
            InvalidFirstByte,
            BInt::parse_with_offset(b"10e").unwrap_err()
        );
        assert_eq!(
            MissingTerminator,
            BInt::parse_with_offset(b"i10").unwrap_err()
        );
    }

    #[test]
    fn bstr_parse() {
        assert_eq!(
            (b"foo".to_vec(), 5),
            BStr::parse_with_offset(b"3:foo").unwrap()
        );
        assert_eq!(
            ByteStringTooLong,
            BStr::parse_with_offset(b"4:foo").unwrap_err()
        );
        assert_eq!(
            InvalidFirstByte,
            BStr::parse_with_offset(b"abc:foo").unwrap_err()
        );
    }

    #[test]
    fn bvec_parse() {
        let (bvec, offset) = BVec::parse_with_offset(b"li10e3:fooe").unwrap();
        assert_eq!(11, offset);

        match bvec.get(0).unwrap() {
            Benc::Int(val) => assert_eq!(&10, val),
            _ => panic!("Something went wrong."),
        }
        match bvec.get(1).unwrap() {
            Benc::Str(val) => assert_eq!(&b"foo".to_vec(), val),
            _ => panic!("Something went wrong."),
        }
        assert_eq!(
            MissingTerminator,
            BVec::parse_with_offset(&b"li10e".to_vec()).unwrap_err()
        );
        assert_eq!(
            (vec![], 2),
            BVec::parse_with_offset(&b"le".to_vec()).unwrap()
        );
    }

    #[test]
    fn bmap_parse() {
        let (bmap, offset) = BMap::parse_with_offset(&b"d3:fooi10e3:bari999ee".to_vec()).unwrap();
        assert_eq!(21, offset);
        assert!(bmap.contains_key("foo"));
        assert_eq!(&Benc::Int(10), bmap.get("foo").unwrap());
        assert!(bmap.contains_key("bar"));
        assert_eq!(&Benc::Int(999), bmap.get("bar").unwrap());

        let (bmap, offset) = BMap::parse_with_offset(&b"de".to_vec()).unwrap();
        assert_eq!(2, offset);
        assert_eq!(0, bmap.len());

        assert_eq!(
            MissingTerminator,
            BMap::parse(&b"d3:fooi4e".to_vec()).unwrap_err()
        );
        assert_eq!(
            InvalidFirstByte,
            BMap::parse(&b"di10ei10e".to_vec()).unwrap_err()
        );
    }
}
