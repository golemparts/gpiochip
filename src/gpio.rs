use std::error;
use std::fmt;
use std::io;
use std::ops::Not;
use std::result;

/// Errors that can occur when accessing the GPIO peripheral.
#[derive(Debug)]
pub enum Error {
    /// Unknown model.
    ///
    /// The Raspberry Pi model or SoC can't be identified. Support for
    /// new models is usually added shortly after they are officially
    /// announced and available to the public. Make sure you're using
    /// the latest release of RPPAL.
    ///
    /// You may also encounter this error if your Linux distribution
    /// doesn't provide any of the common user-accessible system files
    /// that are used to identify the model and SoC.
    UnknownModel,
    /// Pin is already in use.
    ///
    /// The pin is already in use elsewhere in your application. If the pin is currently in
    /// use, you may retrieve it again after the [`Pin`] (or a derived [`InputPin`],
    /// [`OutputPin`] or [`IoPin`]) instance goes out of scope.
    ///
    /// [`Pin`]: struct.Pin.html
    /// [`InputPin`]: struct.InputPin.html
    /// [`OutputPin`]: struct.OutputPin.html
    /// [`IoPin`]: struct.IoPin.html
    PinUsed(u8),
    /// Pin is not available.
    ///
    /// The GPIO peripheral doesn't expose a GPIO pin with the specified number. Pins are
    /// addressed by their BCM GPIO numbers, rather than their physical location on the GPIO
    /// header.
    PinNotAvailable(u8),
    /// Permission denied when opening `/dev/gpiomem`, `/dev/mem` or `/dev/gpiochipN` for
    /// read/write access.
    ///
    /// More information on possible causes for this error can be found [here].
    ///
    /// [here]: index.html#permission-denied
    PermissionDenied(String),
    /// I/O error.
    Io(io::Error),
    /// Thread panicked.
    ThreadPanic,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::UnknownModel => write!(f, "Unknown Raspberry Pi model"),
            Error::PinUsed(pin) => write!(f, "Pin {} is already in use", pin),
            Error::PinNotAvailable(pin) => write!(f, "Pin {} is not available", pin),
            Error::PermissionDenied(ref path) => write!(f, "Permission denied: {}", path),
            Error::Io(ref err) => write!(f, "I/O error: {}", err),
            Error::ThreadPanic => write!(f, "Thread panicked"),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

/// Result type returned from methods that can have `rppal::gpio::Error`s.
pub type Result<T> = result::Result<T, Error>;

/// Pin modes.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Mode {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Mode::Input => write!(f, "In"),
            Mode::Output => write!(f, "Out"),
            Mode::Alt0 => write!(f, "Alt0"),
            Mode::Alt1 => write!(f, "Alt1"),
            Mode::Alt2 => write!(f, "Alt2"),
            Mode::Alt3 => write!(f, "Alt3"),
            Mode::Alt4 => write!(f, "Alt4"),
            Mode::Alt5 => write!(f, "Alt5"),
        }
    }
}

/// Pin logic levels.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Level {
    Low = 0,
    High = 1,
}

impl From<bool> for Level {
    fn from(e: bool) -> Level {
        if e {
            Level::High
        } else {
            Level::Low
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Level::Low => write!(f, "Low"),
            Level::High => write!(f, "High"),
        }
    }
}

impl From<u8> for Level {
    fn from(value: u8) -> Self {
        if value == 0 {
            Level::Low
        } else {
            Level::High
        }
    }
}

impl Not for Level {
    type Output = Level;

    fn not(self) -> Level {
        match self {
            Level::Low => Level::High,
            Level::High => Level::Low,
        }
    }
}

/// Built-in pull-up/pull-down resistor states.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PullUpDown {
    Off = 0b00,
    PullDown = 0b01,
    PullUp = 0b10,
}

impl fmt::Display for PullUpDown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PullUpDown::Off => write!(f, "Off"),
            PullUpDown::PullDown => write!(f, "PullDown"),
            PullUpDown::PullUp => write!(f, "PullUp"),
        }
    }
}

/// Interrupt trigger conditions.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Trigger {
    Disabled = 0,
    RisingEdge = 1,
    FallingEdge = 2,
    Both = 3,
}

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Trigger::Disabled => write!(f, "Disabled"),
            Trigger::RisingEdge => write!(f, "RisingEdge"),
            Trigger::FallingEdge => write!(f, "FallingEdge"),
            Trigger::Both => write!(f, "Both"),
        }
    }
}
