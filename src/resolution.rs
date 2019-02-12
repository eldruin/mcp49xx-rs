use {marker, private, Error};

#[doc(hidden)]
pub trait ResolutionSupport<E>: private::Sealed {
    fn is_value_appropriate(value: u16) -> Result<(), Error<E>>;
    fn get_value_for_spi(value: u16) -> [u8; 2];
}

impl<E> ResolutionSupport<E> for marker::Resolution12Bit {
    fn is_value_appropriate(value: u16) -> Result<(), Error<E>> {
        if value >= 1 << 12 {
            Err(Error::InvalidValue)
        } else {
            Ok(())
        }
    }
}
