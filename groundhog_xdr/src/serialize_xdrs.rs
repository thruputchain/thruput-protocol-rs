const _: () = {
    use crate::common_types::*;
    use ::xdrs::WriteXDR;
    use ::xdrs::serialize::writer::{Writer, Result};

    impl<const MAX: u32> WriteXDR for BytesM<MAX> {
        fn write_xdr<F, W: Writer<F>>(&self, w: &mut W) -> Result<()> {
            self.0.write_xdr(w)
        }
    }

    impl<const MAX: u32, T> WriteXDR for VecM<T, MAX>
    where
        T: WriteXDR
    {
        fn write_xdr<F, W: Writer<F>>(&self, w: &mut W) -> Result<()> {
            let slice: &[T] = self.as_ref();
            slice.write_xdr(w)
        }
    }
};

#[cfg(feature = "alloc")]
const _: () = {
    use crate::common_types::*;
    use ::xdrs::{ReadXDR, read_xdr};
    use ::xdrs::serialize::reader::{Reader, Result, Error};

    use alloc::vec::Vec;

    impl<'r, const MAX: u32> ReadXDR<'r> for BytesM<MAX> {
        fn read_xdr<R: Reader<'r>>(r: &mut R) -> Result<Self> {
            let vec: Vec<u8> = read_xdr(r)?;
            if vec.len() > MAX as usize {
                return Err(Error::MaxLengthExceeded);
            }
            Ok(Self(vec))
        }
    }

    impl<'r, const MAX: u32, T> ReadXDR<'r> for VecM<T, MAX>
    where
        T: ReadXDR<'r>
    {
        fn read_xdr<R: Reader<'r>>(r: &mut R) -> Result<Self> {
            let vec: Vec<T> = read_xdr(r)?;
            if vec.len() > MAX as usize {
                return Err(Error::MaxLengthExceeded);
            }
            Ok(Self(vec))
        }
    }
};
