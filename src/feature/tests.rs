#![cfg(test)]

use sensors_sys::sensors_feature_type::*;
use serial_test::serial;

#[test]
#[serial]
fn new_feature_ref() {
    let s = crate::Initializer::default().initialize().unwrap();
    let chip0 = s.new_chip("*-isa-*").unwrap();

    let mut state = 0;
    let feature = unsafe {
        sensors_sys::sensors_get_features(chip0.as_ref().as_ref(), &mut state)
            .as_ref()
            .unwrap()
    };
    let _feature_ref = unsafe { s.new_feature_ref(chip0.as_ref(), feature) };
}

#[test]
#[serial]
fn feature_iter() {
    use crate::prelude::*;

    let s = crate::Initializer::default().initialize().unwrap();

    let mut iter = s.chip_iter(None);
    let chip0 = iter.next().unwrap();

    let feature0 = chip0.feature_iter().next().unwrap();

    assert_eq!(feature0.chip(), chip0);
    assert!(!feature0.raw_label().unwrap().to_bytes().is_empty());
    assert!(!feature0.raw_name().unwrap().to_bytes().is_empty());
    let _kind = feature0.raw_kind();
    assert!(!feature0.to_string().is_empty());
    assert!(!format!("{:?}", feature0).is_empty());

    assert!(!feature0.label().unwrap().is_empty());
    assert!(!feature0.name().unwrap().unwrap().is_empty());
    let _kind = feature0.kind();
    let _number = feature0.number();

    let chip1 = iter.next().unwrap();

    let feature1 = chip1.feature_iter().next().unwrap();
    assert_ne!(feature0, feature1);
}

#[test]
#[serial]
fn sub_feature_by_kind() {
    use crate::prelude::*;

    let s = crate::Initializer::default().initialize().unwrap();

    let mut iter = s.chip_iter(None);
    let chip0 = iter.next().unwrap();

    let feature0 = chip0.feature_iter().next().unwrap();

    let sub_feature = feature0.sub_feature_iter().next().unwrap();
    let _ignored = sub_feature.raw_kind();

    let sub_feature0 = feature0
        .sub_feature_by_raw_kind(sub_feature.raw_kind())
        .unwrap();
    let sub_feature1 = feature0
        .sub_feature_by_kind(sub_feature.kind().unwrap())
        .unwrap();
    assert_eq!(sub_feature0, sub_feature1);
}

#[test]
fn kind() {
    use super::Kind;

    let k0 = Kind::default();
    assert_eq!(k0, Kind::Unknown);
    assert_eq!(k0.as_raw(), SENSORS_FEATURE_UNKNOWN);
    assert!(!k0.to_string().is_empty());

    for (k, n, s) in [
        (Kind::Voltage, SENSORS_FEATURE_IN, "Voltage"),
        (Kind::Fan, SENSORS_FEATURE_FAN, "Fan"),
        (Kind::Temperature, SENSORS_FEATURE_TEMP, "Temperature"),
        (Kind::Power, SENSORS_FEATURE_POWER, "Power"),
        (Kind::Energy, SENSORS_FEATURE_ENERGY, "Energy"),
        (Kind::Current, SENSORS_FEATURE_CURR, "Current"),
        (Kind::Humidity, SENSORS_FEATURE_HUMIDITY, "Humidity"),
        (Kind::VoltageID, SENSORS_FEATURE_VID, "VoltageID"),
        (Kind::Intrusion, SENSORS_FEATURE_INTRUSION, "Intrusion"),
        (Kind::BeepEnable, SENSORS_FEATURE_BEEP_ENABLE, "BeepEnable"),
        (Kind::Unknown, SENSORS_FEATURE_UNKNOWN, "Unknown"),
    ] {
        assert_eq!(Kind::from_raw(n).unwrap(), k);
        assert_eq!(n, k.as_raw());
        assert_eq!(k.to_string(), s);
    }
}
