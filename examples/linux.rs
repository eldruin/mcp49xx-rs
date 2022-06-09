use linux_embedded_hal::{Pin, Spidev};
use mcp49xx::{Command, Mcp49xx};

fn main() {
    let spi = Spidev::open("/dev/spidev0.0").unwrap();
    let cs = Pin::new(25);
    let mut mcp4921 = Mcp49xx::new_mcp4921(spi, cs);

    let cmd = Command::default();
    let cmd = cmd.double_gain().value(50);

    // enable double gain and set value
    mcp4921.send(cmd).unwrap();

    // keeps double gain enabled but changes value
    mcp4921.send(cmd.value(100)).unwrap();

    // Get SPI device and CS pin back
    let (_spi, _chip_select) = mcp4921.destroy();
}
