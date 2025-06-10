pub type Unsigned = usize;

pub trait Encoder {
    type Error;

    // Required method:

    /// Write literal bytes to the encoding
    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error>;

    // Provided methods:

    /// Write an item "head"
    fn head(&mut self, major: u8, arg: Unsigned) -> Result<(), Self::Error> {
        unimplemented!()
    }

    /// Write a CBOR Bytes value
    fn bytes(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.head(2, buf.len())?;
        self.write(buf)
    }

    fn array<T: Encode>(&mut self, items: &[T]) -> Result<(), Self::Error>
    {
        self.head(4, items.len())?;
        for item in items {
            item.encode(self)?;
        }
        Ok(())
    }

    /*
    fn array<T, I>(&mut self, items: T) -> Result<(), Self::Error>
    where
        T: IntoIterator,
        T::IntoIter: ExactSizeIterator,
        T::Item: Encode
    {
        let mut items = items.into_iter();
        let len = items.len();

        self.head(4, len)?;
        for _ in 0..len {
            let item = items.next().unwrap();
            item.encode(self)?;
        }

        if items.len() > 0 {
            panic!("ExactSizeIterator produced more elements than advertized");
        }

        Ok(())
    }
    */

}

impl Encoder for Vec<u8> {
    type Error = ();

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.extend(buf);
        Ok(())
    }
}

pub trait Encode {
    fn encode<E: Encoder + ?Sized>(&self, cbor: &mut E) -> Result<(), E::Error>;
}

impl Encode for [u8] {
    fn encode<E: Encoder + ?Sized>(&self, cbor: &mut E) -> Result<(), E::Error> {
        cbor.bytes(self)
    }
}

impl<T: Encode> Encode for [T] {
    fn encode<E: Encoder + ?Sized>(&self, cbor: &mut E) -> Result<(), E::Error> {
        cbor.array(self)
    }
}
