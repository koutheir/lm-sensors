#![cfg(test)]

use std::os::raw::{c_int, c_uint};

use sensors_sys::sensors_subfeature_type::*;

use super::{Kind, Unit};

static KIND_LIST: [(Kind, c_uint, Unit, bool, &str); 87] = [
    (
        Kind::VoltageInput,
        SENSORS_SUBFEATURE_IN_INPUT,
        Unit::Volt,
        false,
        "VoltageInput",
    ),
    (
        Kind::VoltageMinimum,
        SENSORS_SUBFEATURE_IN_MIN,
        Unit::Volt,
        false,
        "VoltageMinimum",
    ),
    (
        Kind::VoltageMaximum,
        SENSORS_SUBFEATURE_IN_MAX,
        Unit::Volt,
        false,
        "VoltageMaximum",
    ),
    (
        Kind::VoltageLCritical,
        SENSORS_SUBFEATURE_IN_LCRIT,
        Unit::Volt,
        false,
        "VoltageLCritical",
    ),
    (
        Kind::VoltageCritical,
        SENSORS_SUBFEATURE_IN_CRIT,
        Unit::Volt,
        false,
        "VoltageCritical",
    ),
    (
        Kind::VoltageAverage,
        SENSORS_SUBFEATURE_IN_AVERAGE,
        Unit::Volt,
        false,
        "VoltageAverage",
    ),
    (
        Kind::VoltageLowest,
        SENSORS_SUBFEATURE_IN_LOWEST,
        Unit::Volt,
        false,
        "VoltageLowest",
    ),
    (
        Kind::VoltageHighest,
        SENSORS_SUBFEATURE_IN_HIGHEST,
        Unit::Volt,
        false,
        "VoltageHighest",
    ),
    (
        Kind::VoltageAlarm,
        SENSORS_SUBFEATURE_IN_ALARM,
        Unit::None,
        true,
        "VoltageAlarm",
    ),
    (
        Kind::VoltageMinimumAlarm,
        SENSORS_SUBFEATURE_IN_MIN_ALARM,
        Unit::None,
        true,
        "VoltageMinimumAlarm",
    ),
    (
        Kind::VoltageMaximumAlarm,
        SENSORS_SUBFEATURE_IN_MAX_ALARM,
        Unit::None,
        true,
        "VoltageMaximumAlarm",
    ),
    (
        Kind::VoltageBeep,
        SENSORS_SUBFEATURE_IN_BEEP,
        Unit::None,
        true,
        "VoltageBeep",
    ),
    (
        Kind::VoltageLCriticalAlarm,
        SENSORS_SUBFEATURE_IN_LCRIT_ALARM,
        Unit::None,
        true,
        "VoltageLCriticalAlarm",
    ),
    (
        Kind::VoltageCriticalAlarm,
        SENSORS_SUBFEATURE_IN_CRIT_ALARM,
        Unit::None,
        true,
        "VoltageCriticalAlarm",
    ),
    (
        Kind::FanInput,
        SENSORS_SUBFEATURE_FAN_INPUT,
        Unit::RotationPerMinute,
        false,
        "FanInput",
    ),
    (
        Kind::FanMinimum,
        SENSORS_SUBFEATURE_FAN_MIN,
        Unit::RotationPerMinute,
        false,
        "FanMinimum",
    ),
    (
        Kind::FanMaximum,
        SENSORS_SUBFEATURE_FAN_MAX,
        Unit::RotationPerMinute,
        false,
        "FanMaximum",
    ),
    (
        Kind::FanAlarm,
        SENSORS_SUBFEATURE_FAN_ALARM,
        Unit::None,
        true,
        "FanAlarm",
    ),
    (
        Kind::FanFault,
        SENSORS_SUBFEATURE_FAN_FAULT,
        Unit::None,
        true,
        "FanFault",
    ),
    (
        Kind::FanDivisor,
        SENSORS_SUBFEATURE_FAN_DIV,
        Unit::None,
        false,
        "FanDivisor",
    ),
    (
        Kind::FanBeep,
        SENSORS_SUBFEATURE_FAN_BEEP,
        Unit::None,
        true,
        "FanBeep",
    ),
    (
        Kind::FanPulses,
        SENSORS_SUBFEATURE_FAN_PULSES,
        Unit::None,
        false,
        "FanPulses",
    ),
    (
        Kind::FanMinimumAlarm,
        SENSORS_SUBFEATURE_FAN_MIN_ALARM,
        Unit::None,
        true,
        "FanMinimumAlarm",
    ),
    (
        Kind::FanMaximumAlarm,
        SENSORS_SUBFEATURE_FAN_MAX_ALARM,
        Unit::None,
        true,
        "FanMaximumAlarm",
    ),
    (
        Kind::TemperatureInput,
        SENSORS_SUBFEATURE_TEMP_INPUT,
        Unit::Celcius,
        false,
        "TemperatureInput",
    ),
    (
        Kind::TemperatureMaximum,
        SENSORS_SUBFEATURE_TEMP_MAX,
        Unit::Celcius,
        false,
        "TemperatureMaximum",
    ),
    (
        Kind::TemperatureMaximumHysteresis,
        SENSORS_SUBFEATURE_TEMP_MAX_HYST,
        Unit::Celcius,
        false,
        "TemperatureMaximumHysteresis",
    ),
    (
        Kind::TemperatureMinimum,
        SENSORS_SUBFEATURE_TEMP_MIN,
        Unit::Celcius,
        false,
        "TemperatureMinimum",
    ),
    (
        Kind::TemperatureCritical,
        SENSORS_SUBFEATURE_TEMP_CRIT,
        Unit::Celcius,
        false,
        "TemperatureCritical",
    ),
    (
        Kind::TemperatureCriticalHysteresis,
        SENSORS_SUBFEATURE_TEMP_CRIT_HYST,
        Unit::Celcius,
        false,
        "TemperatureCriticalHysteresis",
    ),
    (
        Kind::TemperatureLCritical,
        SENSORS_SUBFEATURE_TEMP_LCRIT,
        Unit::Celcius,
        false,
        "TemperatureLCritical",
    ),
    (
        Kind::TemperatureEmergency,
        SENSORS_SUBFEATURE_TEMP_EMERGENCY,
        Unit::Celcius,
        false,
        "TemperatureEmergency",
    ),
    (
        Kind::TemperatureEmergencyHysteresis,
        SENSORS_SUBFEATURE_TEMP_EMERGENCY_HYST,
        Unit::Celcius,
        false,
        "TemperatureEmergencyHysteresis",
    ),
    (
        Kind::TemperatureLowest,
        SENSORS_SUBFEATURE_TEMP_LOWEST,
        Unit::Celcius,
        false,
        "TemperatureLowest",
    ),
    (
        Kind::TemperatureHighest,
        SENSORS_SUBFEATURE_TEMP_HIGHEST,
        Unit::Celcius,
        false,
        "TemperatureHighest",
    ),
    (
        Kind::TemperatureMinimumHysteresis,
        SENSORS_SUBFEATURE_TEMP_MIN_HYST,
        Unit::Celcius,
        false,
        "TemperatureMinimumHysteresis",
    ),
    (
        Kind::TemperatureLCriticalHysteresis,
        SENSORS_SUBFEATURE_TEMP_LCRIT_HYST,
        Unit::Celcius,
        false,
        "TemperatureLCriticalHysteresis",
    ),
    (
        Kind::TemperatureAlarm,
        SENSORS_SUBFEATURE_TEMP_ALARM,
        Unit::None,
        true,
        "TemperatureAlarm",
    ),
    (
        Kind::TemperatureMaximumAlarm,
        SENSORS_SUBFEATURE_TEMP_MAX_ALARM,
        Unit::None,
        true,
        "TemperatureMaximumAlarm",
    ),
    (
        Kind::TemperatureMinimumAlarm,
        SENSORS_SUBFEATURE_TEMP_MIN_ALARM,
        Unit::None,
        true,
        "TemperatureMinimumAlarm",
    ),
    (
        Kind::TemperatureCriticalAlarm,
        SENSORS_SUBFEATURE_TEMP_CRIT_ALARM,
        Unit::None,
        true,
        "TemperatureCriticalAlarm",
    ),
    (
        Kind::TemperatureFault,
        SENSORS_SUBFEATURE_TEMP_FAULT,
        Unit::None,
        true,
        "TemperatureFault",
    ),
    (
        Kind::TemperatureType,
        SENSORS_SUBFEATURE_TEMP_TYPE,
        Unit::None,
        false,
        "TemperatureType",
    ),
    (
        Kind::TemperatureOffset,
        SENSORS_SUBFEATURE_TEMP_OFFSET,
        Unit::None,
        false,
        "TemperatureOffset",
    ),
    (
        Kind::TemperatureBeep,
        SENSORS_SUBFEATURE_TEMP_BEEP,
        Unit::None,
        true,
        "TemperatureBeep",
    ),
    (
        Kind::TemperatureEmergencyAlarm,
        SENSORS_SUBFEATURE_TEMP_EMERGENCY_ALARM,
        Unit::None,
        true,
        "TemperatureEmergencyAlarm",
    ),
    (
        Kind::TemperatureLCriticalAlarm,
        SENSORS_SUBFEATURE_TEMP_LCRIT_ALARM,
        Unit::None,
        true,
        "TemperatureLCriticalAlarm",
    ),
    (
        Kind::PowerAverage,
        SENSORS_SUBFEATURE_POWER_AVERAGE,
        Unit::Watt,
        false,
        "PowerAverage",
    ),
    (
        Kind::PowerAverageHighest,
        SENSORS_SUBFEATURE_POWER_AVERAGE_HIGHEST,
        Unit::Watt,
        false,
        "PowerAverageHighest",
    ),
    (
        Kind::PowerAverageLowest,
        SENSORS_SUBFEATURE_POWER_AVERAGE_LOWEST,
        Unit::Watt,
        false,
        "PowerAverageLowest",
    ),
    (
        Kind::PowerInput,
        SENSORS_SUBFEATURE_POWER_INPUT,
        Unit::Watt,
        false,
        "PowerInput",
    ),
    (
        Kind::PowerInputHighest,
        SENSORS_SUBFEATURE_POWER_INPUT_HIGHEST,
        Unit::Watt,
        false,
        "PowerInputHighest",
    ),
    (
        Kind::PowerInputLowest,
        SENSORS_SUBFEATURE_POWER_INPUT_LOWEST,
        Unit::Watt,
        false,
        "PowerInputLowest",
    ),
    (
        Kind::PowerCap,
        SENSORS_SUBFEATURE_POWER_CAP,
        Unit::Watt,
        false,
        "PowerCap",
    ),
    (
        Kind::PowerCapHysteresis,
        SENSORS_SUBFEATURE_POWER_CAP_HYST,
        Unit::Watt,
        false,
        "PowerCapHysteresis",
    ),
    (
        Kind::PowerMaximum,
        SENSORS_SUBFEATURE_POWER_MAX,
        Unit::Watt,
        false,
        "PowerMaximum",
    ),
    (
        Kind::PowerCritical,
        SENSORS_SUBFEATURE_POWER_CRIT,
        Unit::Watt,
        false,
        "PowerCritical",
    ),
    (
        Kind::PowerMinimum,
        SENSORS_SUBFEATURE_POWER_MIN,
        Unit::Watt,
        false,
        "PowerMinimum",
    ),
    (
        Kind::PowerLCritical,
        SENSORS_SUBFEATURE_POWER_LCRIT,
        Unit::Watt,
        false,
        "PowerLCritical",
    ),
    (
        Kind::PowerAverageInterval,
        SENSORS_SUBFEATURE_POWER_AVERAGE_INTERVAL,
        Unit::Second,
        false,
        "PowerAverageInterval",
    ),
    (
        Kind::PowerAlarm,
        SENSORS_SUBFEATURE_POWER_ALARM,
        Unit::None,
        true,
        "PowerAlarm",
    ),
    (
        Kind::PowerCapAlarm,
        SENSORS_SUBFEATURE_POWER_CAP_ALARM,
        Unit::None,
        true,
        "PowerCapAlarm",
    ),
    (
        Kind::PowerMaximumAlarm,
        SENSORS_SUBFEATURE_POWER_MAX_ALARM,
        Unit::None,
        true,
        "PowerMaximumAlarm",
    ),
    (
        Kind::PowerCriticalAlarm,
        SENSORS_SUBFEATURE_POWER_CRIT_ALARM,
        Unit::None,
        true,
        "PowerCriticalAlarm",
    ),
    (
        Kind::PowerMinimumAlarm,
        SENSORS_SUBFEATURE_POWER_MIN_ALARM,
        Unit::None,
        true,
        "PowerMinimumAlarm",
    ),
    (
        Kind::PowerLCriticalAlarm,
        SENSORS_SUBFEATURE_POWER_LCRIT_ALARM,
        Unit::None,
        true,
        "PowerLCriticalAlarm",
    ),
    (
        Kind::EnergyInput,
        SENSORS_SUBFEATURE_ENERGY_INPUT,
        Unit::Joule,
        false,
        "EnergyInput",
    ),
    (
        Kind::CurrentInput,
        SENSORS_SUBFEATURE_CURR_INPUT,
        Unit::Amp,
        false,
        "CurrentInput",
    ),
    (
        Kind::CurrentMinimum,
        SENSORS_SUBFEATURE_CURR_MIN,
        Unit::Amp,
        false,
        "CurrentMinimum",
    ),
    (
        Kind::CurrentMaximum,
        SENSORS_SUBFEATURE_CURR_MAX,
        Unit::Amp,
        false,
        "CurrentMaximum",
    ),
    (
        Kind::CurrentLCritical,
        SENSORS_SUBFEATURE_CURR_LCRIT,
        Unit::Amp,
        false,
        "CurrentLCritical",
    ),
    (
        Kind::CurrentCritical,
        SENSORS_SUBFEATURE_CURR_CRIT,
        Unit::Amp,
        false,
        "CurrentCritical",
    ),
    (
        Kind::CurrentAverage,
        SENSORS_SUBFEATURE_CURR_AVERAGE,
        Unit::Amp,
        false,
        "CurrentAverage",
    ),
    (
        Kind::CurrentLowest,
        SENSORS_SUBFEATURE_CURR_LOWEST,
        Unit::Amp,
        false,
        "CurrentLowest",
    ),
    (
        Kind::CurrentHighest,
        SENSORS_SUBFEATURE_CURR_HIGHEST,
        Unit::Amp,
        false,
        "CurrentHighest",
    ),
    (
        Kind::CurrentAlarm,
        SENSORS_SUBFEATURE_CURR_ALARM,
        Unit::None,
        true,
        "CurrentAlarm",
    ),
    (
        Kind::CurrentMinimumAlarm,
        SENSORS_SUBFEATURE_CURR_MIN_ALARM,
        Unit::None,
        true,
        "CurrentMinimumAlarm",
    ),
    (
        Kind::CurrentMaximumAlarm,
        SENSORS_SUBFEATURE_CURR_MAX_ALARM,
        Unit::None,
        true,
        "CurrentMaximumAlarm",
    ),
    (
        Kind::CurrentBeep,
        SENSORS_SUBFEATURE_CURR_BEEP,
        Unit::None,
        true,
        "CurrentBeep",
    ),
    (
        Kind::CurrentLCriticalAlarm,
        SENSORS_SUBFEATURE_CURR_LCRIT_ALARM,
        Unit::None,
        true,
        "CurrentLCriticalAlarm",
    ),
    (
        Kind::CurrentCriticalAlarm,
        SENSORS_SUBFEATURE_CURR_CRIT_ALARM,
        Unit::None,
        true,
        "CurrentCriticalAlarm",
    ),
    (
        Kind::HumidityInput,
        SENSORS_SUBFEATURE_HUMIDITY_INPUT,
        Unit::Percentage,
        false,
        "HumidityInput",
    ),
    (
        Kind::VoltageID,
        SENSORS_SUBFEATURE_VID,
        Unit::Volt,
        false,
        "VoltageID",
    ),
    (
        Kind::IntrusionAlarm,
        SENSORS_SUBFEATURE_INTRUSION_ALARM,
        Unit::None,
        true,
        "IntrusionAlarm",
    ),
    (
        Kind::IntrusionBeep,
        SENSORS_SUBFEATURE_INTRUSION_BEEP,
        Unit::None,
        true,
        "IntrusionBeep",
    ),
    (
        Kind::BeepEnable,
        SENSORS_SUBFEATURE_BEEP_ENABLE,
        Unit::None,
        true,
        "BeepEnable",
    ),
    (
        Kind::Unknown,
        SENSORS_SUBFEATURE_UNKNOWN,
        Unit::None,
        false,
        "Unknown",
    ),
];

#[test]
fn kind() {
    use Kind;

    let k0 = Kind::default();
    assert_eq!(k0, Kind::Unknown);
    assert_eq!(k0.as_raw(), SENSORS_SUBFEATURE_UNKNOWN);
    assert!(!k0.to_string().is_empty());

    for (k, n, u, _b, s) in KIND_LIST {
        assert_eq!(Kind::from_raw(n).unwrap(), k);
        assert_eq!(n, k.as_raw());
        assert_eq!(k.unit(), u);
        assert_eq!(k.to_string(), s);
    }
}

#[test]
fn unit() {
    use Unit;

    let u0 = Unit::default();
    assert_eq!(u0, Unit::None);
    assert!(u0.to_string().is_empty());

    for (u, s) in [
        (Unit::None, ""),
        (Unit::Volt, "V"),
        (Unit::Amp, "A"),
        (Unit::Watt, "W"),
        (Unit::Joule, "J"),
        (Unit::Celcius, "C"),
        (Unit::Second, "s"),
        (Unit::RotationPerMinute, "RPM"),
        (Unit::Percentage, "%"),
    ] {
        assert_eq!(u.to_string(), s);
    }
}

#[test]
fn temperature_sensor_kind() {
    use super::TemperatureSensorKind;

    let t0 = TemperatureSensorKind::default();
    assert_eq!(t0, TemperatureSensorKind::Disabled);
    assert_eq!(t0.as_raw(), 0);
    assert!(!t0.to_string().is_empty());

    assert!(TemperatureSensorKind::from_raw(-1.0).is_none());
    assert_eq!(
        TemperatureSensorKind::Thermistor,
        TemperatureSensorKind::from_raw(1000.51).unwrap()
    );
    assert!(TemperatureSensorKind::from_raw(1000.49).is_none());
    assert_eq!(
        TemperatureSensorKind::Thermistor,
        TemperatureSensorKind::from_raw(4.49).unwrap()
    );

    for (i, (t, s)) in [
        (TemperatureSensorKind::Disabled, "Disabled"),
        (TemperatureSensorKind::CPUDiode, "CPU diode"),
        (TemperatureSensorKind::Transistor, "Transistor"),
        (TemperatureSensorKind::ThermalDiode, "Thermal diode"),
        (TemperatureSensorKind::Thermistor, "Thermistor"),
        (TemperatureSensorKind::AMDAMDSI, "AMD AMDSI"),
        (TemperatureSensorKind::IntelPECI, "Intel PECI"),
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(TemperatureSensorKind::from_raw(i as f64 + 0.49).unwrap(), t);
        assert_eq!(i as c_int, t.as_raw());
        assert_eq!(t.to_string(), s);
    }
}

#[test]
fn sensor_value_new() {
    use super::Value;

    for (k, n, _u, b, _s) in KIND_LIST {
        let mut v0 = Value::new(k, 3.49).unwrap();
        assert_eq!(v0.kind().as_raw(), n);
        assert!(!v0.to_string().is_empty());

        if b {
            assert!((v0.raw_value() - 1.0).abs() <= f64::EPSILON);
            assert_eq!(v0.unit(), Unit::None);
        } else if k == Kind::TemperatureType {
            assert!((v0.raw_value() - 3.0).abs() <= f64::EPSILON);
            assert_eq!(v0.unit(), Unit::None);

            v0.set_raw_value(-1.0).unwrap_err();
        } else {
            assert!((v0.raw_value() - 3.49).abs() <= f64::EPSILON);
        }

        let mut v1 = Value::from_raw(n, 0.0).unwrap();
        assert_eq!(v1.kind(), k);
        assert!((v1.raw_value() - 0.0).abs() <= f64::EPSILON);
        if b {
            assert_eq!(v1.unit(), Unit::None);
            assert!(v1.to_string().is_empty());
        } else if k == Kind::TemperatureType {
            assert_eq!(v1.unit(), Unit::None);
            assert!(!v1.to_string().is_empty());
        } else {
            assert!(!v1.to_string().is_empty());
        }

        let old_value = v1.set_raw_value(3.49).unwrap();
        assert!((old_value - 0.0).abs() <= f64::EPSILON);

        assert_eq!(v0, v1);
    }
}

#[test]
fn sensor_value_new_bool() {
    use super::Value;

    for (k, n, _u, b, _s) in KIND_LIST {
        if !b {
            continue;
        }

        let v0 = Value::new_bool(k, true).unwrap();
        assert_eq!(v0.kind().as_raw(), n);
        assert_eq!(v0.kind().unit(), Unit::None);
        assert!(!v0.to_string().is_empty());

        let v1 = Value::new_bool(k, false).unwrap();
        assert_eq!(v1.kind().as_raw(), n);
        assert_eq!(v1.kind().unit(), Unit::None);
        assert!(v1.to_string().is_empty());
    }
}

#[test]
fn sensor_value_new_temperature_sensor_kind() {
    use super::{TemperatureSensorKind, Value};

    let v = Value::new_temperature_sensor_kind(
        Kind::TemperatureType,
        TemperatureSensorKind::Transistor,
    )
    .unwrap();
    assert_eq!(v.kind().as_raw(), SENSORS_SUBFEATURE_TEMP_TYPE);
    assert_eq!(v.kind().unit(), Unit::None);

    assert!(Value::new_temperature_sensor_kind(
        Kind::TemperatureInput,
        TemperatureSensorKind::Transistor,
    )
    .is_none());
}
