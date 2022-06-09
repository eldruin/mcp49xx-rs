use crate::{marker, private, Error};

#[doc(hidden)]
pub trait BufferingSupport<CommE, PinE>: private::Sealed {
    fn check_buffering_is_appropriate(buffered: bool) -> Result<(), Error<CommE, PinE>>;
}

impl<CommE, PinE> BufferingSupport<CommE, PinE> for marker::Buffered {
    fn check_buffering_is_appropriate(_buffered: bool) -> Result<(), Error<CommE, PinE>> {
        Ok(())
    }
}

impl<CommE, PinE> BufferingSupport<CommE, PinE> for marker::Unbuffered {
    fn check_buffering_is_appropriate(buffered: bool) -> Result<(), Error<CommE, PinE>> {
        if buffered {
            Err(Error::BufferingNotSupported)
        } else {
            Ok(())
        }
    }
}
