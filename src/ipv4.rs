use std::{fmt, num::ParseIntError};


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Addr(pub [u8; 4]);

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b, c, d] = self.0;
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}


#[derive(Debug)]
pub enum ParseAddrError {
    NotEnoughParts,
    TooManyParts, // new!
    ParseIntError(ParseIntError),
}

impl fmt::Display for ParseAddrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ParseIntError> for ParseAddrError {
    fn from(e: ParseIntError) -> Self {
        ParseAddrError::ParseIntError(e)
    }
}

impl std::error::Error for ParseAddrError {}



impl Addr {
    pub fn parse<S>(s: S) -> Result<Self, ParseAddrError>
    where
        S: AsRef<str>,
    {
        let mut tokens = s.as_ref().split(".");

        let mut res = Self([0, 0, 0, 0]);
        for part in res.0.iter_mut() {
            // `part` is now a mutable reference to one of the
            // parts of `res.0`.
            // and remember, `Addr` is a newtype, it behaves like
            // a tuple that only has one element - that's why we
            // use `res.0` to operate on the `[u8; 4]` inside.
            *part = tokens
                .next()
                .ok_or(ParseAddrError::NotEnoughParts)?
                .parse()?
        }

        // we *should* be getting `None` here, because there
        // should only be four parts. If we get `Some`, there's
        // too many.
        if let Some(_) = tokens.next() {
            return Err(ParseAddrError::TooManyParts);
        }

        Ok(res)
    }
}
