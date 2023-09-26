#![cfg(test)]

use serial_test::serial;

#[test]
#[serial]
fn new_sub_feature_ref() {
    let s = crate::Initializer::default().initialize().unwrap();
    let chip0 = s.new_chip("*-isa-*").unwrap();

    let mut state = 0;
    let feature = unsafe {
        sensors_sys::sensors_get_features(chip0.raw_ref(), &mut state)
            .as_ref()
            .unwrap()
    };
    let feature_ref = unsafe { s.new_feature_ref(chip0.as_ref(), feature) };

    let sub_feature = unsafe {
        let mut state = 0;
        sensors_sys::sensors_get_all_subfeatures(chip0.raw_ref(), feature, &mut state)
            .as_ref()
            .unwrap()
    };
    let sub_feature_ref = unsafe { s.new_sub_feature_ref(feature_ref, sub_feature) };

    assert!(!sub_feature_ref.to_string().is_empty());
}

#[test]
#[serial]
fn feature_iter() {
    let s = crate::Initializer::default().initialize().unwrap();

    for chip in s.chip_iter(None) {
        let feature = chip.feature_iter().next().unwrap();
        let sub_feature = feature.sub_feature_iter().next().unwrap();

        assert_eq!(sub_feature.feature(), feature);
        assert!(!sub_feature.raw_name().unwrap().to_bytes().is_empty());
        assert!(!sub_feature.name().unwrap().unwrap().is_empty());
        let _flags = sub_feature.flags().unwrap();

        if let Ok(value) = sub_feature.raw_value() {
            drop(sub_feature.set_raw_value(value));
        }
    }
}
