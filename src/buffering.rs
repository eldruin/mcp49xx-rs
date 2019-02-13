use {marker, private, Error};

#[doc(hidden)]
pub trait BufferingSupport<E>: private::Sealed {
    fn check_buffering_is_appropriate(buffered: bool) -> Result<(), Error<E>>;
}

impl<E> BufferingSupport<E> for marker::Buffered {
    fn check_buffering_is_appropriate(_buffered: bool) -> Result<(), Error<E>> {
        Ok(())
    }
}
