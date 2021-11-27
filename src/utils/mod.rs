#[cfg(test)]
mod tests;

use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr};
use std::fs::File;
use std::io;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_int};
use std::path::Path;
use std::ptr::NonNull;
use std::sync::atomic;
use std::sync::{Mutex, Once};

use sensors_sys::*;

use crate::errors::{Error, Result};

pub(crate) fn api_access_lock() -> &'static Mutex<()> {
    static INIT: Once = Once::new();
    static mut LOCK: MaybeUninit<Mutex<()>> = MaybeUninit::uninit();

    INIT.call_once(|| unsafe {
        LOCK.write(Mutex::new(()));
    });

    unsafe { LOCK.assume_init_ref() }
}

type ParseErrorProc = unsafe extern "C" fn(err: *const c_char, line_no: c_int);
type ParseErrorWFnProc =
    unsafe extern "C" fn(err: *const c_char, file_name: *const c_char, line_no: c_int);
type FatalErrorProc = unsafe extern "C" fn(procedure: *const c_char, err: *const c_char);

#[derive(Debug)]
pub(crate) struct CallBacks {
    parse_error: Option<ParseErrorProc>,
    parse_error_wfn: Option<ParseErrorWFnProc>,
    fatal_error: Option<FatalErrorProc>,
}

impl CallBacks {
    pub(crate) fn new(
        parse_error: ParseErrorProc,
        parse_error_wfn: ParseErrorWFnProc,
        fatal_error: FatalErrorProc,
    ) -> Self {
        Self {
            parse_error: Some(parse_error),
            parse_error_wfn: Some(parse_error_wfn),
            fatal_error: Some(fatal_error),
        }
    }

    pub(crate) unsafe fn set(&self) {
        atomic::fence(atomic::Ordering::Acquire);

        sensors_parse_error = self.parse_error;
        sensors_parse_error_wfn = self.parse_error_wfn;
        sensors_fatal_error = self.fatal_error;

        atomic::fence(atomic::Ordering::Release);
    }

    pub(crate) unsafe fn replace(self) -> Self {
        atomic::fence(atomic::Ordering::Acquire);

        let previous = Self {
            parse_error: sensors_parse_error,
            parse_error_wfn: sensors_parse_error_wfn,
            fatal_error: sensors_fatal_error,
        };

        sensors_parse_error = self.parse_error;
        sensors_parse_error_wfn = self.parse_error_wfn;
        sensors_fatal_error = self.fatal_error;

        atomic::fence(atomic::Ordering::Release);

        previous
    }
}

#[derive(Debug)]
pub(crate) struct LibCFileStream(NonNull<libc::FILE>);

impl LibCFileStream {
    pub(crate) fn from_path(path: &Path) -> Result<Self> {
        let c_config_file = c_string_from_path(path)?;
        let fp = unsafe { libc::fopen(c_config_file.as_ptr(), "r\0".as_ptr().cast()) };

        let result = NonNull::new(fp).map(Self).ok_or_else(|| {
            let err = io::Error::last_os_error();
            Error::from_io_path("fopen()", path, err)
        })?;

        if result.refers_to_dir(path)? {
            let err = io::Error::from_raw_os_error(libc::EISDIR);
            Err(Error::from_io_path("fopen()", path, err))
        } else {
            Ok(result)
        }
    }

    #[cfg(unix)]
    pub(crate) fn from_file(file: File) -> Result<Self> {
        use std::os::unix::io::IntoRawFd;

        let md = file
            .metadata()
            .map_err(|r| Error::from_io("std::fs::File::metadata()", r))?;

        if md.is_dir() {
            let err = io::Error::from_raw_os_error(libc::EISDIR);
            return Err(Error::from_io("fdopen()", err));
        }

        let fd = file.into_raw_fd();
        let fp = unsafe { libc::fdopen(fd, "r\0".as_ptr().cast()) };

        NonNull::new(fp)
            .map(Self)
            .ok_or_else(|| Error::from_io("fdopen()", io::Error::last_os_error()))
    }

    fn refers_to_dir(&self, path: &Path) -> Result<bool> {
        let mut st = MaybeUninit::zeroed();
        if unsafe { libc::fstat(libc::fileno(self.0.as_ptr()), st.as_mut_ptr()) } == -1 {
            let err = io::Error::last_os_error();
            Err(Error::from_io_path("fstat()", path, err))
        } else {
            let st = unsafe { st.assume_init() };
            Ok((st.st_mode & libc::S_IFMT) == libc::S_IFDIR)
        }
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut libc::FILE {
        self.0.as_ptr()
    }
}

impl Drop for LibCFileStream {
    fn drop(&mut self) {
        unsafe { libc::fclose(self.0.as_ptr()) };
    }
}

pub(crate) fn lossy_string_from_c_str(s: *const c_char, default: &str) -> Cow<str> {
    if s.is_null() {
        Cow::Borrowed(default)
    } else {
        unsafe { CStr::from_ptr(s) }.to_string_lossy()
    }
}

pub(crate) fn str_from_c_str<'t>(s: *const c_char) -> Option<&'t str> {
    if s.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(s) }.to_str().ok()
    }
}

#[cfg(unix)]
pub(crate) fn path_from_c_str<'t>(s: *const c_char) -> Option<&'t Path> {
    use std::os::unix::ffi::OsStrExt;

    if s.is_null() {
        None
    } else {
        let c_str = unsafe { CStr::from_ptr(s) };
        Some(Path::new(OsStr::from_bytes(c_str.to_bytes())))
    }
}

#[cfg(unix)]
fn c_string_from_path(p: &Path) -> Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    CString::new(p.as_os_str().as_bytes()).map_err(Into::into)
}
