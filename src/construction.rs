use core::marker::PhantomData;
use {interface, marker, Mcp49x};

impl<SPI, CS, RES, CH> Mcp49x<interface::SpiInterface<SPI, CS>, RES, CH> {
    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}

macro_rules! impl_create_destroy {
    ($dev:expr, $create:ident, $resolution:ident, $channels:ident) => {
        impl_create_destroy! {
            @gen [$create, $resolution, $channels,
                concat!("Create a new instance of a ", $dev, " device.")]
        }
    };

    ( @gen [$create:ident, $resolution:ident, $channels:ident, $doc:expr] ) => {
        impl<SPI, CS>
            Mcp49x<interface::SpiInterface<SPI, CS>, marker::$resolution, marker::$channels>
        {
            #[doc = $doc]
            pub fn $create(spi: SPI, chip_select: CS) -> Self {
                Mcp49x {
                    iface: interface::SpiInterface {
                        spi,
                        cs: chip_select,
                    },
                    _resolution: PhantomData,
                    _channels: PhantomData,
                }
            }
        }
    };
}

impl_create_destroy!("MCP4901", new_mcp4901, Resolution8Bit, SingleChannel);
impl_create_destroy!("MCP4902", new_mcp4902, Resolution8Bit, DualChannel);
impl_create_destroy!("MCP4911", new_mcp4911, Resolution10Bit, SingleChannel);
impl_create_destroy!("MCP4912", new_mcp4912, Resolution10Bit, DualChannel);
impl_create_destroy!("MCP4921", new_mcp4921, Resolution12Bit, SingleChannel);
impl_create_destroy!("MCP4922", new_mcp4922, Resolution12Bit, DualChannel);
