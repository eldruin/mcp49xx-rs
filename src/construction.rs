use core::marker::PhantomData;
use {interface, marker, Mcp49x};

impl<SPI, CS> Mcp49x<interface::SpiInterface<SPI, CS>, marker::Resolution12Bit> {
    /// Create new MCP4921 device instance
    pub fn new_mcp4921(spi: SPI, chip_select: CS) -> Self {
        Mcp49x {
            iface: interface::SpiInterface {
                spi,
                cs: chip_select,
            },
            _resolution: PhantomData,
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy_mcp4921(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}
