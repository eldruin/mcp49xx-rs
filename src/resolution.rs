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
    fn get_value_for_spi(value: u16) -> [u8; 2] {
        [(value >> 8) as u8, (value & 0xff) as u8]
    }
}

impl<E> ResolutionSupport<E> for marker::Resolution10Bit {
    fn is_value_appropriate(value: u16) -> Result<(), Error<E>> {
        if value >= 1 << 10 {
            Err(Error::InvalidValue)
        } else {
            Ok(())
        }
    }
    fn get_value_for_spi(value: u16) -> [u8; 2] {
        [(value >> 6) as u8, ((value << 2) & 0xff) as u8]
    }
}
