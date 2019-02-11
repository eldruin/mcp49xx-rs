use {marker, private, Error};

#[doc(hidden)]
pub trait CheckValue<E>: private::Sealed {
    fn is_value_appropriate(value: u16) -> Result<(), Error<E>>;
}

impl<E> CheckValue<E> for marker::Resolution12Bit {
    fn is_value_appropriate(value: u16) -> Result<(), Error<E>> {
        if value >= 1 << 12 {
            Err(Error::InvalidValue)
        } else {
            Ok(())
        }
    }
}
