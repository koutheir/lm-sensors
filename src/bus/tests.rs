#![cfg(test)]

use std::os::raw::c_short;

use sensors_sys::*;
use serial_test::serial;

#[test]
#[serial]
fn new() {
    let s = crate::Initializer::default().initialize().unwrap();
    let b0 = s.default_bus();
    assert_eq!(b0.to_string(), "�");

    let b1 = s.new_bus(super::Kind::Any, super::Number::Any);
    assert_eq!(b0, b1);

    let b2 = s.new_bus(super::Kind::PCI, super::Number::Number(42));
    assert_ne!(b0, b2);

    let b3 = s.new_raw_bus(SENSORS_BUS_TYPE_PCI as c_short, 42);
    assert_eq!(b2, b3);

    let b3 = s.new_raw_bus(SENSORS_BUS_TYPE_PCI as c_short, 41);
    assert_ne!(b2, b3);

    let b3 = s.new_raw_bus(SENSORS_BUS_TYPE_I2C as c_short, 42);
    assert_ne!(b2, b3);

    let raw1 = sensors_bus_id {
        type_: SENSORS_BUS_TYPE_PCI as c_short,
        nr: 42,
    };
    let _b4 = s.new_bus_ref(&raw1);

    let mut raw1 = sensors_bus_id {
        type_: SENSORS_BUS_TYPE_PCI as c_short,
        nr: 42,
    };
    let _b4 = s.new_bus_mut(&mut raw1);

    let mut raw0 = sensors_bus_id {
        type_: SENSORS_BUS_TYPE_ANY as c_short,
        nr: SENSORS_BUS_NR_ANY as c_short,
    };
    let b5 = s.new_bus_ref(&raw0);
    assert_eq!(b5.to_string(), "�");

    let b5 = s.new_bus_mut(&mut raw0);
    assert_eq!(b5.to_string(), "�");
}

#[test]
#[serial]
fn raw() {
    use crate::prelude::*;

    let s = crate::Initializer::default().initialize().unwrap();

    let mut b0 = s.new_bus(super::Kind::PCI, super::Number::Number(42));
    let _name = b0.raw_name().unwrap();
    let _kind = b0.raw_kind();
    let _number = b0.raw_number();
    b0.set_raw_number(41);
    b0.set_raw_kind(SENSORS_BUS_TYPE_PCI as c_short);

    let b1 = s.new_bus_ref(b0.as_ref());
    let _name = b1.raw_name().unwrap();
    let _kind = b1.raw_kind();
    let _number = b1.raw_number();

    assert_eq!(b0, b1);
    assert_eq!(b1, b0);

    let mut b0 = s.new_bus(super::Kind::PCI, super::Number::Number(42));

    let mut b2 = s.new_bus_mut(b0.as_mut());
    let _name = b2.raw_name().unwrap();
    let _kind = b2.raw_kind();
    let _number = b2.raw_number();
    b2.set_raw_number(41);
    b2.set_raw_kind(SENSORS_BUS_TYPE_PCI as c_short);

    assert_eq!(b1, b2);
    assert_eq!(b2, b1);

    let b0 = s.new_bus(super::Kind::PCI, super::Number::Number(41));

    assert_eq!(b0, b2);
    assert_eq!(b2, b0);

    let b3 = s.new_bus(super::Kind::Any, super::Number::Any);
    let _name = b3.raw_name().unwrap_err();
    let _kind = b3.raw_kind();
    let _number = b3.raw_number();
}

#[test]
#[serial]
fn bus() {
    use crate::prelude::*;

    let s = crate::Initializer::default().initialize().unwrap();

    let mut b0 = s.new_bus(super::Kind::PCI, super::Number::Number(42));
    let _name = b0.name().unwrap();
    let _kind = b0.kind().unwrap();
    let _number = b0.number();
    assert!(!b0.to_string().is_empty());
    b0.set_number(super::Number::Number(41));
    b0.set_kind(super::Kind::PCI);

    let b1 = s.new_bus_ref(b0.as_ref());
    let _name = b1.name().unwrap();
    let _kind = b1.kind().unwrap();
    let _number = b1.number();
    assert!(!b1.to_string().is_empty());

    let mut b1 = s.new_bus_mut(b0.as_mut());
    let _name = b1.name().unwrap();
    let _kind = b1.kind().unwrap();
    let _number = b1.number();
    b1.set_number(super::Number::Number(42));
    b1.set_kind(super::Kind::PCI);
    assert!(!b1.to_string().is_empty());

    let b2 = s.new_bus(super::Kind::Any, super::Number::Any);
    let _name = b2.name().unwrap_err();
    let _kind = b2.kind().unwrap();
    let _number = b2.number();
}

#[test]
fn number() {
    use super::Number;
    use {SENSORS_BUS_NR_ANY, SENSORS_BUS_NR_IGNORE};

    let n0 = Number::default();
    assert_eq!(n0, Number::Any);
    assert_eq!(n0, Number::from(SENSORS_BUS_NR_ANY as c_short));
    assert_eq!(c_short::from(n0), SENSORS_BUS_NR_ANY as c_short);
    assert_eq!(n0.to_string(), "Any");

    let n1 = Number::Ignore;
    assert_ne!(n0, n1);
    assert_eq!(n1, Number::from(SENSORS_BUS_NR_IGNORE as c_short));
    assert_eq!(c_short::from(n1), SENSORS_BUS_NR_IGNORE as c_short);
    assert_eq!(n1.to_string(), "Ignore");

    let n2 = Number::Number(42);
    assert_ne!(n0, n2);
    assert_eq!(n2, Number::from(42));
    assert_eq!(n2.to_string(), "42");
}

#[test]
fn kind() {
    use super::Kind;

    let k0 = Kind::default();
    assert_eq!(k0, Kind::Any);
    assert_ne!(k0, Kind::I2C);
    assert!(Kind::from_raw(c_short::MAX).is_none());

    for (k, n, s) in [
        (Kind::Any, SENSORS_BUS_TYPE_ANY, "Any"),
        (
            Kind::I2C,
            SENSORS_BUS_TYPE_I2C,
            "Inter-Integrated Circuit (I2C)",
        ),
        (
            Kind::ISA,
            SENSORS_BUS_TYPE_ISA,
            "Industry Standard Architecture (ISA)",
        ),
        (
            Kind::PCI,
            SENSORS_BUS_TYPE_PCI,
            "Peripheral Component Interconnect (PCI)",
        ),
        (
            Kind::SPI,
            SENSORS_BUS_TYPE_SPI,
            "Serial Peripheral Interface (SPI)",
        ),
        (Kind::Virtual, SENSORS_BUS_TYPE_VIRTUAL, "Virtual"),
        (
            Kind::ACPI,
            SENSORS_BUS_TYPE_ACPI,
            "Advanced Configuration and Power Interface (ACPI)",
        ),
        (
            Kind::HID,
            SENSORS_BUS_TYPE_HID,
            "Human Interface Device (HID)",
        ),
        (
            Kind::MDIO,
            SENSORS_BUS_TYPE_MDIO,
            "Management Data Input/Output (MDIO)",
        ),
        (
            Kind::SCSI,
            SENSORS_BUS_TYPE_SCSI,
            "Small Computer System Interface (SCSI)",
        ),
    ] {
        assert_eq!(Kind::from_raw(n as c_short).unwrap(), k);
        assert_eq!(n as c_short, k.as_raw());
        assert_eq!(k.to_string(), s);
    }
}
