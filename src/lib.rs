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
    // Wrong channel for this device provided
    // TODO WrongChannel,
    /// The value provided does not fit the bitness of the device
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
}

impl Default for Channel {
    fn default() -> Self {
        Channel::Ch0
    }
}

/// MCP49x digital potentiometer driver
#[derive(Debug, Default)]
pub struct Mcp49x<DI, RES> {
    iface: DI,
    _resolution: PhantomData<RES>,
}

/// Markers
pub mod marker {
    /// 12-Bit resolution device
    pub struct Resolution12Bit(());
}

#[doc(hidden)]
pub trait CheckValue<E>: private::Sealed {
    fn is_value_appropriate(value: u16) -> Result<(), Error<E>>;
}

impl<E> CheckValue<E> for marker::Resolution12Bit {
    fn is_value_appropriate(value: u16) -> Result<(), Error<E>> {
        if value >= 1 << 12 {
            Err(Error::InvalidValue)
        }
        else {
            Ok(())
        }
    }
}

impl<DI, RES, E> Mcp49x<DI, RES>
where
    DI: interface::WriteCommand<Error = E>,
    RES: CheckValue<E>
{
    /// Send command to device.
    pub fn send(&mut self, command: Command) -> Result<(), Error<E>> {
        RES::is_value_appropriate(command.value)?;
        let value_hi = (command.value >> 8) as u8;
        let data = [
            command.get_config_bits() | value_hi,
            (command.value & 0xff) as u8,
        ];
        self.iface.write_command(data[0], data[1])
    }
}

mod command;
mod construction;
#[doc(hidden)]
pub mod interface;
pub use command::Command;

mod private {
    use super::{interface, marker};
    pub trait Sealed {}

    impl<SPI, CS> Sealed for interface::SpiInterface<SPI, CS> {}
    impl Sealed for marker::Resolution12Bit {}
}
