use crate::{marker, private, Channel, Error};

#[doc(hidden)]
pub trait ChannelSupport<CommE, PinE>: private::Sealed {
    fn check_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE, PinE>>;
}

impl<CommE, PinE> ChannelSupport<CommE, PinE> for marker::SingleChannel {
    fn check_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE, PinE>> {
        if channel != Channel::Ch0 {
            Err(Error::InvalidChannel)
        } else {
            Ok(())
        }
    }
}

impl<CommE, PinE> ChannelSupport<CommE, PinE> for marker::DualChannel {
    fn check_channel_is_appropriate(_channel: Channel) -> Result<(), Error<CommE, PinE>> {
        Ok(())
    }
}
