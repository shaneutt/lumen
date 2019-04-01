use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

use num_bigint::{BigInt, Sign::*};

use crate::exception::Exception;
use crate::term::{Tag::BigInteger, Term};

pub struct Integer {
    #[allow(dead_code)]
    header: Term,
    pub inner: BigInt,
}

impl Integer {
    pub fn new(inner: BigInt) -> Self {
        Integer {
            header: Term {
                tagged: BigInteger as usize,
            },
            inner,
        }
    }
}

impl Eq for Integer {}

impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Integer) -> bool {
        self.inner == other.inner
    }
}

impl TryFrom<Integer> for usize {
    type Error = Exception;

    fn try_from(integer: Integer) -> Result<usize, Exception> {
        big_int_to_usize(&integer.inner)
    }
}

impl TryFrom<&Integer> for usize {
    type Error = Exception;

    fn try_from(integer_ref: &Integer) -> Result<usize, Exception> {
        big_int_to_usize(&integer_ref.inner)
    }
}

pub fn big_int_to_usize(big_int: &BigInt) -> Result<usize, Exception> {
    match big_int.sign() {
        Plus => {
            let (_, bytes) = big_int.to_bytes_be();
            let integer_usize = bytes
                .iter()
                .fold(0_usize, |acc, byte| (acc << 8) | (*byte as usize));

            Ok(integer_usize)
        }
        NoSign => Ok(0),
        Minus => Err(bad_argument!()),
    }
}
