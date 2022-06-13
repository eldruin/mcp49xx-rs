use linux_embedded_hal::{Pin, Spidev};
use mcp49xx::{Command, Mcp49xx};

fn main() {
    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
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
