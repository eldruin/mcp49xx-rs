//! This is a platform-agnostic Rust driver for the MCP49xx and MCP48xx SPI
//! digital-to-analog converters (DAC), based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Set a channel to a value.
//! - Shutdown a channel.
//! - Use buffering on commands.
//! - Select gain.
//!
//! ## The devices
//! The Microchip Technology Inc. MCP49xx and MCP48xx devices are single/dual
//! channel 8-bit, 10-bit and 12-bit buffered voltage output Digital-to-Analog
//! Converters (DACs). The devices operate from a single 2.7V to 5.5V supply
//! with an SPI compatible Serial Peripheral Interface. The user can configure
//! the full-scale range of the device to be Vref or 2*Vref by setting the gain
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
//! This driver is compatible with these devices:
//!
//! | Device  | Resolution | Channels | Buffering     |
//! |---------|------------|----------|---------------|
//! | MCP4801 | 8-bit      | 1        | Not supported |
//! | MCP4802 | 8-bit      | 2        | Not supported |
//! | MCP4811 | 10-bit     | 1        | Not supported |
//! | MCP4812 | 10-bit     | 2        | Not supported |
//! | MCP4821 | 12-bit     | 1        | Not supported |
//! | MCP4822 | 12-bit     | 2        | Not supported |
//! | MCP4901 | 8-bit      | 1        | Supported     |
//! | MCP4902 | 8-bit      | 2        | Supported     |
//! | MCP4911 | 10-bit     | 1        | Supported     |
//! | MCP4912 | 10-bit     | 2        | Supported     |
//! | MCP4921 | 12-bit     | 1        | Supported     |
//! | MCP4922 | 12-bit     | 2        | Supported     |
//!
//! Datasheets:
//! - [MCP48x1](http://ww1.microchip.com/downloads/en/DeviceDoc/22244B.pdf)
//! - [MCP48x2](http://ww1.microchip.com/downloads/en/DeviceDoc/20002249B.pdf)
//! - [MCP49x1](http://ww1.microchip.com/downloads/en/DeviceDoc/22248a.pdf)
//! - [MCP49x2](http://ww1.microchip.com/downloads/en/DeviceDoc/22250A.pdf)
//!
//! ## The interface
//!
//! These devices support changing all configuration flags in each command
//! sent. In order to keep this flexibility, this driver does not provide
//! individual methods to set the settings but provides a `Command` struct
//! which can be used to specify all settings.
//! Then commands can be sent to the device through the `send()` method.
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//! In the following examples an instance of the device MCP4921 will be created
//! as an example. Other devices can be created with similar methods like:
//! `Mcp49xx::new_mcp4822(...)`.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Set channel 0 to position 1024 in a MCP4921 device
//!
//! ```no_run
//! extern crate embedded_hal;
//! extern crate linux_embedded_hal;
//! extern crate mcp49xx;
//! use mcp49xx::{Channel, Command, Mcp49xx};
//! use linux_embedded_hal::{Pin, Spidev};
//!
//! # fn main() {
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = Pin::new(25);
//!
//! let mut dac = Mcp49xx::new_mcp4921(spi, chip_select);
//!
//! let cmd = Command::default();
//! let cmd = cmd.channel(Channel::Ch0).value(1024);
//! dac.send(cmd).unwrap();
//!
//! // Get SPI device and CS pin back
//! let (_spi, _chip_select) = dac.destroy();
//! # }
//! ```
//!
//! ### Set position and shutdown channels in a MCP4822 device
//!
//! ```no_run
//! extern crate embedded_hal;
//! extern crate linux_embedded_hal;
//! extern crate mcp49xx;
//! use mcp49xx::{Channel, Command, Mcp49xx};
//! use linux_embedded_hal::{Pin, Spidev};
//!
//! # fn main() {
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = Pin::new(25);
//!
//! let mut dac = Mcp49xx::new_mcp4822(spi, chip_select);
//!
//! let cmd = Command::default();
//! let cmd = cmd.channel(Channel::Ch1).value(1024);
//! dac.send(cmd).unwrap();
//!
//! let cmd = Command::default();
//! let cmd = cmd.channel(Channel::Ch0).shutdown();
//! dac.send(cmd).unwrap();
//!
//! // Get SPI device and CS pin back
//! let (_spi, _chip_select) = dac.destroy();
//! # }
//! ```
//!
//! ### Set position and activate buffering and double gain in a MCP4911 device
//!
//! ```no_run
//! extern crate embedded_hal;
//! extern crate linux_embedded_hal;
//! extern crate mcp49xx;
//! use mcp49xx::{Channel, Command, Mcp49xx};
//! use linux_embedded_hal::{Pin, Spidev};
//!
//! # fn main() {
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let chip_select = Pin::new(25);
//!
//! let mut dac = Mcp49xx::new_mcp4911(spi, chip_select);
//!
//! let cmd = Command::default();
//! let cmd = cmd.channel(Channel::Ch0).buffered().double_gain().value(511);
//! dac.send(cmd).unwrap();
//!
//! // Get SPI device and CS pin back
//! let (_spi, _chip_select) = dac.destroy();
//! # }
//! ```

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
    /// The channel provided is not available in the current device (MCP4xx1)
    InvalidChannel,
    /// The value provided does not fit the bitness of the current device
    InvalidValue,
    /// Buffering is not available in the current device (MCP48xx)
    BufferingNotSupported,
}

/// SPI mode (CPOL = 0, CPHA = 0)
pub const MODE0: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

/// SPI mode (CPOL = 1, CPHA = 1)
pub const MODE1: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
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

/// MCP49xx digital potentiometer driver
#[derive(Debug, Default)]
pub struct Mcp49xx<DI, RES, CH, BUF> {
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
    /// Device does not support buffered commands
    pub struct Unbuffered(());
}

impl<DI, RES, CH, BUF, E> Mcp49xx<DI, RES, CH, BUF>
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
    /// - If buffering is not supported it will return `Error::BufferingNotSupported`.
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
    impl Sealed for marker::Unbuffered {}
}
