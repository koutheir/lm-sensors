//! Errors.

use std::os::raw::{c_char, c_int};
use std::path::{Path, PathBuf};
use std::sync::atomic::{self, AtomicPtr};
use std::{cmp, fmt, io, process, ptr};

use crate::utils::*;

/// Result of a fallible function.
pub type Result<T> = std::result::Result<T, Error>;

/// Error of a failed function.
#[allow(missing_docs)] // Enum variant names are self-explanatory.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{operation} failed: [{number}] {description}")]
    LMSensors {
        operation: &'static str,
        number: c_int,
        description: String,
    },

    #[error("{operation} failed")]
    IO {
        operation: &'static str,
        source: io::Error,
    },

    #[error("{operation} failed on '{path}'")]
    IO1Path {
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    },

    #[error("Path {0} is not valid UTF-8")]
    PathIsNotUTF8(PathBuf),

    #[error(transparent)]
    PoisonedLMSensors(#[from] std::sync::PoisonError<std::sync::MutexGuard<'static, ()>>),

    #[error(transparent)]
    UnexpectedNul(#[from] std::ffi::NulError),

    #[error(transparent)]
    InvalidUTF8CString(#[from] std::ffi::IntoStringError),

    #[error(transparent)]
    InvalidUTF8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    NotInteger(#[from] std::num::ParseIntError),
}

impl Error {
    pub(crate) fn from_io(operation: &'static str, source: io::Error) -> Self {
        Error::IO { operation, source }
    }

    pub(crate) fn from_io_path(
        operation: &'static str,
        path: impl Into<PathBuf>,
        source: io::Error,
    ) -> Self {
        Error::IO1Path {
            source,
            operation,
            path: path.into(),
        }
    }

    pub(crate) fn from_lm_sensors(operation: &'static str, number: c_int) -> Self {
        // SAFETY:
        // We assume `sensors_strerror()` can be called anytime,
        // including before `sensors_init()` and after `sensors_cleanup()`.
        let description =
            lossy_string_from_c_str(unsafe { sensors_sys::sensors_strerror(number) }, "")
                .into_owned();

        Error::LMSensors {
            operation,
            number: c_int::abs(number),
            description,
        }
    }
}

/// Listener for fatal errors reported by LM sensors.
pub trait Listener: fmt::Debug {
    /// This function is called when a configuration parsing error happens.
    fn on_lm_sensors_config_error(&self, error: &str, file_name: Option<&Path>, line_number: usize);

    /// This function is called when a fatal error happens,
    /// *e.g.,* an out of memory situation.
    ///
    /// # Warning
    ///
    /// Due to requirements of the LM sensors library, this process is aborted
    /// after this function returns.
    fn on_lm_sensors_fatal_error(&self, error: &str, procedure: &str);
}

#[derive(Debug)]
pub(crate) struct DefaultListener;

impl Listener for DefaultListener {
    fn on_lm_sensors_config_error(
        &self,
        error: &str,
        file_name: Option<&Path>,
        line_number: usize,
    ) {
        if let Some(file_name) = file_name {
            eprintln!(
                "[ERROR] lm-sensors configuration: {}, at file '{}' line {}.",
                error,
                file_name.display(),
                line_number
            );
        } else {
            eprintln!(
                "[ERROR] lm-sensors configuration: {}, at line {}.",
                error, line_number
            );
        }
    }

    fn on_lm_sensors_fatal_error(&self, error: &str, procedure: &str) {
        eprintln!(
            "[FATAL] lm-sensors: {}, at procedure '{}'.",
            error, procedure
        );
    }
}

static ERROR_LISTENER: AtomicPtr<Box<dyn Listener>> = AtomicPtr::new(ptr::null_mut());

#[derive(Debug)]
pub(crate) struct Reporter {
    previous_error_listener: *mut Box<dyn Listener>,
    previous_call_backs: CallBacks,
}

impl Reporter {
    pub(crate) fn new(error_listener: *mut Box<dyn Listener>) -> Self {
        let call_backs =
            CallBacks::new(Self::parse_error, Self::parse_error_wfn, Self::fatal_error);

        let previous_error_listener = ERROR_LISTENER.swap(error_listener, atomic::Ordering::AcqRel);
        let previous_call_backs = unsafe { call_backs.replace() };

        Self {
            previous_error_listener,
            previous_call_backs,
        }
    }

    pub(crate) fn restore(&self) -> *mut Box<dyn Listener> {
        unsafe { self.previous_call_backs.set() };
        ERROR_LISTENER.swap(self.previous_error_listener, atomic::Ordering::AcqRel)
    }

    extern "C" fn parse_error(err: *const c_char, line_number: c_int) {
        Self::parse_error_wfn(err, ptr::null(), line_number);
    }

    extern "C" fn parse_error_wfn(err: *const c_char, file_name: *const c_char, line_no: c_int) {
        let error = lossy_string_from_c_str(err, "<unknown-error>");
        let file_name = path_from_c_str(file_name);
        let line_number = cmp::max(line_no, 1) as usize;

        let listener = unsafe { ERROR_LISTENER.load(atomic::Ordering::Acquire).as_ref() }
            .map_or(&crate::errors::DefaultListener as &dyn Listener, |v| &**v);

        listener.on_lm_sensors_config_error(&error, file_name, line_number);
    }

    extern "C" fn fatal_error(procedure: *const c_char, err: *const c_char) {
        let procedure = str_from_c_str(procedure).unwrap_or("<unknown-procedure>");
        let error = lossy_string_from_c_str(err, "<unknown-error>");

        let listener = unsafe { ERROR_LISTENER.load(atomic::Ordering::Acquire).as_ref() }
            .map_or(&crate::errors::DefaultListener as &dyn Listener, |v| &**v);

        listener.on_lm_sensors_fatal_error(&error, procedure);
        process::abort();
    }
}
