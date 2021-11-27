#![cfg(test)]

use std::fs::File;

use serial_test::serial;

#[test]
#[serial]
fn version() {
    let s = super::Initializer::default().initialize().unwrap();
    let v = s.raw_version();
    assert!(v.is_some());
    assert!(!v.unwrap().to_bytes().is_empty());
    assert!(!s.version().unwrap().is_empty());
}

#[test]
#[serial]
fn init_simple() {
    let _s = super::Initializer::default().initialize().unwrap();
}

#[test]
#[serial]
fn init_multiple_simultaneously() {
    let s0 = super::Initializer::default().initialize().unwrap();
    let _e0 = super::Initializer::default().initialize().unwrap_err();
    let _e1 = super::Initializer::default().initialize().unwrap_err();
    drop(s0);
    let _s1 = super::Initializer::default().initialize().unwrap();
}

#[test]
#[serial]
fn init_invalid_config_paths() {
    for &path in &["/inexistent-file", "/tmp", "/etc/shadow"] {
        let _e = super::Initializer::default()
            .config_path(path)
            .initialize()
            .unwrap_err();
    }
}

#[test]
#[serial]
fn init_empty_config_file() {
    let s = super::Initializer::default()
        .config_path("/dev/null")
        .initialize()
        .unwrap();
    drop(s);

    let _s = super::Initializer::default()
        .config_file(File::open("/dev/null").unwrap())
        .initialize()
        .unwrap();
}

#[test]
#[serial]
fn init_error_listener() {
    #[derive(Debug)]
    struct EL(u8);

    impl super::errors::Listener for EL {
        fn on_lm_sensors_config_error(
            &self,
            _error: &str,
            _file_name: Option<&std::path::Path>,
            _line_number: usize,
        ) {
            unreachable!()
        }

        fn on_lm_sensors_fatal_error(&self, _error: &str, _procedure: &str) {
            unreachable!()
        }
    }

    let s0 = super::Initializer::default()
        .config_path("/dev/null")
        .initialize()
        .unwrap();

    let _s1 = super::Initializer::default()
        .error_listener(Box::new(EL(42)))
        .initialize()
        .unwrap_err();

    drop(s0);

    let _s1 = super::Initializer::default()
        .error_listener(Box::new(EL(42)))
        .initialize()
        .unwrap();
}
