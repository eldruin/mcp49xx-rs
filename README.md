# Rust MCP49xx and MCP48xx digital-to-analog converter (DAC) driver

[![crates.io](https://img.shields.io/crates/v/mcp49xx.svg)](https://crates.io/crates/mcp49xx)
[![Docs](https://docs.rs/mcp49xx/badge.svg)](https://docs.rs/mcp49xx)
[![Build Status](https://github.com/eldruin/mcp49xx-rs/workflows/Build/badge.svg)](https://github.com/eldruin/mcp49xx-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/mcp49xx-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/mcp49xx-rs?branch=master)

This is a platform-agnostic Rust driver for the MCP49xx and MCP48xx SPI
digital-to-analog converters (DAC), based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Set a channel to a value.
- Shutdown a channel.
- Use buffering on commands.
- Select gain.

## The devices
The Microchip Technology Inc. MCP49xx devices are single/dual channel 8-bit,
10-bit and 12-bit buffered voltage output Digital-to-Analog Converters
(DACs). The devices operate from a single 2.7V to 5.5V supply with an SPI
compatible Serial Peripheral Interface. The user can configure the
full-scale range of the device to be Vref or 2*Vref by setting the gain
selection option bit (gain of 1 of 2).

The user can shut down the device by setting the Configuration Register bit.
In Shutdown mode, most of the internal circuits are turned off for power
savings, and the output amplifier is configured to present a known high
resistance output load (500 kΩ, typical).

The devices include double-buffered registers, allowing synchronous updates
of the DAC output using the LDAC pin. These devices also incorporate a
Power-on Reset (POR) circuit to ensure reliable power-up.

The devices utilize a resistive string architecture, with its inherent
advantages of low Differential Non-Linearity (DNL) error and fast settling
time. These devices are specified over the extended temperature range (+125°C).

The devices provide high accuracy and low noise performance for consumer
and industrial applications where calibration or compensation of signals
(such as temperature, pressure and humidity) are required.

This driver is compatible with these devices:

| Device  | Resolution | Channels | Buffering     |
|---------|------------|----------|---------------|
| MCP4801 | 8-bit      | 1        | Not available |
| MCP4802 | 8-bit      | 2        | Not available |
| MCP4811 | 10-bit     | 1        | Not available |
| MCP4812 | 10-bit     | 2        | Not available |
| MCP4821 | 12-bit     | 1        | Not available |
| MCP4822 | 12-bit     | 2        | Not available |
| MCP4901 | 8-bit      | 1        | Available     |
| MCP4902 | 8-bit      | 2        | Available     |
| MCP4911 | 10-bit     | 1        | Available     |
| MCP4912 | 10-bit     | 2        | Available     |
| MCP4921 | 12-bit     | 1        | Available     |
| MCP4922 | 12-bit     | 2        | Available     |

Datasheets:
- [MCP48x1](http://ww1.microchip.com/downloads/en/DeviceDoc/22244B.pdf)
- [MCP48x2](http://ww1.microchip.com/downloads/en/DeviceDoc/20002249B.pdf)
- [MCP49x1](http://ww1.microchip.com/downloads/en/DeviceDoc/22248a.pdf)
- [MCP49x2](http://ww1.microchip.com/downloads/en/DeviceDoc/22250A.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.
In the following examples an instance of the device MCP4921 will be created
as an example. Other devices can be created with similar methods like:
`Mcp49xx::new_mcp4822(...)`.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::{Pin, Spidev};
use mcp49xx::{Command, Mcp49xx};

fn main() {
    let spi = Spidev::open("/dev/spidev0.0").unwrap();
    let cs = Pin::new(25);
    let mut mcp4921 = Mcp49xx::new_mcp4921(cs);

    let cmd = Command::default();
    let cmd = cmd.double_gain().value(50);

    // enable double gain and set value
    mcp4921.send(&mut spi, cmd).unwrap();

    // keeps double gain enabled but changes value
    mcp4921.send(&mut spi, cmd.value(100)).unwrap();

    // Get CS pin back
    let _chip_select = mcp4921.destroy();
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/mcp49xx-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

