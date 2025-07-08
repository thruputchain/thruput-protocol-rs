const _: () = {
    use crate::common_types::*;
    use ::xdrs::WriteXDR;
    use ::xdrs::serialize::writer::{Writer, Result};

    impl WriteXDR for VecM<u8> {
        fn write_xdr<F, W: Writer<F>>(&self, w: &mut W) -> Result<()> {
            self.0.write_xdr(w)
        }
    }

    impl WriteXDR for BytesM {
        fn write_xdr<F, W: Writer<F>>(&self, w: &mut W) -> Result<()> {
            self.0.write_xdr(w)
        }
    }

    /*
    impl<T> WriteXDR for VecM<T>
    where
        T: WriteXDR
    {
        fn write_xdr<F, W: Writer<F>>(&self, w: W) -> Result<()> {
            let slice: &[T] = self.as_ref();
            w.write_slice(slice)
        }
    }
    */
};

const _: () = {
    use crate::common_types::*;
    use ::xdrs::{ReadXDR, read_xdr};
    use ::xdrs::serialize::reader::{Reader, Result};

    use alloc::vec::Vec; // can use noalloc machinery in common_types to avoid this

    impl<'r> ReadXDR<'r> for BytesM {
        fn read_xdr<R: Reader<'r>>(r: &mut R) -> Result<Self> {
            let vec: Vec<u8> = read_xdr(r)?;
            Ok(Self(vec))
        }
    }

    impl<'r> ReadXDR<'r> for VecM<u8> {
        fn read_xdr<R: Reader<'r>>(r: &mut R) -> Result<Self> {
            let vec: Vec<u8> = read_xdr(r)?;
            Ok(Self(vec))
        }
    }

    /*
    impl<'r, T> ReadXDR<'r> for VecM<T>
    where
        T: ReadXDR<'r>
    {
        fn read_xdr<R: Reader<'r>>(r: &mut R) -> Result<Self> {
            let vec: Vec<T> = read_xdr(r)?;
            Ok(vec.into())
        }
    }
    */
};
