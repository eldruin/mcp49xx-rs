use super::Channel;

/// Configurable command that can be sent to the device
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Command {
    pub(crate) channel: Channel,
    pub(crate) buffered: bool,
    double_gain: bool,
    shutdown: bool,
    pub(crate) value: u16,
}

impl Default for Command {
    /// Create new command instance.
    ///
    /// Per default the command is on channel 0, unbuffered, with single gain,
    /// enabled and with value 0.
    fn default() -> Self {
        Command {
            channel: Channel::Ch0,
            buffered: false,
            double_gain: false,
            shutdown: false,
            value: 0,
        }
    }
}

impl Command {
    /// Select the channel
    pub fn channel(self, channel: Channel) -> Self {
        let mut cmd = self;
        cmd.channel = channel;
        cmd
    }

    /// Shutdown the channel
    pub fn shutdown(self) -> Self {
        let mut cmd = self;
        cmd.shutdown = true;
        cmd
    }

    /// Enable the channel (undo a shutdown)
    pub fn enable(self) -> Self {
        let mut cmd = self;
        cmd.shutdown = false;
        cmd
    }

    /// Send the value buffered
    pub fn buffered(self) -> Self {
        let mut cmd = self;
        cmd.buffered = true;
        cmd
    }

    /// Send the value unbuffered
    pub fn unbuffered(self) -> Self {
        let mut cmd = self;
        cmd.buffered = false;
        cmd
    }

    /// Send the value with double gain (2x)
    pub fn double_gain(self) -> Self {
        let mut cmd = self;
        cmd.double_gain = true;
        cmd
    }

    /// Send the value with single gain (1x)
    pub fn single_gain(self) -> Self {
        let mut cmd = self;
        cmd.double_gain = false;
        cmd
    }

    /// Set the value
    pub fn value(self, value: u16) -> Self {
        let mut cmd = self;
        cmd.value = value;
        cmd
    }

    // get the config bits at the beginning of the command
    pub(crate) fn get_config_bits(self) -> u8 {
        let mut value = 0b0011_0000;
        if self.channel != Channel::Ch0 {
            value |= 0b1000_0000;
        }
        if self.buffered {
            value |= 0b0100_0000;
        }
        if self.double_gain {
            value &= 0b1101_1111;
        }
        if self.shutdown {
            value &= 0b1110_1111;
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_cmd(
        cmd: Command,
        channel: Channel,
        buffered: bool,
        double_gain: bool,
        shutdown: bool,
        value: u16,
    ) {
        assert_eq!(cmd.channel, channel);
        assert_eq!(cmd.buffered, buffered);
        assert_eq!(cmd.double_gain, double_gain);
        assert_eq!(cmd.shutdown, shutdown);
        assert_eq!(cmd.value, value);
    }

    #[test]
    fn default_command_all_off() {
        let cmd = Command::default();
        check_cmd(cmd, Channel::Ch0, false, false, false, 0);
    }

    #[test]
    fn can_set_double_gain() {
        let cmd = Command::default().double_gain();
        check_cmd(cmd, Channel::Ch0, false, true, false, 0);
    }

    #[test]
    fn can_set_single_gain() {
        let cmd = Command::default().double_gain().single_gain();
        check_cmd(cmd, Channel::Ch0, false, false, false, 0);
    }

    #[test]
    fn can_set_shutdown() {
        let cmd = Command::default().shutdown();
        check_cmd(cmd, Channel::Ch0, false, false, true, 0);
    }

    #[test]
    fn can_set_enable() {
        let cmd = Command::default().shutdown().enable();
        check_cmd(cmd, Channel::Ch0, false, false, false, 0);
    }

    #[test]
    fn can_set_buffered() {
        let cmd = Command::default().buffered();
        check_cmd(cmd, Channel::Ch0, true, false, false, 0);
    }

    #[test]
    fn can_set_unbuffered() {
        let cmd = Command::default().buffered().unbuffered();
        check_cmd(cmd, Channel::Ch0, false, false, false, 0);
    }

    #[test]
    fn can_set_value() {
        let cmd = Command::default().value(1024);
        check_cmd(cmd, Channel::Ch0, false, false, false, 1024);
    }

    #[test]
    fn operations_leave_original_command_unchanged() {
        let original = Command::default();
        check_cmd(original.shutdown(), Channel::Ch0, false, false, true, 0);
        check_cmd(original, Channel::Ch0, false, false, false, 0);
    }
}
