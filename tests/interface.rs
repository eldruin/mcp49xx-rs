extern crate mcp49x;
use mcp49x::{interface, marker, Channel, Command, Error, Mcp49x};
extern crate embedded_hal_mock as hal;
use self::hal::spi::{Mock as SpiMock, Transaction as SpiTrans};

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) {}
    fn set_high(&mut self) {}
}

macro_rules! device_support {
    ($create:ident, $resolution:ident, $channels:ident) => {
        pub fn $create(
            transactions: &[SpiTrans],
        ) -> Mcp49x<
            interface::SpiInterface<SpiMock, DummyOutputPin>,
            marker::$resolution,
            marker::$channels,
        > {
            Mcp49x::$create(SpiMock::new(&transactions), DummyOutputPin)
        }
    };
}

device_support!(new_mcp4921, Resolution12Bit, SingleChannel);
device_support!(new_mcp4911, Resolution10Bit, SingleChannel);
device_support!(new_mcp4901, Resolution8Bit, SingleChannel);

#[macro_export]
macro_rules! test {
    ($name:ident, $create:ident, $cmd:expr, $value:expr ) => {
        #[test]
        fn $name() {
            let trans = [SpiTrans::write(vec![
                ($value >> 8) as u8,
                ($value & 0xff) as u8,
            ])];
            let mut dev = $create(&trans);
            dev.send($cmd).unwrap();
            dev.destroy().0.done();
        }
    };
}

macro_rules! assert_error {
    ($result:expr, $error_variant:ident) => {
        match $result {
            Err(Error::$error_variant) => (),
            _ => panic!("Error not reported."),
        }
    };
}

#[test]
fn matches() {
    let result: Result<(), Error<()>> = Err(Error::InvalidValue);
    assert_error!(result, InvalidValue);
}

#[should_panic]
#[test]
fn can_fail() {
    let result: Result<(), Error<()>> = Ok(());
    assert_error!(result, InvalidValue);
}

macro_rules! ic_test {
    ($ic:ident, $create:ident, $value:expr, $expected_value:expr, $too_big_value:expr) => {
        mod $ic {
            use super::*;

            test!(
                send_default,
                $create,
                Command::default(),
                0b0011_0000_0000_0000
            );

            test!(
                send_value,
                $create,
                Command::default().value($value),
                $expected_value
            );

            #[test]
            fn cannot_send_invalid_value() {
                let mut dev = $create(&[]);
                assert_error!(
                    dev.send(Command::default().value($too_big_value)),
                    InvalidValue
                );
                dev.destroy().0.done();
            }

            #[test]
            fn cannot_send_invalid_channel() {
                let mut dev = $create(&[]);
                assert_error!(
                    dev.send(Command::default().channel(Channel::Ch1)),
                    InvalidChannel
                );
                dev.destroy().0.done();
            }

            test!(
                send_shutdown,
                $create,
                Command::default().shutdown(),
                0b0010_0000_0000_0000
            );

            test!(
                send_double_gain,
                $create,
                Command::default().double_gain(),
                0b0001_0000_0000_0000
            );

            test!(
                send_buffered,
                $create,
                Command::default().buffered(),
                0b0111_0000_0000_0000
            );
        }
    };
}

ic_test!(
    mcp4921,
    new_mcp4921,
    0b0000_1010_1010_1010,
    0b0011_1010_1010_1010,
    1 << 12
);

ic_test!(
    mcp4911,
    new_mcp4911,
    0b0000_0010_1010_1011,
    0b0011_1010_1010_1100,
    1 << 10
);

ic_test!(
    mcp4901,
    new_mcp4901,
    0b0000_0000_1010_1011,
    0b0011_1010_1011_0000,
    1 << 9
);
