//! Sensor or actuator (*a.k.a.,* feature) controlled by a chip.

#[cfg(test)]
mod tests;

use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_uint};
use std::{fmt, io};

use sensors_sys::sensors_feature_type::*;
use sensors_sys::*;

use crate::errors::{Error, Result};
use crate::utils::api_access_lock;
use crate::{ChipRef, FeatureRef, SubFeatureRef};

/// Iterator over available features of a chip. Yields [`FeatureRef`]s.
#[derive(Debug)]
pub struct Iter<'a> {
    pub(crate) chip: ChipRef<'a>,
    pub(crate) state: c_int,
}

impl<'a> Iterator for Iter<'a> {
    type Item = FeatureRef<'a>;

    /// See: [`sensors_get_features`].
    fn next(&mut self) -> Option<Self::Item> {
        api_access_lock()
            .lock()
            .map(|_guard| unsafe {
                sensors_get_features(self.chip.as_ref(), &mut self.state).as_ref()
            })
            .ok()?
            .map(move |raw| FeatureRef {
                chip: self.chip,
                raw,
            })
    }
}

impl<'a> FeatureRef<'a> {
    /// Return the chip controlling this feature.
    pub fn chip(&self) -> ChipRef {
        self.chip
    }

    /// Return the name of this feature, if it is valid UTF-8.
    pub fn name(&self) -> Option<Result<&str>> {
        self.raw_name()
            .map(|name| name.to_str().map_err(Into::into))
    }

    /// Return the label of this feature, if it is valid UTF-8.
    pub fn label(&self) -> Result<String> {
        self.raw_label()?.into_string().map_err(Into::into)
    }

    /// Return the number of this feature.
    pub fn number(&self) -> c_int {
        self.as_ref().number
    }

    /// Return the type of this feature, if it is a valid [`Kind`].
    pub fn kind(&self) -> Option<Kind> {
        Kind::from_raw(self.raw_kind())
    }

    /// Return the sub-feature of the given type for a given main feature,
    /// if it exists, or an error otherwise.
    pub fn sub_feature_by_kind(&self, kind: crate::value::Kind) -> Result<SubFeatureRef> {
        self.sub_feature_by_raw_kind(kind as c_uint)
    }

    /// Return an iterator which yields all sub-features belonging
    /// to this feature.
    pub fn sub_feature_iter(&self) -> crate::sub_feature::Iter {
        crate::sub_feature::Iter {
            feature: *self,
            state: 0,
        }
    }

    /// Return the raw name of this feature, if available.
    pub fn raw_name(&self) -> Option<&CStr> {
        let name = self.as_ref().name;
        (!name.is_null()).then(move || unsafe { CStr::from_ptr(name) })
    }

    /// Return the raw label of this feature.
    ///
    /// If no label exists for this feature, its name is returned.
    ///
    /// See: [`sensors_get_label`].
    pub fn raw_label(&self) -> Result<CString> {
        let label = api_access_lock()
            .lock()
            .map(move |_guard| unsafe { sensors_get_label(self.chip().as_ref(), self.as_ref()) })?;

        if label.is_null() {
            let err = io::ErrorKind::InvalidInput.into();
            Err(Error::from_io("sensors_get_label()", err))
        } else {
            Ok(unsafe {
                let result = CString::from(CStr::from_ptr(label));
                libc::free(label.cast());
                result
            })
        }
    }

    /// Return the raw type of this feature, which is one of `SENSORS_FEATURE_*`,
    /// *e.g.*, [`SENSORS_FEATURE_TEMP`].
    ///
    /// [`SENSORS_FEATURE_TEMP`]: sensors_feature_type::SENSORS_FEATURE_TEMP
    pub fn raw_kind(&self) -> c_uint {
        self.as_ref().type_
    }

    /// Return the sub-feature of the given type for a given main feature,
    /// if it exists, or an error otherwise.
    ///
    /// `kind` is one of `SENSORS_SUBFEATURE_*`.
    ///
    /// See: [`sensors_get_subfeature`].
    pub fn sub_feature_by_raw_kind(&self, kind: c_uint) -> Result<SubFeatureRef> {
        let sub_feature = api_access_lock().lock().map(move |_guard| unsafe {
            sensors_get_subfeature(self.chip().as_ref(), self.as_ref(), kind).as_ref()
        })?;

        sub_feature
            .map(|raw| SubFeatureRef {
                feature: *self,
                raw,
            })
            .ok_or_else(|| {
                let err = io::ErrorKind::NotFound.into();
                Error::from_io("sensors_get_subfeature", err)
            })
    }
}

impl<'a> AsRef<sensors_feature> for FeatureRef<'a> {
    fn as_ref(&self) -> &sensors_feature {
        self.raw
    }
}

impl<'a> PartialEq for FeatureRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.chip == other.chip
            && self.number() == other.number()
            && self.raw_kind() == other.raw_kind()
            && self.raw_name() == other.raw_name()
    }
}

impl<'a> fmt::Display for FeatureRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(label) = self.raw_label() {
            write!(f, "{}", label.to_string_lossy())
        } else {
            write!(f, "�")
        }
    }
}

/// Type of a sensor or actuator (*a.k.a.,* feature) controlled by a chip.
#[allow(missing_docs)] // Enum variant names are self-explanatory.
#[repr(u32)]
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
    Voltage = SENSORS_FEATURE_IN,
    Fan = SENSORS_FEATURE_FAN,
    Temperature = SENSORS_FEATURE_TEMP,
    Power = SENSORS_FEATURE_POWER,
    Energy = SENSORS_FEATURE_ENERGY,
    Current = SENSORS_FEATURE_CURR,
    Humidity = SENSORS_FEATURE_HUMIDITY,
    //MaximumMain = SENSORS_FEATURE_MAX_MAIN,
    VoltageID = SENSORS_FEATURE_VID,
    Intrusion = SENSORS_FEATURE_INTRUSION,
    //MaximumOther = SENSORS_FEATURE_MAX_OTHER,
    BeepEnable = SENSORS_FEATURE_BEEP_ENABLE,
    // Maximum = SENSORS_FEATURE_MAX,
    Unknown = SENSORS_FEATURE_UNKNOWN,
}

impl Kind {
    /// Return an instance from one of the `SENSORS_FEATURE_*` values,
    /// *e.g.,* [`SENSORS_FEATURE_TEMP`].
    pub fn from_raw(kind: c_uint) -> Option<Self> {
        Self::try_from(kind).ok()
    }

    /// Return one of the `SENSORS_FEATURE_*` values
    /// (*e.g.,* [`SENSORS_FEATURE_TEMP`]) equivalent to this instance.
    pub fn as_raw(self) -> c_uint {
        self.into()
    }
}

impl Default for Kind {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Voltage => write!(f, "Voltage"),
            Self::Fan => write!(f, "Fan"),
            Self::Temperature => write!(f, "Temperature"),
            Self::Power => write!(f, "Power"),
            Self::Energy => write!(f, "Energy"),
            Self::Current => write!(f, "Current"),
            Self::Humidity => write!(f, "Humidity"),
            Self::VoltageID => write!(f, "VoltageID"),
            Self::Intrusion => write!(f, "Intrusion"),
            Self::BeepEnable => write!(f, "BeepEnable"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
