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

/// Configurable command that can be sent to the device
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Command {
    channel: Channel,
    buffered: bool,
    double_gain: bool,
    shutdown: bool,
    value: u16,
}

impl Default for Command {
    /// Create new command instance.
    ///
    /// Per default the command is on channel 0, unbuffered, with single gain,
    /// enabled and with value 0.
    fn default() -> Self {
        Command {
            channel: Channel::Ch0,
            buffered: false,
            double_gain: false,
            shutdown: false,
            value: 0
        }
    }
}

impl Command {
    /// Shutdown the channel
    pub fn shutdown(self) -> Self {
        let mut cmd = self;
        cmd.shutdown = true;
        cmd
    }

    /// Enable the channel (undo a shutdown)
    pub fn enable(self) -> Self {
        let mut cmd = self;
        cmd.shutdown = false;
        cmd
    }

    /// Send the value buffered
    pub fn buffered(self) -> Self {
        let mut cmd = self;
        cmd.buffered = true;
        cmd
    }

    /// Send the value unbuffered
    pub fn unbuffered(self) -> Self {
        let mut cmd = self;
        cmd.buffered = false;
        cmd
    }

    /// Send the value with double gain (2x)
    pub fn double_gain(self) -> Self {
        let mut cmd = self;
        cmd.double_gain = true;
        cmd
    }

    /// Send the value with single gain (1x)
    pub fn single_gain(self) -> Self {
        let mut cmd = self;
        cmd.double_gain = false;
        cmd
    }

    /// Set the value
    pub fn value(self, value: u16) -> Self {
        let mut cmd = self;
        cmd.value = value;
        cmd
    }

    // get the config bits at the beginning of the command
    fn get_config_bits(self) -> u8 {
        let mut value = 0b0011_0000;
        if self.channel != Channel::Ch0 {
            value |= 0b1000_0000;
        }
        if self.buffered {
            value |= 0b0100_0000;
        }
        if self.double_gain {
            value &= 0b1101_1111;
        }
        if self.shutdown {
            value &= 0b1110_1111;
        }
        value
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

mod private {
    use super::interface;
    pub trait Sealed {}

    impl<SPI, CS> Sealed for interface::SpiInterface<SPI, CS> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_cmd(cmd: Command, channel: Channel, buffered: bool, double_gain: bool, shutdown: bool, value:  u16) {
        assert_eq!(cmd.channel, channel);
        assert_eq!(cmd.buffered, buffered);
        assert_eq!(cmd.double_gain, double_gain);
        assert_eq!(cmd.shutdown, shutdown);
        assert_eq!(cmd.value, value);
    }

    #[test]
    fn default_command_all_off() {
        let cmd = Command::default();
        check_cmd(cmd, Channel::Ch0, false, false, false, 0);
    }

    #[test]
    fn can_set_double_gain() {
        let cmd = Command::default().double_gain();
        check_cmd(cmd, Channel::Ch0, false, true, false, 0);
    }

    #[test]
    fn can_set_single_gain() {
        let cmd = Command::default().double_gain().single_gain();
        check_cmd(cmd, Channel::Ch0, false, false, false, 0);
    }

    #[test]
    fn can_set_shutdown() {
        let cmd = Command::default().shutdown();
        check_cmd(cmd, Channel::Ch0, false, false, true, 0);
    }

    #[test]
    fn can_set_enable() {
        let cmd = Command::default().shutdown().enable();
        check_cmd(cmd, Channel::Ch0, false, false, false, 0);
    }

    #[test]
    fn can_set_buffered() {
        let cmd = Command::default().buffered();
        check_cmd(cmd, Channel::Ch0, true, false, false, 0);
    }

    #[test]
    fn can_set_unbuffered() {
        let cmd = Command::default().buffered().unbuffered();
        check_cmd(cmd, Channel::Ch0, false, false, false, 0);
    }

    #[test]
    fn can_set_value() {
        let cmd = Command::default().value(1024);
        check_cmd(cmd, Channel::Ch0, false, false, false, 1024);
    }

    #[test]
    fn operations_leave_original_command_unchanged() {
        let original = Command::default();
        check_cmd(original.shutdown(), Channel::Ch0, false, false, true, 0);
        check_cmd(original, Channel::Ch0, false, false, false, 0);
    }
}
