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
    ($create:ident, $resolution:ident, $channels:ident, $buffering:ident) => {
        pub fn $create(
            transactions: &[SpiTrans],
        ) -> Mcp49x<
            interface::SpiInterface<SpiMock, DummyOutputPin>,
            marker::$resolution,
            marker::$channels,
            marker::$buffering,
        > {
            Mcp49x::$create(SpiMock::new(&transactions), DummyOutputPin)
        }
    };
}

device_support!(new_mcp4921, Resolution12Bit, SingleChannel, Buffered);
device_support!(new_mcp4911, Resolution10Bit, SingleChannel, Buffered);
device_support!(new_mcp4901, Resolution8Bit, SingleChannel, Buffered);

device_support!(new_mcp4922, Resolution12Bit, DualChannel, Buffered);
device_support!(new_mcp4912, Resolution10Bit, DualChannel, Buffered);
device_support!(new_mcp4902, Resolution8Bit, DualChannel, Buffered);

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

macro_rules! for_all_ics {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_mcp4901, new_mcp4901);
            $name!(for_mcp4902, new_mcp4902);
            $name!(for_mcp4911, new_mcp4911);
            $name!(for_mcp4912, new_mcp4912);
            $name!(for_mcp4921, new_mcp4921);
            $name!(for_mcp4922, new_mcp4922);
        }
    };
}

macro_rules! for_all_single_channel_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4901, new_mcp4901 $(, $arg)*);
            $macroname!(for_mcp4911, new_mcp4911 $(, $arg)*);
            $macroname!(for_mcp4921, new_mcp4921 $(, $arg)*);
        }
    };
}

macro_rules! for_all_dual_channel_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4902, new_mcp4902 $(, $arg)*);
            $macroname!(for_mcp4912, new_mcp4912 $(, $arg)*);
            $macroname!(for_mcp4922, new_mcp4922 $(, $arg)*);
        }
    };
}

macro_rules! for_all_12bit_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4921, new_mcp4921 $(, $arg)*);
            $macroname!(for_mcp4922, new_mcp4922 $(, $arg)*);
        }
    };
}

macro_rules! for_all_10bit_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4911, new_mcp4911 $(, $arg)*);
            $macroname!(for_mcp4912, new_mcp4912 $(, $arg)*);
        }
    };
}

macro_rules! for_all_8bit_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4901, new_mcp4901 $(, $arg)*);
            $macroname!(for_mcp4902, new_mcp4902 $(, $arg)*);
        }
    };
}

macro_rules! common {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;

            test!(
                send_default,
                $create,
                Command::default(),
                0b0011_0000_0000_0000
            );

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
        }
    };
}

for_all_ics!(common);

macro_rules! buffered {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;

            test!(
                send_buffered,
                $create,
                Command::default().buffered(),
                0b0111_0000_0000_0000
            );
        }
    };
}

for_all_ics!(buffered);

macro_rules! send_value_test {
    ($name:ident, $create:ident, $value:expr, $expected_value:expr) => {
        mod $name {
            use super::*;
            test!(
                send_value,
                $create,
                Command::default().value($value),
                $expected_value
            );
        }
    };
}

for_all_12bit_ics!(
    send_value_12bit,
    send_value_test,
    0b0000_1010_1010_1011,
    0b0011_1010_1010_1011
);
for_all_10bit_ics!(
    send_value_10bit,
    send_value_test,
    0b0000_0010_1010_1011,
    0b0011_1010_1010_1100
);
for_all_8bit_ics!(
    send_value_8bit,
    send_value_test,
    0b0000_0000_1010_1011,
    0b0011_1010_1011_0000
);

macro_rules! invalid_value_test {
    ($name:ident, $create:ident, $too_big_value:expr) => {
        mod $name {
            use super::*;
            #[test]
            fn cannot_send_invalid_value() {
                let mut dev = $create(&[]);
                assert_error!(
                    dev.send(Command::default().value($too_big_value)),
                    InvalidValue
                );
                dev.destroy().0.done();
            }
        }
    };
}

for_all_12bit_ics!(invalid_value_12bit, invalid_value_test, 1 << 12);
for_all_10bit_ics!(invalid_value_10bit, invalid_value_test, 1 << 10);
for_all_8bit_ics!(invalid_value_8bit, invalid_value_test, 1 << 8);

macro_rules! invalid_channel_test {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;
            #[test]
            fn cannot_send_invalid_channel() {
                let mut dev = $create(&[]);
                assert_error!(
                    dev.send(Command::default().channel(Channel::Ch1)),
                    InvalidChannel
                );
                dev.destroy().0.done();
            }
        }
    };
}

for_all_single_channel_ics!(invalid_channel, invalid_channel_test);

macro_rules! send_channel1_test {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;
            test!(
                send_channel1,
                $create,
                Command::default().channel(Channel::Ch1),
                0b1011_0000_0000_0000
            );
        }
    };
}

for_all_dual_channel_ics!(send_channel1, send_channel1_test);
