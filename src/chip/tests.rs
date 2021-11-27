#![cfg(test)]

use std::mem::MaybeUninit;

use serial_test::serial;

#[test]
#[serial]
fn new() {
    use crate::prelude::*;

    let s = crate::Initializer::default().initialize().unwrap();
    s.new_chip("").unwrap_err();
    let chip0 = s.new_chip("*-isa-*").unwrap();

    let mut state = 0;
    let chip1 = unsafe {
        let chip_ref = sensors_sys::sensors_get_detected_chips(chip0.as_ref().as_ref(), &mut state)
            .as_ref()
            .unwrap();
        s.new_chip_ref(chip_ref)
    };

    let _name = chip1.name().unwrap();
    let c_name = chip1.raw_name().unwrap();
    assert!(!chip1.to_string().is_empty());

    let mut sensors_chip_name = MaybeUninit::zeroed();
    let mut chip2 = unsafe {
        let r = sensors_sys::sensors_parse_chip_name(
            c_name.as_ptr().cast(),
            sensors_chip_name.as_mut_ptr(),
        );
        assert_eq!(r, 0);
        s.new_raw_chip(sensors_chip_name.assume_init())
    };
    chip2.bus_mut().set_raw_number(0); // Workaround for libsensors.
    let _name = chip2.name().unwrap();
    assert!(!chip2.to_string().is_empty());
}

#[test]
#[serial]
fn raw() {
    use crate::prelude::*;

    let s = crate::Initializer::default().initialize().unwrap();
    let chip0 = s.new_chip("*-isa-*").unwrap();

    assert!(chip0.raw_prefix().is_none());
    assert!(chip0.prefix().is_none());

    let _address = chip0.raw_address();
    assert!(chip0.address().is_none());

    chip0.raw_name().unwrap_err();
    chip0.name().unwrap_err();

    assert!(chip0.raw_path().is_none());
    assert!(chip0.path().is_none());

    let _b0 = chip0.bus();

    drop(unsafe { s.new_raw_chip(chip0.into_raw_parts()) });

    let mut iter = s.chip_iter(None);
    let chip1 = iter.next().unwrap();

    assert!(!chip1.raw_prefix().unwrap().to_bytes().is_empty());
    assert!(!chip1.prefix().unwrap().unwrap().is_empty());

    let _address = chip1.raw_address();
    let _address = chip1.address().unwrap();

    let _name = chip1.raw_name().unwrap();
    let _name = chip1.name().unwrap();

    assert!(!chip1.raw_path().unwrap().to_bytes().is_empty());
    assert!(!chip1.path().unwrap().as_os_str().is_empty());

    let _b1 = chip1.bus();

    let chip2 = iter.next().unwrap();
    assert_ne!(chip1, chip2);

    let chip3 = s.new_chip("*-isa-*").unwrap();
    assert_ne!(chip2, chip3);
    assert_ne!(chip3, chip2);
}

#[test]
#[serial]
fn do_chip_sets() {
    use crate::prelude::*;

    let s = crate::Initializer::default().initialize().unwrap();
    let chip0 = s.chip_iter(None).next().unwrap();
    chip0.do_chip_sets().unwrap();
}

#[test]
#[serial]
fn iter() {
    let s = crate::Initializer::default().initialize().unwrap();

    let c0 = s.chip_iter(None).count();
    assert!(c0 > 0);

    let chip0 = s.new_chip("*-isa-*").unwrap();
    let c1 = s.chip_iter(Some(chip0.as_ref())).count();
    assert!(c1 > 0);

    assert!(c0 >= c1);
}

#[test]
#[serial]
fn feature_iter() {
    use crate::prelude::*;

    let s = crate::Initializer::default().initialize().unwrap();

    let chip0 = s.new_chip("*-isa-*").unwrap();
    let chip1 = s.chip_iter(None).next().unwrap();

    let _count1 = chip0.feature_iter().count();
    let _count2 = chip1.feature_iter().count();
}
