//! Bus connections.

#[cfg(test)]
mod tests;

use core::ffi::CStr;
use core::fmt;
use std::io;
use std::os::raw::{c_int, c_short};

use sensors_sys::*;

use crate::errors::{Error, Result};
use crate::utils::api_access_lock;

/// Bus connection of some [`Kind`], *e.g.,* PCI.
///
/// [`Kind`]: crate::bus::Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bus(pub(crate) sensors_bus_id);

impl Bus {
    /// Return the adapter name of this bus.
    ///
    /// If it could not be found, it returns an error.
    pub fn name(&self) -> Result<&str> {
        self.raw_name()?.to_str().map_err(Into::into)
    }

    /// Return the bus type.
    #[must_use]
    pub fn kind(&self) -> Option<Kind> {
        Kind::from_raw(self.raw_kind())
    }

    /// Return the bus number.
    #[must_use]
    pub fn number(&self) -> Number {
        self.raw_number().into()
    }

    /// Return the adapter name of this bus *(type, number)* pair, as used
    /// within [`sensors_chip_name`].
    /// If it could not be found, it returns an error.
    ///
    /// See: [`sensors_get_adapter_name`].
    pub fn raw_name(&self) -> Result<&CStr> {
        let name = api_access_lock()
            .lock()
            // Safety: this is assumed to be safe.
            .map(|_guard| unsafe { sensors_get_adapter_name(&self.0) })?;

        (!name.is_null())
            // Safety: sensors_get_adapter_name() returned a null-terminated string.
            .then(|| unsafe { CStr::from_ptr(name) })
            .ok_or_else(|| {
                let err = io::ErrorKind::NotFound.into();
                Error::from_io("sensors_get_adapter_name", err)
            })
    }

    /// Return one of `SENSORS_BUS_TYPE_*` values,
    /// *e.g.*, [`SENSORS_BUS_TYPE_ANY`].
    #[must_use]
    pub fn raw_kind(&self) -> c_short {
        self.0.type_
    }

    /// Return a number, or one of `SENSORS_BUS_NR_*` values,
    /// *e.g.*, [`SENSORS_BUS_NR_ANY`].
    #[must_use]
    pub fn raw_number(&self) -> c_short {
        self.0.nr
    }

    /// Set the bus type.
    pub fn set_kind(&mut self, kind: Kind) {
        self.set_raw_kind(c_short::from(kind));
    }

    /// Set the bus number.
    pub fn set_number(&mut self, number: Number) {
        self.set_raw_number(number.into());
    }

    /// Set the bus type to one of `SENSORS_BUS_TYPE_*` values,
    /// *e.g.*, [`SENSORS_BUS_TYPE_PCI`].
    pub fn set_raw_kind(&mut self, kind: c_short) {
        self.0.type_ = kind;
    }

    /// Set the bus number to one of `SENSORS_BUS_NR_*` values
    /// (*e.g.*, [`SENSORS_BUS_NR_ANY`]), or to a specific number.
    pub fn set_raw_number(&mut self, number: c_short) {
        self.0.nr = number;
    }
}

impl AsMut<sensors_bus_id> for Bus {
    fn as_mut(&mut self) -> &mut sensors_bus_id {
        &mut self.0
    }
}

impl AsRef<sensors_bus_id> for Bus {
    fn as_ref(&self) -> &sensors_bus_id {
        &self.0
    }
}

impl fmt::Display for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(name) = self.raw_name() {
            write!(f, "{}", name.to_string_lossy())
        } else {
            write!(f, "\u{fffd}")
        }
    }
}

/// Type of a [`Bus`].
#[repr(i16)]
#[non_exhaustive]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
pub enum Kind {
    /// Any type of bus.
    Any = SENSORS_BUS_TYPE_ANY as c_short,
    /// Inter-Integrated Circuit.
    I2C = SENSORS_BUS_TYPE_I2C as c_short,
    /// Industry Standard Architecture.
    ISA = SENSORS_BUS_TYPE_ISA as c_short,
    /// Peripheral Component Interconnect.
    PCI = SENSORS_BUS_TYPE_PCI as c_short,
    /// Serial Peripheral Interface.
    SPI = SENSORS_BUS_TYPE_SPI as c_short,
    /// Virtual bus.
    Virtual = SENSORS_BUS_TYPE_VIRTUAL as c_short,
    /// Advanced Configuration and Power Interface.
    ACPI = SENSORS_BUS_TYPE_ACPI as c_short,
    /// Human Interface Device.
    HID = SENSORS_BUS_TYPE_HID as c_short,
    /// Management Data Input/Output.
    MDIO = SENSORS_BUS_TYPE_MDIO as c_short,
    /// Small Computer System Interface.
    SCSI = SENSORS_BUS_TYPE_SCSI as c_short,
}

impl Kind {
    /// Return an instance from one of the `SENSORS_BUS_TYPE_*` values,
    /// *e.g.,* [`SENSORS_BUS_TYPE_PCI`].
    #[must_use]
    pub fn from_raw(kind: c_short) -> Option<Self> {
        Self::try_from(kind).ok()
    }

    /// Return one of the `SENSORS_BUS_TYPE_*` values
    /// (*e.g.,* [`SENSORS_BUS_TYPE_PCI`]) equivalent to this instance.
    #[must_use]
    pub fn as_raw(self) -> c_short {
        self.into()
    }
}

impl Default for Kind {
    fn default() -> Self {
        Self::Any
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Any => write!(f, "Any"),
            Self::I2C => write!(f, "Inter-Integrated Circuit (I2C)"),
            Self::ISA => write!(f, "Industry Standard Architecture (ISA)"),
            Self::PCI => write!(f, "Peripheral Component Interconnect (PCI)"),
            Self::SPI => write!(f, "Serial Peripheral Interface (SPI)"),
            Self::Virtual => write!(f, "Virtual"),
            Self::ACPI => write!(f, "Advanced Configuration and Power Interface (ACPI)"),
            Self::HID => write!(f, "Human Interface Device (HID)"),
            Self::MDIO => write!(f, "Management Data Input/Output (MDIO)"),
            Self::SCSI => write!(f, "Small Computer System Interface (SCSI)"),
        }
    }
}

/// Number of a [`Bus`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    /// A bus of any number.
    Any,
    /// Ignored bus number.
    Ignore,
    /// Bus of a specific number.
    Number(c_short),
}

impl Default for Number {
    fn default() -> Self {
        Self::Any
    }
}

impl From<c_short> for Number {
    fn from(other: c_short) -> Self {
        match c_int::from(other) {
            SENSORS_BUS_NR_ANY => Number::Any,
            SENSORS_BUS_NR_IGNORE => Number::Ignore,
            _ => Number::Number(other),
        }
    }
}

impl From<Number> for c_short {
    fn from(other: Number) -> Self {
        match other {
            Number::Any => SENSORS_BUS_NR_ANY as Self,
            Number::Ignore => SENSORS_BUS_NR_IGNORE as Self,
            Number::Number(n) => n,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Number::Any => write!(f, "Any"),
            Number::Ignore => write!(f, "Ignore"),
            Number::Number(n) => write!(f, "{n}"),
        }
    }
}
