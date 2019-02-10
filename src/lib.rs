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

#![deny(unsafe_code)]
#![deny(missing_docs)]
// TODO #![deny(warnings)]
#![no_std]

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
pub struct Mcp49x<DI> {
    iface: DI,
}

impl<DI, E> Mcp49x<DI>
where
    DI: interface::WriteCommand<Error = E>,
{
    /// Send command to device.
    pub fn send(&mut self, command: Command) -> Result<(), Error<E>> {
        if command.value > 0x0fff {
            return Err(Error::InvalidValue);
        }
        let value_hi = (command.value >> 8) as u8;
        let data = [
            command.get_config_bits() | value_hi,
            (command.value & 0xff) as u8
        ];
        self.iface.write_command(data[0], data[1])
    }
}

impl<SPI, CS> Mcp49x<interface::SpiInterface<SPI, CS>> {
    /// Create new MCP4921 device instance
    pub fn new_mcp4921(spi: SPI, chip_select: CS) -> Self {
        Mcp49x {
            iface: interface::SpiInterface {
                spi,
                cs: chip_select,
            },
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy_mcp4921(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}

#[doc(hidden)]
pub mod interface;
mod command;
pub use command::Command;

mod private {
    use super::interface;
    pub trait Sealed {}

    impl<SPI, CS> Sealed for interface::SpiInterface<SPI, CS> {}
}

