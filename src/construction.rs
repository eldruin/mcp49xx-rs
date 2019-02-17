use core::marker::PhantomData;
use {interface, marker, Mcp49xx};

impl<SPI, CS, RES, CH, BUF> Mcp49xx<interface::SpiInterface<SPI, CS>, RES, CH, BUF> {
    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}

macro_rules! impl_create {
    ($dev:expr, $create:ident, $resolution:ident, $channels:ident, $buffering:ident) => {
        impl_create! {
            @gen [$create, $resolution, $channels, $buffering,
                concat!("Create a new instance of a ", $dev, " device.")]
        }
    };

    ( @gen [$create:ident, $resolution:ident, $channels:ident, $buffering:ident, $doc:expr] ) => {
        impl<SPI, CS>
            Mcp49xx<
                interface::SpiInterface<SPI, CS>,
                marker::$resolution,
                marker::$channels,
                marker::$buffering,
            >
        {
            #[doc = $doc]
            pub fn $create(spi: SPI, chip_select: CS) -> Self {
                Mcp49xx {
                    iface: interface::SpiInterface {
                        spi,
                        cs: chip_select,
                    },
                    _resolution: PhantomData,
                    _channels: PhantomData,
                    _buffering: PhantomData,
                }
            }
        }
    };
}

impl_create!(
    "MCP4801",
    new_mcp4801,
    Resolution8Bit,
    SingleChannel,
    Unbuffered
);
impl_create!(
    "MCP4802",
    new_mcp4802,
    Resolution8Bit,
    DualChannel,
    Unbuffered
);
impl_create!(
    "MCP4811",
    new_mcp4811,
    Resolution10Bit,
    SingleChannel,
    Unbuffered
);
impl_create!(
    "MCP4812",
    new_mcp4812,
    Resolution10Bit,
    DualChannel,
    Unbuffered
);
impl_create!(
    "MCP4821",
    new_mcp4821,
    Resolution12Bit,
    SingleChannel,
    Unbuffered
);
impl_create!(
    "MCP4822",
    new_mcp4822,
    Resolution12Bit,
    DualChannel,
    Unbuffered
);

impl_create!(
    "MCP4901",
    new_mcp4901,
    Resolution8Bit,
    SingleChannel,
    Buffered
);
impl_create!(
    "MCP4902",
    new_mcp4902,
    Resolution8Bit,
    DualChannel,
    Buffered
);
impl_create!(
    "MCP4911",
    new_mcp4911,
    Resolution10Bit,
    SingleChannel,
    Buffered
);
impl_create!(
    "MCP4912",
    new_mcp4912,
    Resolution10Bit,
    DualChannel,
    Buffered
);
impl_create!(
    "MCP4921",
    new_mcp4921,
    Resolution12Bit,
    SingleChannel,
    Buffered
);
impl_create!(
    "MCP4922",
    new_mcp4922,
    Resolution12Bit,
    DualChannel,
    Buffered
);
