use {marker, private, Channel, Error};

#[doc(hidden)]
pub trait ChannelSupport<E>: private::Sealed {
    fn check_channel_is_appropriate(channel: Channel) -> Result<(), Error<E>>;
}

impl<E> ChannelSupport<E> for marker::SingleChannel {
    fn check_channel_is_appropriate(channel: Channel) -> Result<(), Error<E>> {
        if channel != Channel::Ch0 {
            Err(Error::InvalidChannel)
        } else {
            Ok(())
        }
    }
}
