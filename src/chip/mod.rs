//! Chips controlling sensors and actuators.

#[cfg(test)]
mod tests;

use std::ffi::{CStr, CString, OsStr};
use std::marker::PhantomData;
use std::mem::{self, MaybeUninit};
use std::os::raw::c_int;
use std::path::Path;
use std::{fmt, io, ptr};

use sensors_sys::*;

use crate::errors::{Error, Result};
use crate::utils::api_access_lock;
use crate::{BusMut, BusRef, Chip, ChipRef};

/// Iterator over available chips. Yields [`ChipRef`]s.
#[derive(Debug)]
pub struct Iter<'a> {
    pub(crate) state: c_int,
    pub(crate) match_pattern: Option<ChipRef<'a>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = ChipRef<'a>;

    /// See: [`sensors_get_detected_chips`].
    fn next(&mut self) -> Option<Self::Item> {
        let match_pattern = self.match_pattern.map_or_else(ptr::null, |c| c.as_ref());

        api_access_lock()
            .lock()
            .map(|_guard| unsafe {
                sensors_get_detected_chips(match_pattern, &mut self.state).as_ref()
            })
            .ok()?
            .map(ChipRef)
    }
}

/// Functions of a chip available without exclusive access to the chip.
pub trait SharedChip: AsRef<sensors_chip_name> + PartialEq<Self> {
    /// Return name of this chip, if it is valid UTF-8.
    fn name(&self) -> Result<String> {
        self.raw_name()?.into_string().map_err(Into::into)
    }

    /// Return the prefix of this chip, if it is valid UTF-8.
    fn prefix(&self) -> Option<Result<&str>> {
        self.raw_prefix()
            .map(|prefix| prefix.to_str().map_err(Into::into))
    }

    /// Return the path of the driver of this chip, if available.
    #[cfg(unix)]
    fn path(&self) -> Option<&Path> {
        use std::os::unix::ffi::OsStrExt;

        self.raw_path()
            .map(CStr::to_bytes)
            .map(OsStr::from_bytes)
            .map(Path::new)
    }

    /// Return the address of this chip, if available.
    fn address(&self) -> Option<c_int> {
        let addr = self.raw_address();
        (addr != SENSORS_CHIP_NAME_ADDR_ANY).then_some(addr)
    }

    /// Execute all set statements for this chip.
    ///
    /// See: [`sensors_do_chip_sets`].
    fn do_chip_sets(&self) -> Result<()> {
        let r = api_access_lock()
            .lock()
            .map(|_guard| unsafe { sensors_do_chip_sets(self.as_ref()) })?;
        if r == 0 {
            Ok(())
        } else {
            Err(Error::from_lm_sensors("sensors_do_chip_sets()", r))
        }
    }

    /// Return the bus connected to this chip.
    fn bus(&self) -> BusRef {
        BusRef(&self.as_ref().bus)
    }

    /// Return an iterator which yields all sensors and actuators
    /// (*a.k.a.,* features) controlled by this chip.
    fn feature_iter(&self) -> crate::feature::Iter;

    /// Return the raw name of this chip.
    ///
    /// See: [`sensors_snprintf_chip_name`].
    fn raw_name(&self) -> Result<CString> {
        let (r, mut buffer) = api_access_lock().lock().map(|_guard| {
            let result = unsafe { sensors_snprintf_chip_name(ptr::null_mut(), 0, self.as_ref()) };
            if result < 0 {
                (result, Vec::default())
            } else {
                let mut buffer = vec![0_u8; (result as usize).saturating_add(1)];
                let result = unsafe {
                    sensors_snprintf_chip_name(
                        buffer.as_mut_ptr().cast(),
                        buffer.len(),
                        self.as_ref(),
                    )
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
    fn raw_prefix(&self) -> Option<&CStr> {
        let prefix = self.as_ref().prefix;
        (!prefix.is_null()).then(|| unsafe { CStr::from_ptr(prefix) })
    }

    /// Return the raw path of the driver of this chip, if available.
    fn raw_path(&self) -> Option<&CStr> {
        let path = self.as_ref().path;
        (!path.is_null()).then(|| unsafe { CStr::from_ptr(path) })
    }

    /// Return the raw address of this chip, which is either a number,
    /// or [`SENSORS_CHIP_NAME_ADDR_ANY`].
    fn raw_address(&self) -> c_int {
        self.as_ref().addr
    }
}

impl<'a> SharedChip for ChipRef<'a> {
    fn feature_iter(&self) -> crate::feature::Iter {
        crate::feature::Iter {
            chip: *self,
            state: 0,
        }
    }
}

impl<'a> AsRef<sensors_chip_name> for ChipRef<'a> {
    fn as_ref(&self) -> &'a sensors_chip_name {
        self.0
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

impl<'a> Chip<'a> {
    /// # Safety
    /// It is the responsibility of the caller to call
    /// [`sensors_free_chip_name`] on the result.
    /// Failing to do so leaks memory.
    #[must_use]
    pub fn into_raw_parts(self) -> sensors_chip_name {
        let raw = self.raw;
        mem::forget(self);
        raw
    }

    /// Return a shared reference to this chip.
    #[must_use]
    pub fn as_ref(&self) -> ChipRef {
        ChipRef(&self.raw)
    }

    /// See: [`sensors_parse_chip_name`].
    pub(crate) fn new(name: &str) -> Result<Self> {
        // Though undocumented, sensors_parse_chip_name() assumes its output
        // parameter to be zero-initialized.
        let c_name = CString::new(name)?;
        let mut result = MaybeUninit::zeroed();

        let r = api_access_lock().lock().map(|_guard| unsafe {
            sensors_parse_chip_name(c_name.as_ptr(), result.as_mut_ptr())
        })?;

        if r == 0 {
            Ok(Self {
                raw: unsafe { result.assume_init() },
                _phantom: &PhantomData,
            })
        } else {
            Err(Error::from_lm_sensors("sensors_parse_chip_name()", r))
        }
    }

    /// Return an exclusive reference to the bus connected to this chip.
    pub fn bus_mut(&mut self) -> BusMut {
        BusMut(&mut self.as_mut().bus)
    }
}

impl<'a> Drop for Chip<'a> {
    /// See: [`sensors_free_chip_name`].
    fn drop(&mut self) {
        let _ignored = api_access_lock()
            .lock()
            .map(|_guard| unsafe { sensors_free_chip_name(self.as_mut()) });
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

impl<'a> SharedChip for Chip<'a> {
    fn feature_iter(&self) -> crate::feature::Iter {
        crate::feature::Iter {
            chip: ChipRef(&self.raw),
            state: 0,
        }
    }
}

impl<'a> AsRef<sensors_chip_name> for Chip<'a> {
    fn as_ref(&self) -> &sensors_chip_name {
        &self.raw
    }
}

impl<'a> AsMut<sensors_chip_name> for Chip<'a> {
    fn as_mut(&mut self) -> &mut sensors_chip_name {
        &mut self.raw
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
