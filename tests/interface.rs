extern crate mcp49x;
use mcp49x::{interface, marker, Command, Error, Mcp49x};
extern crate embedded_hal_mock as hal;
use self::hal::spi::{Mock as SpiMock, Transaction as SpiTrans};

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) {}
    fn set_high(&mut self) {}
}

macro_rules! device_support {
    ($create:ident, $destroy:ident, $resolution:ident) => {
        pub fn $create(
            transactions: &[SpiTrans],
        ) -> Mcp49x<interface::SpiInterface<SpiMock, DummyOutputPin>, marker::$resolution> {
            Mcp49x::$create(SpiMock::new(&transactions), DummyOutputPin)
        }

        pub fn $destroy(
            dev: Mcp49x<interface::SpiInterface<SpiMock, DummyOutputPin>, marker::$resolution>,
        ) {
            dev.$destroy().0.done();
        }
    };
}

device_support!(new_mcp4921, destroy_mcp4921, Resolution12Bit);

#[macro_export]
macro_rules! test {
    ($name:ident, $create:ident, $destroy:ident, $cmd:expr, $value:expr ) => {
        #[test]
        fn $name() {
            let trans = [SpiTrans::write(vec![
                ($value >> 8) as u8,
                ($value & 0xff) as u8,
            ])];
            let mut dev = $create(&trans);
            dev.send($cmd).unwrap();
            $destroy(dev);
        }
    };
}

fn assert_invalid_value<T, E>(result: &Result<T, Error<E>>) {
    match result {
        Err(Error::InvalidValue) => (),
        _ => panic!("Invalid value not reported."),
    }
}

#[test]
fn invalid_value_matches() {
    assert_invalid_value::<(), ()>(&Err(Error::InvalidValue));
}

#[should_panic]
#[test]
fn invalid_value_can_fail() {
    assert_invalid_value::<(), ()>(&Ok(()));
}

macro_rules! ic_test {
    ($ic:ident, $create:ident, $destroy:ident, $value:expr,
     $expected_value:expr, $too_big_value:expr) => {
        mod $ic {
            use super::*;

            test!(
                send_default,
                $create,
                $destroy,
                Command::default(),
                0b0011_0000_0000_0000
            );

            test!(
                send_value,
                $create,
                $destroy,
                Command::default().value($value),
                $expected_value
            );

            #[test]
            fn cannot_send_invalid_value() {
                let mut dev = $create(&[]);
                assert_invalid_value(&dev.send(Command::default().value($too_big_value)));
                dev.$destroy().0.done();
            }

            test!(
                send_shutdown,
                $create,
                $destroy,
                Command::default().shutdown(),
                0b0010_0000_0000_0000
            );

            test!(
                send_double_gain,
                $create,
                $destroy,
                Command::default().double_gain(),
                0b0001_0000_0000_0000
            );

            test!(
                send_buffered,
                $create,
                $destroy,
                Command::default().buffered(),
                0b0111_0000_0000_0000
            );
        }
    };
}

ic_test!(
    mcp4921,
    new_mcp4921,
    destroy_mcp4921,
    0b0000_1010_1010_1010,
    0b0011_1010_1010_1010,
    1 << 12
);
