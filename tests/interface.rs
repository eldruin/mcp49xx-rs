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

mod mcp4921 {
    use super::*;

    test!(
        send_default,
        new_mcp4921,
        destroy_mcp4921,
        Command::default(),
        0b0011_0000_0000_0000
    );

    test!(
        send_value,
        new_mcp4921,
        destroy_mcp4921,
        Command::default().value(0b0000_1010_1010_1010),
        0b0011_1010_1010_1010
    );

    #[test]
    fn cannot_send_invalid_value() {
        let mut dev = new_mcp4921(&[]);
        assert_invalid_value(&dev.send(Command::default().value(1 << 12)));
        dev.destroy_mcp4921().0.done();
    }

    test!(
        send_shutdown,
        new_mcp4921,
        destroy_mcp4921,
        Command::default().shutdown(),
        0b0010_0000_0000_0000
    );

    test!(
        send_double_gain,
        new_mcp4921,
        destroy_mcp4921,
        Command::default().double_gain(),
        0b0001_0000_0000_0000
    );

    test!(
        send_buffered,
        new_mcp4921,
        destroy_mcp4921,
        Command::default().buffered(),
        0b0111_0000_0000_0000
    );
}
