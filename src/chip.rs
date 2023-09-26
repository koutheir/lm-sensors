//! Chips controlling sensors and actuators.

#[cfg(test)]
mod tests;

use core::ffi::CStr;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::{fmt, mem, ptr};
use std::ffi::{CString, OsStr};
use std::io;
use std::os::raw::c_int;
use std::path::Path;

use sensors_sys::*;

use crate::errors::{Error, Result};
use crate::utils::api_access_lock;
use crate::Bus;

/// Chip connected to sensors or actuators.
#[derive(Debug, PartialEq, Eq)]
pub struct Chip<'a> {
    pub(crate) raw: sensors_chip_name,
    pub(crate) _phantom: &'a PhantomData<crate::LMSensors>,
}

impl<'a> Chip<'a> {
    /// See: [`sensors_parse_chip_name`].
    pub(crate) fn new(name: &str) -> Result<Self> {
        // Though undocumented, sensors_parse_chip_name() assumes its output
        // parameter to be zero-initialized.
        let c_name = CString::new(name)?;
        let mut result = MaybeUninit::zeroed();

        let r = api_access_lock()
            .lock()
            // Safety: `c_name` and `result` are properly initialized.
            .map(|_guard| unsafe {
                sensors_parse_chip_name(c_name.as_ptr(), result.as_mut_ptr())
            })?;

        if r == 0 {
            Ok(Self {
                // Safety: sensors_parse_chip_name() initialized `result`.
                raw: unsafe { result.assume_init() },
                _phantom: &PhantomData,
            })
        } else {
            Err(Error::from_lm_sensors("sensors_parse_chip_name()", r))
        }
    }

    /// # Safety
    /// It is the responsibility of the caller to call
    /// [`sensors_free_chip_name`] on the result.
    /// Failing to do so leaks memory.
    #[must_use]
    pub unsafe fn into_raw_parts(self) -> sensors_chip_name {
        let raw = self.raw;
        mem::forget(self);
        raw
    }

    /// Returns a shared reference to the raw data structure [`sensors_chip_name`].
    #[must_use]
    pub fn raw_ref(&self) -> &sensors_chip_name {
        &self.raw
    }

    /// Returns an exclusive reference to the raw data structure [`sensors_chip_name`].
    ///
    /// # Safety
    /// Changing the raw data structure in an unsupported way leads to undefined results.
    #[must_use]
    pub unsafe fn raw_mut(&mut self) -> &mut sensors_chip_name {
        &mut self.raw
    }

    /// Return a shared reference to this chip.
    #[must_use]
    pub fn as_ref(&'a self) -> ChipRef<'a> {
        ChipRef(&self.raw)
    }

    /// Set the bus connected to this chip.
    ///
    /// See: [`Chip::do_chip_sets`].
    pub fn set_bus(&mut self, new_bus: &Bus) {
        self.raw.bus = new_bus.0;
    }

    /// Return an iterator which yields all sensors and actuators
    /// (*a.k.a.,* features) controlled by this chip.
    pub fn feature_iter(&'a self) -> crate::feature::Iter<'a> {
        crate::feature::Iter {
            chip: ChipRef(&self.raw),
            state: 0,
        }
    }

    /// Return name of this chip, if it is valid UTF-8.
    pub fn name(&self) -> Result<String> {
        self.as_ref().name()
    }

    /// Return the prefix of this chip, if it is valid UTF-8.
    #[must_use]
    pub fn prefix(&self) -> Option<Result<&str>> {
        self.as_ref().prefix()
    }

    /// Return the path of the driver of this chip, if available.
    #[cfg(unix)]
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.as_ref().path()
    }

    /// Return the address of this chip, if available.
    #[must_use]
    pub fn address(&self) -> Option<c_int> {
        self.as_ref().address()
    }

    /// Execute all set statements for this chip.
    ///
    /// See: [`sensors_do_chip_sets`].
    pub fn do_chip_sets(&self) -> Result<()> {
        self.as_ref().do_chip_sets()
    }

    /// Return a copy of the bus connected to this chip.
    #[must_use]
    pub fn bus(&self) -> Bus {
        Bus(self.raw.bus)
    }

    /// Return the raw name of this chip.
    ///
    /// See: [`sensors_snprintf_chip_name`].
    pub fn raw_name(&self) -> Result<CString> {
        self.as_ref().raw_name()
    }

    /// Return the raw prefix of this chip, if available.
    #[must_use]
    pub fn raw_prefix(&self) -> Option<&CStr> {
        self.as_ref().raw_prefix()
    }

    /// Return the raw path of the driver of this chip, if available.
    #[must_use]
    pub fn raw_path(&self) -> Option<&CStr> {
        self.as_ref().raw_path()
    }

    /// Return the raw address of this chip, which is either a number,
    /// or [`SENSORS_CHIP_NAME_ADDR_ANY`].
    #[must_use]
    pub fn raw_address(&self) -> c_int {
        self.raw.addr
    }
}

impl<'a> Drop for Chip<'a> {
    /// See: [`sensors_free_chip_name`].
    fn drop(&mut self) {
        let _ignored = api_access_lock()
            .lock()
            // Safety: sensors_free_chip_name() is assumed to be safe.
            .map(|_guard| unsafe { sensors_free_chip_name(&mut self.raw) });
    }
}

impl<'a> PartialEq<ChipRef<'_>> for Chip<'a> {
    fn eq(&self, other: &ChipRef<'_>) -> bool {
        self.as_ref() == *other
    }
}

impl<'a> PartialEq<Chip<'_>> for ChipRef<'a> {
    fn eq(&self, other: &Chip<'_>) -> bool {
        *self == other.as_ref()
    }
}

impl<'a> fmt::Display for Chip<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(name) = self.raw_name() {
            write!(f, "{}", name.to_string_lossy())
        } else {
            write!(f, "\u{fffd}")
        }
    }
}

/// Shared reference to a chip connected to sensors or actuators.
#[derive(Debug, Clone, Copy, Eq)]
pub struct ChipRef<'a>(pub(crate) &'a sensors_chip_name);

impl<'a> ChipRef<'a> {
    /// Returns a shared reference to the raw data structure [`sensors_chip_name`].
    #[must_use]
    pub fn raw_ref(self) -> &'a sensors_chip_name {
        self.0
    }

    /// Return an iterator which yields all sensors and actuators
    /// (*a.k.a.,* features) controlled by this chip.
    pub fn feature_iter(self) -> crate::feature::Iter<'a> {
        crate::feature::Iter {
            chip: self,
            state: 0,
        }
    }

    /// Return name of this chip, if it is valid UTF-8.
    pub fn name(self) -> Result<String> {
        self.raw_name()?.into_string().map_err(Into::into)
    }

    /// Return the prefix of this chip, if it is valid UTF-8.
    #[must_use]
    pub fn prefix(self) -> Option<Result<&'a str>> {
        self.raw_prefix()
            .map(|prefix| prefix.to_str().map_err(Into::into))
    }

    /// Return the path of the driver of this chip, if available.
    #[cfg(unix)]
    #[must_use]
    pub fn path(self) -> Option<&'a Path> {
        use std::os::unix::ffi::OsStrExt;

        self.raw_path()
            .map(CStr::to_bytes)
            .map(OsStr::from_bytes)
            .map(Path::new)
    }

    /// Return the address of this chip, if available.
    #[must_use]
    pub fn address(self) -> Option<c_int> {
        let addr = self.raw_address();
        (addr != SENSORS_CHIP_NAME_ADDR_ANY).then_some(addr)
    }

    /// Execute all set statements for this chip.
    ///
    /// See: [`sensors_do_chip_sets`].
    pub fn do_chip_sets(self) -> Result<()> {
        let r = api_access_lock()
            .lock()
            // Safety: sensors_do_chip_sets() is assumed to be safe.
            .map(|_guard| unsafe { sensors_do_chip_sets(self.0) })?;
        if r == 0 {
            Ok(())
        } else {
            Err(Error::from_lm_sensors("sensors_do_chip_sets()", r))
        }
    }

    /// Return a copy of the bus connected to this chip.
    #[must_use]
    pub fn bus(self) -> Bus {
        Bus(self.0.bus)
    }

    /// Return the raw name of this chip.
    ///
    /// See: [`sensors_snprintf_chip_name`].
    pub fn raw_name(self) -> Result<CString> {
        let (r, mut buffer) = api_access_lock().lock().map(|_guard| {
            // Safety: sensors_snprintf_chip_name(NULL,0,...) is assumed to be safe.
            let result = unsafe { sensors_snprintf_chip_name(ptr::null_mut(), 0, self.0) };
            if result < 0 {
                (result, Vec::default())
            } else {
                let mut buffer = vec![0_u8; (result as usize).saturating_add(1)];

                // Safety: `buffer` was properly initialized.
                let result = unsafe {
                    sensors_snprintf_chip_name(buffer.as_mut_ptr().cast(), buffer.len(), self.0)
                };
                (result, buffer)
            }
        })?;

        if r < 0 {
            Err(Error::from_lm_sensors("sensors_snprintf_chip_name()", r))
        } else {
            let len = r as usize;
            if len >= buffer.len() {
                // The name was truncated.
                let err = io::ErrorKind::InvalidData.into();
                Err(Error::from_io("sensors_snprintf_chip_name()", err))
            } else {
                buffer.resize_with(len, Default::default);
                CString::new(buffer).map_err(Into::into)
            }
        }
    }

    /// Return the raw prefix of this chip, if available.
    #[must_use]
    pub fn raw_prefix(self) -> Option<&'a CStr> {
        // Safety: if `prefix` is not null, then it is assumed to be a null-terminated string.
        (!self.0.prefix.is_null()).then(|| unsafe { CStr::from_ptr(self.0.prefix) })
    }

    /// Return the raw path of the driver of this chip, if available.
    #[must_use]
    pub fn raw_path(self) -> Option<&'a CStr> {
        // Safety: if `path` is not null, then it is assumed to be a null-terminated string.
        (!self.0.path.is_null()).then(|| unsafe { CStr::from_ptr(self.0.path) })
    }

    /// Return the raw address of this chip, which is either a number,
    /// or [`SENSORS_CHIP_NAME_ADDR_ANY`].
    #[must_use]
    pub fn raw_address(self) -> c_int {
        self.0.addr
    }
}

impl<'a> PartialEq<ChipRef<'_>> for ChipRef<'a> {
    fn eq(&self, other: &ChipRef<'_>) -> bool {
        self.raw_address() == other.raw_address()
            && self.bus() == other.bus()
            && self.raw_prefix() == other.raw_prefix()
            && self.raw_path() == other.raw_path()
    }
}

impl<'a> fmt::Display for ChipRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(name) = self.raw_name() {
            write!(f, "{}", name.to_string_lossy())
        } else {
            write!(f, "\u{fffd}")
        }
    }
}

/// Iterator over available chips. Yields [`ChipRef`]s.
#[derive(Debug)]
#[must_use]
pub struct Iter<'a> {
    pub(crate) state: c_int,
    pub(crate) match_pattern: Option<ChipRef<'a>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = ChipRef<'a>;

    /// See: [`sensors_get_detected_chips`].
    fn next(&mut self) -> Option<Self::Item> {
        let match_pattern = self.match_pattern.map_or_else(ptr::null, |c| c.raw_ref());

        api_access_lock()
            .lock()
            // Safety: `match_pattern` is null or initialized, and `state` is initialized.
            .map(|_guard| unsafe {
                sensors_get_detected_chips(match_pattern, &mut self.state).as_ref()
            })
            .ok()?
            .map(ChipRef)
    }
}
