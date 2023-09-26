#![doc = include_str!("../README.md")]
#![warn(unsafe_op_in_unsafe_fn, missing_docs)]
/*
#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::wildcard_imports,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::question_mark_used,
    clippy::single_char_lifetime_names,
    clippy::min_ident_chars,
    clippy::single_call_fn,
    clippy::too_many_lines,
    clippy::print_stderr
)]
*/

pub mod bus;
pub mod chip;
pub mod errors;
pub mod feature;
pub mod sub_feature;
mod utils;
pub mod value;

#[cfg(test)]
mod tests;

use core::ffi::CStr;
use core::marker::PhantomData;
use core::ptr;
use core::sync::atomic;
use core::sync::atomic::AtomicBool;
use std::fs::File;
use std::io;
use std::os::raw::c_short;
use std::path::PathBuf;

use sensors_sys::*;

use crate::errors::{Error, Listener, Reporter, Result};
use crate::utils::{api_access_lock, LibCFileStream};

pub use crate::bus::Bus;
pub use crate::chip::{Chip, ChipRef};
pub use crate::feature::FeatureRef;
pub use crate::sub_feature::SubFeatureRef;
pub use crate::value::Value;

/// LM sensors library initializer, producing an instance of [`LMSensors`].
#[derive(Debug, Default)]
pub struct Initializer {
    error_listener: Option<Box<dyn Listener>>,
    config_path: Option<PathBuf>,
    config_file: Option<File>,
}

/// LM sensors library instance, producing instances of [`Chip`]s, [`Bus`]es, etc.
#[derive(Debug)]
pub struct LMSensors {
    error_reporter: Reporter,
}

impl Initializer {
    /**
    Set the path of the configuration file to be read during LM sensors
    library initialization.

    # Example

    ```rust
    let sensors = lm_sensors::Initializer::default()
        .config_path("/dev/null")
        .initialize()?;
    # Ok::<(), lm_sensors::errors::Error>(())
    ```
    */
    #[must_use]
    pub fn config_path(self, path: impl Into<PathBuf>) -> Self {
        Self {
            error_listener: self.error_listener,
            config_path: Some(path.into()),
            config_file: None,
        }
    }

    /**
    Set the configuration contents to be used during LM sensors
    library initialization.

    # Example

    ```rust
    # use std::fs::File;
    let config_file = File::open("/dev/null").unwrap();
    let sensors = lm_sensors::Initializer::default()
        .config_file(config_file)
        .initialize()?;
    # Ok::<(), lm_sensors::errors::Error>(())
    ```
    */
    #[must_use]
    pub fn config_file(self, file: File) -> Self {
        Self {
            error_listener: self.error_listener,
            config_path: None,
            config_file: Some(file),
        }
    }

    /**
    Set the error listener to be used during LM sensors library initialization.

    # Example

    ```rust
    #[derive(Debug)]
    struct EL;

    impl lm_sensors::errors::Listener for EL {
        fn on_lm_sensors_config_error(&self, error: &str,
            file_name: Option<&std::path::Path>, line_number: usize)
        {
            if let Some(file_name) = file_name {
                eprintln!("[ERROR] lm-sensors config: {} @{}:{}",
                          error, file_name.display(), line_number);
            } else {
                eprintln!("[ERROR] lm-sensors config: {} @<config>:{}",
                          error, line_number);
            }
        }

        fn on_lm_sensors_fatal_error(&self, error: &str, procedure: &str) {
            eprintln!("[FATAL] lm-sensors: {} @{}", error, procedure);
        }
    }

    let sensors = lm_sensors::Initializer::default()
        .error_listener(Box::new(EL))
        .initialize()?;
    # Ok::<(), lm_sensors::errors::Error>(())
    ```
    */
    #[must_use]
    pub fn error_listener(self, listener: Box<dyn Listener>) -> Self {
        Self {
            error_listener: Some(listener),
            config_path: self.config_path,
            config_file: self.config_file,
        }
    }

    /**
    Return an instance of a loaded and initialized LM sensors library.

    # Example

    ```rust
    let sensors = lm_sensors::Initializer::default().initialize()?;
    # Ok::<(), lm_sensors::errors::Error>(())
    ```
    */
    pub fn initialize(self) -> Result<LMSensors> {
        let config_file_fp = match (self.config_path, self.config_file) {
            (None, None) => None,
            (None, Some(config_file)) => LibCFileStream::from_file(config_file).map(Some)?,
            (Some(config_path), None) => LibCFileStream::from_path(&config_path).map(Some)?,
            _ => unreachable!(),
        };

        let error_listener = self
            .error_listener
            .map_or_else(ptr::null_mut, |v| Box::into_raw(Box::new(v)));

        let result = LMSensors::new(config_file_fp, error_listener);

        if result.is_err() && !error_listener.is_null() {
            // Safety: error_listener was allocated locally and is now unused.
            drop(unsafe { Box::from_raw(error_listener) });
        }
        result
    }
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

impl LMSensors {
    /// Returns the version of the LM sensors library,
    /// if available and valid UTF-8.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.raw_version().and_then(|s| s.to_str().ok())
    }

    /// Returns the raw version of the LM sensors library, if available.
    #[must_use]
    pub fn raw_version(&self) -> Option<&CStr> {
        // Safety: `libsensors_version` has already been initialized, and is now constant.
        let version = unsafe { libsensors_version };
        // Safety: if `libsensors_version` is not null, then it is assumed to be a null-terminated
        // string.
        (!version.is_null()).then(|| unsafe { CStr::from_ptr(version) })
    }

    /// Return a new instance of [`ChipRef`], given a shared reference
    /// to a raw chip.
    ///
    /// # Safety
    ///
    /// - The given [`sensors_chip_name`] reference must have been returned from
    ///   [`sensors_get_detected_chips`].
    #[must_use]
    pub unsafe fn new_chip_ref<'a>(&'a self, chip: &'a sensors_chip_name) -> ChipRef<'a> {
        ChipRef(chip)
    }

    /// Return a new instance of [`Chip`], given a raw chip.
    ///
    /// # Safety
    ///
    /// - The given [`sensors_chip_name`] must have been previously initialized
    ///   by calling [`sensors_parse_chip_name`].
    #[must_use]
    pub unsafe fn new_raw_chip(&'_ self, chip: sensors_chip_name) -> Chip<'_> {
        Chip {
            raw: chip,
            _phantom: &PhantomData,
        }
    }

    /// Return a new instance of [`Chip`], given a chip name.
    pub fn new_chip<'a>(&'a self, name: &str) -> Result<Chip<'a>> {
        Chip::new(name)
    }

    /// Return a new instance of [`Bus`], given a raw *(bus type, bus number)*.
    #[must_use]
    pub fn new_raw_bus(&self, kind: c_short, number: c_short) -> Bus {
        Bus(sensors_bus_id {
            type_: kind,
            nr: number,
        })
    }

    /// Return a new instance of [`Bus`], given a *(bus type, bus number)*.
    #[must_use]
    pub fn new_bus(&self, kind: bus::Kind, number: bus::Number) -> Bus {
        Bus(sensors_bus_id {
            type_: c_short::from(kind),
            nr: number.into(),
        })
    }

    /// Return a new default instance of [`Bus`].
    #[must_use]
    pub fn default_bus(&self) -> Bus {
        Bus(sensors_bus_id {
            type_: SENSORS_BUS_TYPE_ANY as c_short,
            nr: SENSORS_BUS_NR_ANY as c_short,
        })
    }

    /// Return a new instance of [`FeatureRef`] given a shared reference
    /// to a raw feature.
    ///
    /// # Safety
    ///
    /// - The given [`sensors_feature`] reference must have been returned from
    ///   [`sensors_get_features`].
    #[must_use]
    pub unsafe fn new_feature_ref<'a>(
        &'a self,
        chip: ChipRef<'a>,
        raw: &'a sensors_feature,
    ) -> FeatureRef<'a> {
        FeatureRef { chip, raw }
    }

    /// Return a new instance of [`SubFeatureRef`] given a shared reference
    /// to a raw sub-feature.
    ///
    /// # Safety
    ///
    /// - The given [`sensors_subfeature`] reference must have been returned
    ///   either from [`sensors_get_all_subfeatures`] or from
    ///   [`sensors_get_subfeature`].
    #[must_use]
    pub unsafe fn new_sub_feature_ref<'a>(
        &'a self,
        feature: FeatureRef<'a>,
        raw: &'a sensors_subfeature,
    ) -> SubFeatureRef<'a> {
        SubFeatureRef { feature, raw }
    }

    /// Return an iterator which yields all chips matching the given pattern.
    ///
    /// Specifying `None` for the `match_pattern` yields all chips.
    pub fn chip_iter<'a>(&'a self, match_pattern: Option<ChipRef<'a>>) -> crate::chip::Iter<'a> {
        crate::chip::Iter {
            state: 0,
            match_pattern,
        }
    }

    /// See: [`sensors_init`].
    fn new(
        config_file_stream: Option<LibCFileStream>,
        error_listener: *mut Box<dyn Listener>,
    ) -> Result<Self> {
        let config_file_fp = config_file_stream
            .as_ref()
            .map_or(ptr::null_mut(), LibCFileStream::as_mut_ptr);

        let locked_self = api_access_lock().lock()?;

        if INITIALIZED.load(atomic::Ordering::Acquire) {
            drop(locked_self); // Unlock early.

            let err = io::ErrorKind::AlreadyExists.into();
            return Err(Error::from_io("sensors_init()", err));
        }

        // We're creating the only instance.
        let error_reporter = Reporter::new(error_listener);

        // Safety: this is assumed to be safe.
        let r = unsafe { sensors_init(config_file_fp.cast()) };
        if r == 0 {
            INITIALIZED.store(true, atomic::Ordering::Release);

            return Ok(Self { error_reporter });
        }

        // sensors_init() failed.
        // Restore previous global state.
        error_reporter.restore();

        drop(locked_self); // Unlock early.

        Err(Error::from_lm_sensors("sensors_init()", r))
    }
}

impl Drop for LMSensors {
    /// See: [`sensors_cleanup`].
    fn drop(&mut self) {
        let error_listener = api_access_lock()
            .lock()
            .map(|_guard| {
                // Safety: this is assumed to be safe.
                unsafe { sensors_cleanup() }

                let error_listener = self.error_reporter.restore();

                INITIALIZED.store(false, atomic::Ordering::Release);

                error_listener
            })
            .unwrap_or(ptr::null_mut());

        if !error_listener.is_null() {
            // Safety: error_listener was allocated before and is now unused.
            drop(unsafe { Box::from_raw(error_listener) });
        }
    }
}
