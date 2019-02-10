//! SPI interface

#![deny(missing_docs)]

extern crate embedded_hal as hal;
use super::{private, Error};

/// SPI interface
#[derive(Debug, Default)]
pub struct SpiInterface<SPI, CS> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS,
}

/// Perform a command
pub trait WriteCommand: private::Sealed {
    /// Error type
    type Error;
    /// Command
    fn write_command(&mut self, command: u8, data: u8) -> Result<(), Error<Self::Error>>;
}

impl<SPI, CS, E> WriteCommand for SpiInterface<SPI, CS>
where
    SPI: hal::blocking::spi::Write<u8, Error = E>,
    CS: hal::digital::OutputPin,
{
    type Error = E;
    fn write_command(&mut self, command: u8, data: u8) -> Result<(), Error<E>> {
        self.cs.set_low();

        let payload: [u8; 2] = [command, data];
        let result = self.spi.write(&payload).map_err(Error::Comm);

        self.cs.set_high();
        result
    }
}
