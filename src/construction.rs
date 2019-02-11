use core::marker::PhantomData;
use {interface, marker, Mcp49x};

macro_rules! impl_create_destroy {
    ($dev:expr, $create:ident, $resolution:ident) => {
        impl_create_destroy! {
            @gen [$create, $resolution,
                concat!("Create a new instance of a ", $dev, " device.")]
        }
    };

    ( @gen [$create:ident, $resolution:ident, $doc:expr] ) => {
        impl<SPI, CS> Mcp49x<interface::SpiInterface<SPI, CS>, marker::$resolution> {
            #[doc = $doc]
            pub fn $create(spi: SPI, chip_select: CS) -> Self {
                Mcp49x {
                    iface: interface::SpiInterface {
                        spi,
                        cs: chip_select,
                    },
                    _resolution: PhantomData,
                }
            }

            /// Destroy driver instance, return SPI bus instance and CS output pin.
            pub fn destroy(self) -> (SPI, CS) {
                (self.iface.spi, self.iface.cs)
            }
        }
    };
}

impl_create_destroy!("MCP4921", new_mcp4921, Resolution12Bit);
