//! Data reported by a sensor or set for an actuator (*a.k.a.,* sub-feature)
//! controlled by a chip.

#[cfg(test)]
mod tests;

use std::ffi::CStr;
use std::fmt::Debug;
use std::os::raw::{c_int, c_uint};
use std::{fmt, io};

use bitflags::bitflags;
use sensors_sys::*;

use crate::errors::{Error, Result};
use crate::utils::api_access_lock;
use crate::value::Kind;
use crate::{FeatureRef, SubFeatureRef, Value};

/// Iterator over available sub-features of a chip. Yields [`SubFeatureRef`]s.
#[derive(Debug)]
pub struct Iter<'a> {
    pub(crate) feature: FeatureRef<'a>,
    pub(crate) state: c_int,
}

impl<'a> Iterator for Iter<'a> {
    type Item = SubFeatureRef<'a>;

    /// See: [`sensors_get_all_subfeatures`].
    fn next(&mut self) -> Option<Self::Item> {
        api_access_lock()
            .lock()
            .map(|_guard| unsafe {
                sensors_get_all_subfeatures(
                    self.feature.chip.as_ref(),
                    self.feature.as_ref(),
                    &mut self.state,
                )
                .as_ref()
            })
            .ok()?
            .map(move |raw| SubFeatureRef {
                feature: self.feature,
                raw,
            })
    }
}

impl<'a> SubFeatureRef<'a> {
    /// Return the feature to which this sub-feature belongs.
    #[must_use]
    pub fn feature(&self) -> FeatureRef {
        self.feature
    }

    /// Return the name of this sub-feature, if available and valid UTF-8.
    ///
    /// This returns `None` if no name is available, and returns `Some(Err(_))`
    /// if the available name is not valid UTF-8.
    #[must_use]
    pub fn name(&self) -> Option<Result<&str>> {
        self.raw_name()
            .map(|name| name.to_str().map_err(Into::into))
    }

    /// Return the number of this sub-feature.
    #[must_use]
    pub fn number(&self) -> c_int {
        self.raw.number
    }

    /// Return the number of a main feature this sub-feature belongs to.
    #[must_use]
    pub fn mapping(&self) -> c_int {
        self.raw.mapping
    }

    /// Return the type of this sub-feature, if it is valid [`Kind`].
    #[must_use]
    pub fn kind(&self) -> Option<Kind> {
        Kind::from_raw(self.raw_kind())
    }

    /// Return the flags of this sub-feature, if it is valid [`Flags`].
    #[must_use]
    pub fn flags(&self) -> Option<Flags> {
        Flags::from_bits(self.raw_flags())
    }

    /// Return the value reported by this sub-feature, *e.g.,* sensor.
    pub fn value(&self) -> Result<Value> {
        let value = self.raw_value()?;
        Value::from_raw(self.raw_kind(), value)
            .ok_or_else(|| Error::from_io("Value::from_raw", io::ErrorKind::InvalidData.into()))
    }

    /// Set the value associated with this sub-feature, *e.g.,* actuator.
    pub fn set_value(&self, new_value: &Value) -> Result<()> {
        self.set_raw_value(new_value.raw_value())
    }

    /// Return the raw name of this sub-feature, if available.
    #[must_use]
    pub fn raw_name(&self) -> Option<&CStr> {
        let name = self.as_ref().name;
        (!name.is_null()).then(move || unsafe { CStr::from_ptr(name) })
    }

    /// Return the raw type of this sub-feature, which is one
    /// of `SENSORS_SUBFEATURE_*`, *e.g.*, [`SENSORS_SUBFEATURE_TEMP_INPUT`].
    ///
    /// [`SENSORS_SUBFEATURE_TEMP_INPUT`]: sensors_subfeature_type::SENSORS_SUBFEATURE_TEMP_INPUT
    #[must_use]
    pub fn raw_kind(&self) -> c_uint {
        self.as_ref().type_
    }

    /// Return the raw flags of this sub-feature, which is a combination
    /// of [`SENSORS_MODE_R`], [`SENSORS_MODE_W`] and [`SENSORS_COMPUTE_MAPPING`].
    #[must_use]
    pub fn raw_flags(&self) -> c_uint {
        self.as_ref().flags
    }

    /// Return the raw value reported by this sub-feature, *e.g.,* sensor.
    ///
    /// See: [`sensors_get_value`].
    pub fn raw_value(&self) -> Result<f64> {
        let mut result = 0.0_f64;

        let r = api_access_lock().lock().map(|_guard| unsafe {
            sensors_get_value(
                self.feature.chip.as_ref(),
                self.feature.as_ref().number,
                &mut result,
            )
        })?;
        if r == 0 {
            Ok(result)
        } else {
            Err(Error::from_lm_sensors("sensors_get_value()", r))
        }
    }

    /// Set the raw value associated with this sub-feature, *e.g.,* actuator.
    ///
    /// See: [`sensors_set_value`].
    pub fn set_raw_value(&self, new_value: f64) -> Result<()> {
        let chip = self.feature.chip.as_ref();
        let number = self.feature.as_ref().number;
        let r = api_access_lock()
            .lock()
            .map(move |_guard| unsafe { sensors_set_value(chip, number, new_value) })?;
        if r == 0 {
            Ok(())
        } else {
            Err(Error::from_lm_sensors("sensors_set_value()", r))
        }
    }
}

impl<'a> AsRef<sensors_subfeature> for SubFeatureRef<'a> {
    fn as_ref(&self) -> &sensors_subfeature {
        self.raw
    }
}

impl<'a> PartialEq for SubFeatureRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.feature == other.feature
            && self.number() == other.number()
            && self.mapping() == other.mapping()
            && self.raw_kind() == other.raw_kind()
            && self.raw_flags() == other.raw_flags()
            && self.raw_name() == other.raw_name()
    }
}

impl<'a> fmt::Display for SubFeatureRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = self.raw_name() {
            write!(f, "{}", name.to_string_lossy())
        } else {
            Ok(())
        }
    }
}

bitflags! {
    /// Flags of a sub-feature of a chip.
    #[repr(transparent)]
    pub struct Flags: c_uint {
        /// Sub-feature is readable, *e.g.,* sensor data.
        const READABLE = SENSORS_MODE_R as c_uint;

        /// Sub-feature is writable, *e.g.,* actuator value.
        const WRITABLE = SENSORS_MODE_W as c_uint;

        /// Sub-feature value is affected by the computation rules of
        /// the main feature.
        const COMPUTE_MAPPING = SENSORS_COMPUTE_MAPPING as c_uint;
    }
}
