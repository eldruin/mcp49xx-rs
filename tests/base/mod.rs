use embedded_hal_mock::pin::{Mock as PinMock, State as PinState, Transaction as PinTrans};
use embedded_hal_mock::spi::{Mock as SpiMock, Transaction as SpiTrans};
use mcp49xx::{marker, Mcp49xx};

macro_rules! device_support {
    ($create:ident, $resolution:ident, $channels:ident, $buffering:ident) => {
        pub fn $create(
            transactions: &[SpiTrans],
        ) -> Mcp49xx<PinMock, SpiMock, marker::$resolution, marker::$channels, marker::$buffering> {
            let pin_transactions: Vec<PinTrans> = transactions
                .iter()
                .flat_map(|_| [PinTrans::set(PinState::Low), PinTrans::set(PinState::High)])
                .collect();
            Mcp49xx::$create(PinMock::new(&pin_transactions))
        }
    };
}

device_support!(new_mcp4921, Resolution12Bit, SingleChannel, Buffered);
device_support!(new_mcp4911, Resolution10Bit, SingleChannel, Buffered);
device_support!(new_mcp4901, Resolution8Bit, SingleChannel, Buffered);

device_support!(new_mcp4922, Resolution12Bit, DualChannel, Buffered);
device_support!(new_mcp4912, Resolution10Bit, DualChannel, Buffered);
device_support!(new_mcp4902, Resolution8Bit, DualChannel, Buffered);

device_support!(new_mcp4821, Resolution12Bit, SingleChannel, Unbuffered);
device_support!(new_mcp4811, Resolution10Bit, SingleChannel, Unbuffered);
device_support!(new_mcp4801, Resolution8Bit, SingleChannel, Unbuffered);

device_support!(new_mcp4822, Resolution12Bit, DualChannel, Unbuffered);
device_support!(new_mcp4812, Resolution10Bit, DualChannel, Unbuffered);
device_support!(new_mcp4802, Resolution8Bit, DualChannel, Unbuffered);

#[macro_export]
macro_rules! for_all_ics_with_buffering {
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

#[macro_export]
macro_rules! for_all_ics_without_buffering {
    ($name:ident) => {
        mod $name {
            use super::*;
            $name!(for_mcp4801, new_mcp4801);
            $name!(for_mcp4802, new_mcp4802);
            $name!(for_mcp4811, new_mcp4811);
            $name!(for_mcp4812, new_mcp4812);
            $name!(for_mcp4821, new_mcp4821);
            $name!(for_mcp4822, new_mcp4822);
        }
    };
}

#[macro_export]
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
            $name!(for_mcp4801, new_mcp4801);
            $name!(for_mcp4802, new_mcp4802);
            $name!(for_mcp4811, new_mcp4811);
            $name!(for_mcp4812, new_mcp4812);
            $name!(for_mcp4821, new_mcp4821);
            $name!(for_mcp4822, new_mcp4822);
        }
    };
}

#[macro_export]
macro_rules! for_all_single_channel_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4901, new_mcp4901 $(, $arg)*);
            $macroname!(for_mcp4911, new_mcp4911 $(, $arg)*);
            $macroname!(for_mcp4921, new_mcp4921 $(, $arg)*);
            $macroname!(for_mcp4801, new_mcp4801 $(, $arg)*);
            $macroname!(for_mcp4811, new_mcp4811 $(, $arg)*);
            $macroname!(for_mcp4821, new_mcp4821 $(, $arg)*);
        }
    };
}

#[macro_export]
macro_rules! for_all_dual_channel_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4902, new_mcp4902 $(, $arg)*);
            $macroname!(for_mcp4912, new_mcp4912 $(, $arg)*);
            $macroname!(for_mcp4922, new_mcp4922 $(, $arg)*);
            $macroname!(for_mcp4802, new_mcp4802 $(, $arg)*);
            $macroname!(for_mcp4812, new_mcp4812 $(, $arg)*);
            $macroname!(for_mcp4822, new_mcp4822 $(, $arg)*);
        }
    };
}

#[macro_export]
macro_rules! for_all_12bit_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4921, new_mcp4921 $(, $arg)*);
            $macroname!(for_mcp4922, new_mcp4922 $(, $arg)*);
            $macroname!(for_mcp4821, new_mcp4821 $(, $arg)*);
            $macroname!(for_mcp4822, new_mcp4822 $(, $arg)*);
        }
    };
}

#[macro_export]
macro_rules! for_all_10bit_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4911, new_mcp4911 $(, $arg)*);
            $macroname!(for_mcp4912, new_mcp4912 $(, $arg)*);
            $macroname!(for_mcp4811, new_mcp4811 $(, $arg)*);
            $macroname!(for_mcp4812, new_mcp4812 $(, $arg)*);
        }
    };
}

#[macro_export]
macro_rules! for_all_8bit_ics {
    ($name:ident, $macroname:ident $(, $arg:expr)*) => {
        mod $name {
            use super::*;
            $macroname!(for_mcp4901, new_mcp4901 $(, $arg)*);
            $macroname!(for_mcp4902, new_mcp4902 $(, $arg)*);
            $macroname!(for_mcp4801, new_mcp4801 $(, $arg)*);
            $macroname!(for_mcp4802, new_mcp4802 $(, $arg)*);
        }
    };
}
