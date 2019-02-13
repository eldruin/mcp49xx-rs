//! This is a platform-agnostic Rust driver for the MCP49xx and MCP48xx SPI
//! digital-to-analog converters (DAC), based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! TODO
//!
//! ## The devices
//! The Microchip Technology Inc. MCP49xx devices are single/dual channel 8-bit,
//! 10-bit and 12-bit buffered voltage output Digital-to-Analog Converters
//! (DACs). The devices operate from a single 2.7V to 5.5V supply with an SPI
//! compatible Serial Peripheral Interface. The user can configure the
//! full-scale range of the device to be Vref or 2*Vref by setting the gain
//! selection option bit (gain of 1 of 2).
//!
//! The user can shut down the device by setting the Configuration Register bit.
//! In Shutdown mode, most of the internal circuits are turned off for power
//! savings, and the output amplifier is configured to present a known high
//! resistance output load (500 kΩ, typical).
//!
//! The devices include double-buffered registers, allowing synchronous updates
//! of the DAC output using the LDAC pin. These devices also incorporate a
//! Power-on Reset (POR) circuit to ensure reliable power-up.
//!
//! The devices utilize a resistive string architecture, with its inherent
//! advantages of low Differential Non-Linearity (DNL) error and fast settling
//! time. These devices are specified over the extended temperature range (+125°C).
//!
//! The devices provide high accuracy and low noise performance for consumer
//! and industrial applications where calibration or compensation of signals
//! (such as temperature, pressure and humidity) are required.
//!
//! This driver should be compatible at least with the devices:
//!
//! TODO compatible devices table with characteristics and datasheets: MCP48xx, MCP49xx
//!
//! ## Usage
//! TODO

#![deny(unsafe_code, missing_docs, warnings)]
#![no_std]

use core::marker::PhantomData;
extern crate embedded_hal as hal;
use hal::spi::{Mode, Phase, Polarity};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// Communication error
    Comm(E),
    /// The channel provided is not available in the current device
    InvalidChannel,
    /// The value provided does not fit the bitness of the current device
    InvalidValue,
}

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

/// Channel selector
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Channel {
    /// Channel 0
    Ch0,
    /// Channel 1 (only valid for dual devices. i.e. MCP4xx2)
    ///
    /// Sending a command on this channel to a single channel device will
    /// return an `Error::InvalidChannel`.
    Ch1,
}

impl Default for Channel {
    fn default() -> Self {
        Channel::Ch0
    }
}

/// MCP49x digital potentiometer driver
#[derive(Debug, Default)]
pub struct Mcp49x<DI, RES, CH, BUF> {
    iface: DI,
    _resolution: PhantomData<RES>,
    _channels: PhantomData<CH>,
    _buffering: PhantomData<BUF>,
}

/// Markers
pub mod marker {
    /// 12-Bit resolution device
    pub struct Resolution12Bit(());
    /// 10-Bit resolution device
    pub struct Resolution10Bit(());
    /// 8-Bit resolution device
    pub struct Resolution8Bit(());

    /// Single channel device
    pub struct SingleChannel(());
    /// Dual channel device
    pub struct DualChannel(());

    /// Device supports buffered commands
    pub struct Buffered(());
}

impl<DI, RES, CH, BUF, E> Mcp49x<DI, RES, CH, BUF>
where
    DI: interface::WriteCommand<Error = E>,
    RES: ResolutionSupport<E>,
    CH: ChannelSupport<E>,
    BUF: BufferingSupport<E>,
{
    /// Send command to device.
    ///
    /// This will return an error if the command is not appropriate for the current device:
    /// - If the channel is not available it will return `Error::InvalidChannel`.
    /// - If the value is too big it will return `Error::InvalidValue`.
    ///
    /// Otherwise if a communication error happened it will return `Error::Comm`.
    pub fn send(&mut self, command: Command) -> Result<(), Error<E>> {
        CH::check_channel_is_appropriate(command.channel)?;
        RES::check_value_is_appropriate(command.value)?;
        BUF::check_buffering_is_appropriate(command.buffered)?;
        let value = RES::get_value_for_spi(command.value);
        self.iface
            .write_command(command.get_config_bits() | value[0], value[1])
    }
}

mod command;
mod construction;
#[doc(hidden)]
pub mod interface;
mod resolution;
pub use command::Command;
#[doc(hidden)]
pub use resolution::ResolutionSupport;
mod channel;
#[doc(hidden)]
pub use channel::ChannelSupport;
mod buffering;
#[doc(hidden)]
pub use buffering::BufferingSupport;

mod private {
    use super::{interface, marker};
    pub trait Sealed {}

    impl<SPI, CS> Sealed for interface::SpiInterface<SPI, CS> {}

    impl Sealed for marker::Resolution12Bit {}
    impl Sealed for marker::Resolution10Bit {}
    impl Sealed for marker::Resolution8Bit {}

    impl Sealed for marker::SingleChannel {}
    impl Sealed for marker::DualChannel {}

    impl Sealed for marker::Buffered {}
}
