#![cfg(test)]

use std::borrow::Cow;
use std::fs::OpenOptions;
use std::path::Path;
use std::ptr;

#[test]
fn libcfilestream_from_file() {
    let dir = OpenOptions::new().read(true).open("/").unwrap();
    super::LibCFileStream::from_file(dir).unwrap_err();
}

#[test]
fn lossy_string_from_c_str() {
    assert_eq!(
        super::lossy_string_from_c_str(ptr::null(), ""),
        Cow::Borrowed("")
    );
    assert_eq!(
        super::lossy_string_from_c_str(ptr::null(), "xyz"),
        Cow::Borrowed("xyz")
    );
    assert_eq!(
        super::lossy_string_from_c_str("\0".as_ptr().cast(), "xyz"),
        Cow::Borrowed("")
    );
    assert_eq!(
        super::lossy_string_from_c_str("xyz\0".as_ptr().cast(), "xyz"),
        Cow::Borrowed("xyz")
    );
    assert_eq!(
        super::lossy_string_from_c_str([b'K', b'A', 255_u8, 255, 0].as_ptr().cast(), "xyz"),
        Cow::<str>::Owned(String::from("KA��"))
    );
}

#[test]
fn str_from_c_str() {
    assert!(super::str_from_c_str(ptr::null()).is_none());
    assert!(super::str_from_c_str([255_u8, 255, 0].as_ptr().cast()).is_none());
    assert!(super::str_from_c_str("\0".as_ptr().cast())
        .unwrap()
        .is_empty());
    assert_eq!(
        super::str_from_c_str("xyz/abc\0".as_ptr().cast()).unwrap(),
        "xyz/abc"
    );
}

#[test]
fn path_from_c_str() {
    assert!(super::path_from_c_str(ptr::null()).is_none());
    assert_eq!(
        super::path_from_c_str("\0".as_ptr().cast()).unwrap(),
        Path::new("")
    );
    assert_eq!(
        super::path_from_c_str("xyz/abc\0".as_ptr().cast()).unwrap(),
        Path::new("xyz").join("abc")
    );
}

#[test]
fn c_string_from_path() {
    assert_eq!(
        super::c_string_from_path(Path::new(""))
            .unwrap()
            .as_bytes_with_nul(),
        &[0_u8]
    );

    assert_eq!(
        super::c_string_from_path(&Path::new("xyz").join("abc"))
            .unwrap()
            .as_bytes_with_nul(),
        "xyz/abc\0".as_bytes()
    );
}
