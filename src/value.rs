//! Values of sensors or actuators.

#[cfg(test)]
mod tests;

use core::fmt;
use std::io;
use std::os::raw::{c_int, c_uint};

use sensors_sys::sensors_subfeature_type::*;

use crate::errors::{Error, Result};

/// Value reported by a sensor or set for an actuator,
/// controlled by a [`SubFeatureRef`] instance.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum Value {
    VoltageInput(f64),
    VoltageMinimum(f64),
    VoltageMaximum(f64),
    VoltageLCritical(f64),
    VoltageCritical(f64),
    VoltageAverage(f64),
    VoltageLowest(f64),
    VoltageHighest(f64),
    VoltageAlarm(bool),
    VoltageMinimumAlarm(bool),
    VoltageMaximumAlarm(bool),
    VoltageBeep(bool),
    VoltageLCriticalAlarm(bool),
    VoltageCriticalAlarm(bool),

    FanInput(f64),
    FanMinimum(f64),
    FanMaximum(f64),
    FanAlarm(bool),
    FanFault(bool),
    FanDivisor(f64),
    FanBeep(bool),
    FanPulses(f64),
    FanMinimumAlarm(bool),
    FanMaximumAlarm(bool),

    TemperatureInput(f64),
    TemperatureMaximum(f64),
    TemperatureMaximumHysteresis(f64),
    TemperatureMinimum(f64),
    TemperatureCritical(f64),
    TemperatureCriticalHysteresis(f64),
    TemperatureLCritical(f64),
    TemperatureEmergency(f64),
    TemperatureEmergencyHysteresis(f64),
    TemperatureLowest(f64),
    TemperatureHighest(f64),
    TemperatureMinimumHysteresis(f64),
    TemperatureLCriticalHysteresis(f64),
    TemperatureAlarm(bool),
    TemperatureMaximumAlarm(bool),
    TemperatureMinimumAlarm(bool),
    TemperatureCriticalAlarm(bool),
    TemperatureFault(bool),
    TemperatureType(TemperatureSensorKind),
    TemperatureOffset(f64),
    TemperatureBeep(bool),
    TemperatureEmergencyAlarm(bool),
    TemperatureLCriticalAlarm(bool),

    PowerAverage(f64),
    PowerAverageHighest(f64),
    PowerAverageLowest(f64),
    PowerInput(f64),
    PowerInputHighest(f64),
    PowerInputLowest(f64),
    PowerCap(f64),
    PowerCapHysteresis(f64),
    PowerMaximum(f64),
    PowerCritical(f64),
    PowerMinimum(f64),
    PowerLCritical(f64),
    PowerAverageInterval(f64),
    PowerAlarm(bool),
    PowerCapAlarm(bool),
    PowerMaximumAlarm(bool),
    PowerCriticalAlarm(bool),
    PowerMinimumAlarm(bool),
    PowerLCriticalAlarm(bool),

    EnergyInput(f64),

    CurrentInput(f64),
    CurrentMinimum(f64),
    CurrentMaximum(f64),
    CurrentLCritical(f64),
    CurrentCritical(f64),
    CurrentAverage(f64),
    CurrentLowest(f64),
    CurrentHighest(f64),
    CurrentAlarm(bool),
    CurrentMinimumAlarm(bool),
    CurrentMaximumAlarm(bool),
    CurrentBeep(bool),
    CurrentLCriticalAlarm(bool),
    CurrentCriticalAlarm(bool),

    HumidityInput(f64),

    VoltageID(f64),

    IntrusionAlarm(bool),
    IntrusionBeep(bool),

    BeepEnable(bool),

    Unknown { kind: Kind, value: f64 },
}

impl Value {
    /// Return an instance of the given type and value.
    ///
    /// The valid range for the value depends on the kind.
    #[must_use]
    pub fn new(kind: Kind, value: f64) -> Option<Self> {
        let result = match kind {
            Kind::VoltageInput => Self::VoltageInput(value),
            Kind::VoltageMinimum => Self::VoltageMinimum(value),
            Kind::VoltageMaximum => Self::VoltageMaximum(value),
            Kind::VoltageLCritical => Self::VoltageLCritical(value),
            Kind::VoltageCritical => Self::VoltageCritical(value),
            Kind::VoltageAverage => Self::VoltageAverage(value),
            Kind::VoltageLowest => Self::VoltageLowest(value),
            Kind::VoltageHighest => Self::VoltageHighest(value),
            Kind::VoltageAlarm => Self::VoltageAlarm(value != 0.0_f64),
            Kind::VoltageMinimumAlarm => Self::VoltageMinimumAlarm(value != 0.0_f64),
            Kind::VoltageMaximumAlarm => Self::VoltageMaximumAlarm(value != 0.0_f64),
            Kind::VoltageBeep => Self::VoltageBeep(value != 0.0_f64),
            Kind::VoltageLCriticalAlarm => Self::VoltageLCriticalAlarm(value != 0.0_f64),
            Kind::VoltageCriticalAlarm => Self::VoltageCriticalAlarm(value != 0.0_f64),

            Kind::FanInput => Self::FanInput(value),
            Kind::FanMinimum => Self::FanMinimum(value),
            Kind::FanMaximum => Self::FanMaximum(value),
            Kind::FanAlarm => Self::FanAlarm(value != 0.0_f64),
            Kind::FanFault => Self::FanFault(value != 0.0_f64),
            Kind::FanDivisor => Self::FanDivisor(value),
            Kind::FanBeep => Self::FanBeep(value != 0.0_f64),
            Kind::FanPulses => Self::FanPulses(value),
            Kind::FanMinimumAlarm => Self::FanMinimumAlarm(value != 0.0_f64),
            Kind::FanMaximumAlarm => Self::FanMaximumAlarm(value != 0.0_f64),

            Kind::TemperatureInput => Self::TemperatureInput(value),
            Kind::TemperatureMaximum => Self::TemperatureMaximum(value),
            Kind::TemperatureMaximumHysteresis => Self::TemperatureMaximumHysteresis(value),
            Kind::TemperatureMinimum => Self::TemperatureMinimum(value),
            Kind::TemperatureCritical => Self::TemperatureCritical(value),
            Kind::TemperatureCriticalHysteresis => Self::TemperatureCriticalHysteresis(value),
            Kind::TemperatureLCritical => Self::TemperatureLCritical(value),
            Kind::TemperatureEmergency => Self::TemperatureEmergency(value),
            Kind::TemperatureEmergencyHysteresis => Self::TemperatureEmergencyHysteresis(value),
            Kind::TemperatureLowest => Self::TemperatureLowest(value),
            Kind::TemperatureHighest => Self::TemperatureHighest(value),
            Kind::TemperatureMinimumHysteresis => Self::TemperatureMinimumHysteresis(value),
            Kind::TemperatureLCriticalHysteresis => Self::TemperatureLCriticalHysteresis(value),
            Kind::TemperatureAlarm => Self::TemperatureAlarm(value != 0.0_f64),
            Kind::TemperatureMaximumAlarm => Self::TemperatureMaximumAlarm(value != 0.0_f64),
            Kind::TemperatureMinimumAlarm => Self::TemperatureMinimumAlarm(value != 0.0_f64),
            Kind::TemperatureCriticalAlarm => Self::TemperatureCriticalAlarm(value != 0.0_f64),
            Kind::TemperatureFault => Self::TemperatureFault(value != 0.0_f64),
            Kind::TemperatureType => {
                let value = TemperatureSensorKind::from_raw(value)?;
                Self::TemperatureType(value)
            }
            Kind::TemperatureOffset => Self::TemperatureOffset(value),
            Kind::TemperatureBeep => Self::TemperatureBeep(value != 0.0_f64),
            Kind::TemperatureEmergencyAlarm => Self::TemperatureEmergencyAlarm(value != 0.0_f64),
            Kind::TemperatureLCriticalAlarm => Self::TemperatureLCriticalAlarm(value != 0.0_f64),

            Kind::PowerAverage => Self::PowerAverage(value),
            Kind::PowerAverageHighest => Self::PowerAverageHighest(value),
            Kind::PowerAverageLowest => Self::PowerAverageLowest(value),
            Kind::PowerInput => Self::PowerInput(value),
            Kind::PowerInputHighest => Self::PowerInputHighest(value),
            Kind::PowerInputLowest => Self::PowerInputLowest(value),
            Kind::PowerCap => Self::PowerCap(value),
            Kind::PowerCapHysteresis => Self::PowerCapHysteresis(value),
            Kind::PowerMaximum => Self::PowerMaximum(value),
            Kind::PowerCritical => Self::PowerCritical(value),
            Kind::PowerMinimum => Self::PowerMinimum(value),
            Kind::PowerLCritical => Self::PowerLCritical(value),
            Kind::PowerAverageInterval => Self::PowerAverageInterval(value),
            Kind::PowerAlarm => Self::PowerAlarm(value != 0.0_f64),
            Kind::PowerCapAlarm => Self::PowerCapAlarm(value != 0.0_f64),
            Kind::PowerMaximumAlarm => Self::PowerMaximumAlarm(value != 0.0_f64),
            Kind::PowerCriticalAlarm => Self::PowerCriticalAlarm(value != 0.0_f64),
            Kind::PowerMinimumAlarm => Self::PowerMinimumAlarm(value != 0.0_f64),
            Kind::PowerLCriticalAlarm => Self::PowerLCriticalAlarm(value != 0.0_f64),

            Kind::EnergyInput => Self::EnergyInput(value),

            Kind::CurrentInput => Self::CurrentInput(value),
            Kind::CurrentMinimum => Self::CurrentMinimum(value),
            Kind::CurrentMaximum => Self::CurrentMaximum(value),
            Kind::CurrentLCritical => Self::CurrentLCritical(value),
            Kind::CurrentCritical => Self::CurrentCritical(value),
            Kind::CurrentAverage => Self::CurrentAverage(value),
            Kind::CurrentLowest => Self::CurrentLowest(value),
            Kind::CurrentHighest => Self::CurrentHighest(value),
            Kind::CurrentAlarm => Self::CurrentAlarm(value != 0.0_f64),
            Kind::CurrentMinimumAlarm => Self::CurrentMinimumAlarm(value != 0.0_f64),
            Kind::CurrentMaximumAlarm => Self::CurrentMaximumAlarm(value != 0.0_f64),
            Kind::CurrentBeep => Self::CurrentBeep(value != 0.0_f64),
            Kind::CurrentLCriticalAlarm => Self::CurrentLCriticalAlarm(value != 0.0_f64),
            Kind::CurrentCriticalAlarm => Self::CurrentCriticalAlarm(value != 0.0_f64),

            Kind::HumidityInput => Self::HumidityInput(value),

            Kind::VoltageID => Self::VoltageID(value),

            Kind::IntrusionAlarm => Self::IntrusionAlarm(value != 0.0_f64),
            Kind::IntrusionBeep => Self::IntrusionBeep(value != 0.0_f64),

            Kind::BeepEnable => Self::BeepEnable(value != 0.0_f64),

            Kind::Unknown => Self::Unknown { kind, value },
        };
        Some(result)
    }

    /// Return an instance of the given type and boolean value.
    #[must_use]
    pub fn new_bool(kind: Kind, value: bool) -> Option<Self> {
        Self::new(kind, if value { 1.0 } else { 0.0 })
    }

    /// Return an instance of the given type and temperature sensor type.
    #[must_use]
    pub fn new_temperature_sensor_kind(kind: Kind, value: TemperatureSensorKind) -> Option<Self> {
        match kind {
            Kind::TemperatureType => Some(Self::TemperatureType(value)),

            Kind::VoltageInput
            | Kind::VoltageMinimum
            | Kind::VoltageMaximum
            | Kind::VoltageLCritical
            | Kind::VoltageCritical
            | Kind::VoltageAverage
            | Kind::VoltageLowest
            | Kind::VoltageHighest
            | Kind::VoltageAlarm
            | Kind::VoltageMinimumAlarm
            | Kind::VoltageMaximumAlarm
            | Kind::VoltageBeep
            | Kind::VoltageLCriticalAlarm
            | Kind::VoltageCriticalAlarm
            | Kind::FanInput
            | Kind::FanMinimum
            | Kind::FanMaximum
            | Kind::FanAlarm
            | Kind::FanFault
            | Kind::FanDivisor
            | Kind::FanBeep
            | Kind::FanPulses
            | Kind::FanMinimumAlarm
            | Kind::FanMaximumAlarm
            | Kind::TemperatureInput
            | Kind::TemperatureMaximum
            | Kind::TemperatureMaximumHysteresis
            | Kind::TemperatureMinimum
            | Kind::TemperatureCritical
            | Kind::TemperatureCriticalHysteresis
            | Kind::TemperatureLCritical
            | Kind::TemperatureEmergency
            | Kind::TemperatureEmergencyHysteresis
            | Kind::TemperatureLowest
            | Kind::TemperatureHighest
            | Kind::TemperatureMinimumHysteresis
            | Kind::TemperatureLCriticalHysteresis
            | Kind::TemperatureAlarm
            | Kind::TemperatureMaximumAlarm
            | Kind::TemperatureMinimumAlarm
            | Kind::TemperatureCriticalAlarm
            | Kind::TemperatureFault
            | Kind::TemperatureOffset
            | Kind::TemperatureBeep
            | Kind::TemperatureEmergencyAlarm
            | Kind::TemperatureLCriticalAlarm
            | Kind::PowerAverage
            | Kind::PowerAverageHighest
            | Kind::PowerAverageLowest
            | Kind::PowerInput
            | Kind::PowerInputHighest
            | Kind::PowerInputLowest
            | Kind::PowerCap
            | Kind::PowerCapHysteresis
            | Kind::PowerMaximum
            | Kind::PowerCritical
            | Kind::PowerMinimum
            | Kind::PowerLCritical
            | Kind::PowerAverageInterval
            | Kind::PowerAlarm
            | Kind::PowerCapAlarm
            | Kind::PowerMaximumAlarm
            | Kind::PowerCriticalAlarm
            | Kind::PowerMinimumAlarm
            | Kind::PowerLCriticalAlarm
            | Kind::EnergyInput
            | Kind::CurrentInput
            | Kind::CurrentMinimum
            | Kind::CurrentMaximum
            | Kind::CurrentLCritical
            | Kind::CurrentCritical
            | Kind::CurrentAverage
            | Kind::CurrentLowest
            | Kind::CurrentHighest
            | Kind::CurrentAlarm
            | Kind::CurrentMinimumAlarm
            | Kind::CurrentMaximumAlarm
            | Kind::CurrentBeep
            | Kind::CurrentLCriticalAlarm
            | Kind::CurrentCriticalAlarm
            | Kind::HumidityInput
            | Kind::VoltageID
            | Kind::IntrusionAlarm
            | Kind::IntrusionBeep
            | Kind::BeepEnable
            | Kind::Unknown => None,
        }
    }

    /// Return an instance of the given raw type and raw value.
    ///
    /// The raw type is one of `SENSORS_SUBFEATURE_*` values,
    /// *e.g.,* [`SENSORS_SUBFEATURE_TEMP_INPUT`].
    /// The valid range for the raw value depends on the raw kind.
    #[must_use]
    pub fn from_raw(kind: c_uint, value: f64) -> Option<Self> {
        Kind::from_raw(kind).and_then(|kind| Self::new(kind, value))
    }

    /// Return the type of this instance.
    #[must_use]
    pub fn kind(&self) -> Kind {
        match *self {
            Self::VoltageInput(_) => Kind::VoltageInput,
            Self::VoltageMinimum(_) => Kind::VoltageMinimum,
            Self::VoltageMaximum(_) => Kind::VoltageMaximum,
            Self::VoltageLCritical(_) => Kind::VoltageLCritical,
            Self::VoltageCritical(_) => Kind::VoltageCritical,
            Self::VoltageAverage(_) => Kind::VoltageAverage,
            Self::VoltageLowest(_) => Kind::VoltageLowest,
            Self::VoltageHighest(_) => Kind::VoltageHighest,
            Self::VoltageAlarm(_) => Kind::VoltageAlarm,
            Self::VoltageMinimumAlarm(_) => Kind::VoltageMinimumAlarm,
            Self::VoltageMaximumAlarm(_) => Kind::VoltageMaximumAlarm,
            Self::VoltageBeep(_) => Kind::VoltageBeep,
            Self::VoltageLCriticalAlarm(_) => Kind::VoltageLCriticalAlarm,
            Self::VoltageCriticalAlarm(_) => Kind::VoltageCriticalAlarm,

            Self::FanInput(_) => Kind::FanInput,
            Self::FanMinimum(_) => Kind::FanMinimum,
            Self::FanMaximum(_) => Kind::FanMaximum,
            Self::FanAlarm(_) => Kind::FanAlarm,
            Self::FanFault(_) => Kind::FanFault,
            Self::FanDivisor(_) => Kind::FanDivisor,
            Self::FanBeep(_) => Kind::FanBeep,
            Self::FanPulses(_) => Kind::FanPulses,
            Self::FanMinimumAlarm(_) => Kind::FanMinimumAlarm,
            Self::FanMaximumAlarm(_) => Kind::FanMaximumAlarm,

            Self::TemperatureInput(_) => Kind::TemperatureInput,
            Self::TemperatureMaximum(_) => Kind::TemperatureMaximum,
            Self::TemperatureMaximumHysteresis(_) => Kind::TemperatureMaximumHysteresis,
            Self::TemperatureMinimum(_) => Kind::TemperatureMinimum,
            Self::TemperatureCritical(_) => Kind::TemperatureCritical,
            Self::TemperatureCriticalHysteresis(_) => Kind::TemperatureCriticalHysteresis,
            Self::TemperatureLCritical(_) => Kind::TemperatureLCritical,
            Self::TemperatureEmergency(_) => Kind::TemperatureEmergency,
            Self::TemperatureEmergencyHysteresis(_) => Kind::TemperatureEmergencyHysteresis,
            Self::TemperatureLowest(_) => Kind::TemperatureLowest,
            Self::TemperatureHighest(_) => Kind::TemperatureHighest,
            Self::TemperatureMinimumHysteresis(_) => Kind::TemperatureMinimumHysteresis,
            Self::TemperatureLCriticalHysteresis(_) => Kind::TemperatureLCriticalHysteresis,
            Self::TemperatureAlarm(_) => Kind::TemperatureAlarm,
            Self::TemperatureMaximumAlarm(_) => Kind::TemperatureMaximumAlarm,
            Self::TemperatureMinimumAlarm(_) => Kind::TemperatureMinimumAlarm,
            Self::TemperatureCriticalAlarm(_) => Kind::TemperatureCriticalAlarm,
            Self::TemperatureFault(_) => Kind::TemperatureFault,
            Self::TemperatureType(_) => Kind::TemperatureType,
            Self::TemperatureOffset(_) => Kind::TemperatureOffset,
            Self::TemperatureBeep(_) => Kind::TemperatureBeep,
            Self::TemperatureEmergencyAlarm(_) => Kind::TemperatureEmergencyAlarm,
            Self::TemperatureLCriticalAlarm(_) => Kind::TemperatureLCriticalAlarm,

            Self::PowerAverage(_) => Kind::PowerAverage,
            Self::PowerAverageHighest(_) => Kind::PowerAverageHighest,
            Self::PowerAverageLowest(_) => Kind::PowerAverageLowest,
            Self::PowerInput(_) => Kind::PowerInput,
            Self::PowerInputHighest(_) => Kind::PowerInputHighest,
            Self::PowerInputLowest(_) => Kind::PowerInputLowest,
            Self::PowerCap(_) => Kind::PowerCap,
            Self::PowerCapHysteresis(_) => Kind::PowerCapHysteresis,
            Self::PowerMaximum(_) => Kind::PowerMaximum,
            Self::PowerCritical(_) => Kind::PowerCritical,
            Self::PowerMinimum(_) => Kind::PowerMinimum,
            Self::PowerLCritical(_) => Kind::PowerLCritical,
            Self::PowerAverageInterval(_) => Kind::PowerAverageInterval,
            Self::PowerAlarm(_) => Kind::PowerAlarm,
            Self::PowerCapAlarm(_) => Kind::PowerCapAlarm,
            Self::PowerMaximumAlarm(_) => Kind::PowerMaximumAlarm,
            Self::PowerCriticalAlarm(_) => Kind::PowerCriticalAlarm,
            Self::PowerMinimumAlarm(_) => Kind::PowerMinimumAlarm,
            Self::PowerLCriticalAlarm(_) => Kind::PowerLCriticalAlarm,

            Self::EnergyInput(_) => Kind::EnergyInput,

            Self::CurrentInput(_) => Kind::CurrentInput,
            Self::CurrentMinimum(_) => Kind::CurrentMinimum,
            Self::CurrentMaximum(_) => Kind::CurrentMaximum,
            Self::CurrentLCritical(_) => Kind::CurrentLCritical,
            Self::CurrentCritical(_) => Kind::CurrentCritical,
            Self::CurrentAverage(_) => Kind::CurrentAverage,
            Self::CurrentLowest(_) => Kind::CurrentLowest,
            Self::CurrentHighest(_) => Kind::CurrentHighest,
            Self::CurrentAlarm(_) => Kind::CurrentAlarm,
            Self::CurrentMinimumAlarm(_) => Kind::CurrentMinimumAlarm,
            Self::CurrentMaximumAlarm(_) => Kind::CurrentMaximumAlarm,
            Self::CurrentBeep(_) => Kind::CurrentBeep,
            Self::CurrentLCriticalAlarm(_) => Kind::CurrentLCriticalAlarm,
            Self::CurrentCriticalAlarm(_) => Kind::CurrentCriticalAlarm,

            Self::HumidityInput(_) => Kind::HumidityInput,

            Self::VoltageID(_) => Kind::VoltageID,

            Self::IntrusionAlarm(_) => Kind::IntrusionAlarm,
            Self::IntrusionBeep(_) => Kind::IntrusionBeep,

            Self::BeepEnable(_) => Kind::BeepEnable,

            Self::Unknown { .. } => Kind::Unknown,
        }
    }

    /// Return the raw value of this instance.
    #[must_use]
    pub fn raw_value(&self) -> f64 {
        match *self {
            // Voltage
            Self::VoltageInput(value)
            | Self::VoltageMinimum(value)
            | Self::VoltageMaximum(value)
            | Self::VoltageLCritical(value)
            | Self::VoltageCritical(value)
            | Self::VoltageAverage(value)
            | Self::VoltageLowest(value)
            | Self::VoltageHighest(value)
            // Fan
            | Self::FanInput(value)
            | Self::FanMinimum(value)
            | Self::FanMaximum(value)
            | Self::FanDivisor(value)
            | Self::FanPulses(value)
            // Temperature
            | Self::TemperatureInput(value)
            | Self::TemperatureMaximum(value)
            | Self::TemperatureMaximumHysteresis(value)
            | Self::TemperatureMinimum(value)
            | Self::TemperatureCritical(value)
            | Self::TemperatureCriticalHysteresis(value)
            | Self::TemperatureLCritical(value)
            | Self::TemperatureEmergency(value)
            | Self::TemperatureEmergencyHysteresis(value)
            | Self::TemperatureLowest(value)
            | Self::TemperatureHighest(value)
            | Self::TemperatureMinimumHysteresis(value)
            | Self::TemperatureLCriticalHysteresis(value)
            | Self::TemperatureOffset(value)
            // Power
            | Self::PowerAverage(value)
            | Self::PowerAverageHighest(value)
            | Self::PowerAverageLowest(value)
            | Self::PowerInput(value)
            | Self::PowerInputHighest(value)
            | Self::PowerInputLowest(value)
            | Self::PowerCap(value)
            | Self::PowerCapHysteresis(value)
            | Self::PowerMaximum(value)
            | Self::PowerCritical(value)
            | Self::PowerMinimum(value)
            | Self::PowerLCritical(value)
            | Self::PowerAverageInterval(value)
            // Energy
            | Self::EnergyInput(value)
            // Current
            | Self::CurrentInput(value)
            | Self::CurrentMinimum(value)
            | Self::CurrentMaximum(value)
            | Self::CurrentLCritical(value)
            | Self::CurrentCritical(value)
            | Self::CurrentAverage(value)
            | Self::CurrentLowest(value)
            | Self::CurrentHighest(value)
            // Humidity
            | Self::HumidityInput(value)
            // VoltageID
            | Self::VoltageID(value)
            // Unknown
            | Self::Unknown { value, .. }
            => value,

            // Voltage
            Self::VoltageAlarm(value)
            | Self::VoltageMinimumAlarm(value)
            | Self::VoltageMaximumAlarm(value)
            | Self::VoltageBeep(value)
            | Self::VoltageLCriticalAlarm(value)
            | Self::VoltageCriticalAlarm(value)
            // Fan
            | Self::FanAlarm(value)
            | Self::FanFault(value)
            | Self::FanBeep(value)
            | Self::FanMinimumAlarm(value)
            | Self::FanMaximumAlarm(value)
            // Temperature
            | Self::TemperatureAlarm(value)
            | Self::TemperatureMaximumAlarm(value)
            | Self::TemperatureMinimumAlarm(value)
            | Self::TemperatureCriticalAlarm(value)
            | Self::TemperatureFault(value)
            | Self::TemperatureBeep(value)
            | Self::TemperatureEmergencyAlarm(value)
            | Self::TemperatureLCriticalAlarm(value)
            // Power
            | Self::PowerAlarm(value)
            | Self::PowerCapAlarm(value)
            | Self::PowerMaximumAlarm(value)
            | Self::PowerCriticalAlarm(value)
            | Self::PowerMinimumAlarm(value)
            | Self::PowerLCriticalAlarm(value)
            // Current
            | Self::CurrentAlarm(value)
            | Self::CurrentMinimumAlarm(value)
            | Self::CurrentMaximumAlarm(value)
            | Self::CurrentBeep(value)
            | Self::CurrentLCriticalAlarm(value)
            | Self::CurrentCriticalAlarm(value)
            // Intrusion
            | Self::IntrusionAlarm(value)
            | Self::IntrusionBeep(value)
            // Beep
            | Self::BeepEnable(value) => {
                if value {
                    1.0_f64
                } else {
                    0.0_f64
                }
            }

            Self::TemperatureType(value) => f64::from(value.as_raw()),
        }
    }

    /// Set the raw value of this instance.
    pub fn set_raw_value(&mut self, new_value: f64) -> Result<f64> {
        match self {
            // Voltage
            Self::VoltageInput(value)
            | Self::VoltageMinimum(value)
            | Self::VoltageMaximum(value)
            | Self::VoltageLCritical(value)
            | Self::VoltageCritical(value)
            | Self::VoltageAverage(value)
            | Self::VoltageLowest(value)
            | Self::VoltageHighest(value)
            // Fan
            | Self::FanInput(value)
            | Self::FanMinimum(value)
            | Self::FanMaximum(value)
            | Self::FanDivisor(value)
            | Self::FanPulses(value)
            // Temperature
            | Self::TemperatureInput(value)
            | Self::TemperatureMaximum(value)
            | Self::TemperatureMaximumHysteresis(value)
            | Self::TemperatureMinimum(value)
            | Self::TemperatureCritical(value)
            | Self::TemperatureCriticalHysteresis(value)
            | Self::TemperatureLCritical(value)
            | Self::TemperatureEmergency(value)
            | Self::TemperatureEmergencyHysteresis(value)
            | Self::TemperatureLowest(value)
            | Self::TemperatureHighest(value)
            | Self::TemperatureMinimumHysteresis(value)
            | Self::TemperatureLCriticalHysteresis(value)
            | Self::TemperatureOffset(value)
            // Power
            | Self::PowerAverage(value)
            | Self::PowerAverageHighest(value)
            | Self::PowerAverageLowest(value)
            | Self::PowerInput(value)
            | Self::PowerInputHighest(value)
            | Self::PowerInputLowest(value)
            | Self::PowerCap(value)
            | Self::PowerCapHysteresis(value)
            | Self::PowerMaximum(value)
            | Self::PowerCritical(value)
            | Self::PowerMinimum(value)
            | Self::PowerLCritical(value)
            | Self::PowerAverageInterval(value)
            // Energy
            | Self::EnergyInput(value)
            // Current
            | Self::CurrentInput(value)
            | Self::CurrentMinimum(value)
            | Self::CurrentMaximum(value)
            | Self::CurrentLCritical(value)
            | Self::CurrentCritical(value)
            | Self::CurrentAverage(value)
            | Self::CurrentLowest(value)
            | Self::CurrentHighest(value)
            // Humidity
            | Self::HumidityInput(value)
            // VoltageID
            | Self::VoltageID(value)
            // Unknown
            | Self::Unknown { value, .. } => {
                let result = *value;
                *value = new_value;
                Ok(result)
            }

            // Voltage
            Self::VoltageAlarm(value)
            | Self::VoltageMinimumAlarm(value)
            | Self::VoltageMaximumAlarm(value)
            | Self::VoltageBeep(value)
            | Self::VoltageLCriticalAlarm(value)
            | Self::VoltageCriticalAlarm(value)
            // Fan
            | Self::FanAlarm(value)
            | Self::FanFault(value)
            | Self::FanBeep(value)
            | Self::FanMinimumAlarm(value)
            | Self::FanMaximumAlarm(value)
            // Temperature
            | Self::TemperatureAlarm(value)
            | Self::TemperatureMaximumAlarm(value)
            | Self::TemperatureMinimumAlarm(value)
            | Self::TemperatureCriticalAlarm(value)
            | Self::TemperatureFault(value)
            | Self::TemperatureBeep(value)
            | Self::TemperatureEmergencyAlarm(value)
            | Self::TemperatureLCriticalAlarm(value)
            // Power
            | Self::PowerAlarm(value)
            | Self::PowerCapAlarm(value)
            | Self::PowerMaximumAlarm(value)
            | Self::PowerCriticalAlarm(value)
            | Self::PowerMinimumAlarm(value)
            | Self::PowerLCriticalAlarm(value)
            // Current
            | Self::CurrentAlarm(value)
            | Self::CurrentMinimumAlarm(value)
            | Self::CurrentMaximumAlarm(value)
            | Self::CurrentBeep(value)
            | Self::CurrentLCriticalAlarm(value)
            | Self::CurrentCriticalAlarm(value)
            // Intrusion
            | Self::IntrusionAlarm(value)
            | Self::IntrusionBeep(value)
            // Beep
            | Self::BeepEnable(value) => {
                let result: f64 = if *value { 1.0 } else { 0.0 };
                *value = new_value != 0.0_f64;
                Ok(result)
            },

            Self::TemperatureType(value) => {
                let new_value = TemperatureSensorKind::from_raw(new_value)
                    .ok_or_else(|| Error::from_io("TemperatureSensorKind::new", io::ErrorKind::InvalidData.into()))?;

                let result = value.as_raw() as f64;
                *value = new_value;
                Ok(result)
            },
        }
    }

    /// Return the measurement unit of this instance.
    #[must_use]
    pub fn unit(&self) -> Unit {
        self.kind().unit()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::VoltageAlarm(value)
            | Self::VoltageMinimumAlarm(value)
            | Self::VoltageMaximumAlarm(value)
            | Self::VoltageLCriticalAlarm(value)
            | Self::VoltageCriticalAlarm(value)
            | Self::FanAlarm(value)
            | Self::FanMinimumAlarm(value)
            | Self::FanMaximumAlarm(value)
            | Self::TemperatureAlarm(value)
            | Self::TemperatureMaximumAlarm(value)
            | Self::TemperatureMinimumAlarm(value)
            | Self::TemperatureCriticalAlarm(value)
            | Self::TemperatureEmergencyAlarm(value)
            | Self::TemperatureLCriticalAlarm(value)
            | Self::PowerAlarm(value)
            | Self::PowerCapAlarm(value)
            | Self::PowerMaximumAlarm(value)
            | Self::PowerCriticalAlarm(value)
            | Self::PowerMinimumAlarm(value)
            | Self::PowerLCriticalAlarm(value)
            | Self::CurrentAlarm(value)
            | Self::CurrentMinimumAlarm(value)
            | Self::CurrentMaximumAlarm(value)
            | Self::CurrentLCriticalAlarm(value)
            | Self::CurrentCriticalAlarm(value)
            | Self::IntrusionAlarm(value) => {
                if value {
                    write!(f, "ALARM")
                } else {
                    Ok(())
                }
            }

            Self::VoltageBeep(value)
            | Self::FanBeep(value)
            | Self::TemperatureBeep(value)
            | Self::CurrentBeep(value)
            | Self::IntrusionBeep(value)
            | Self::BeepEnable(value) => {
                if value {
                    write!(f, "BEEP")
                } else {
                    Ok(())
                }
            }

            Self::FanFault(value) | Self::TemperatureFault(value) => {
                if value {
                    write!(f, "FAULT")
                } else {
                    Ok(())
                }
            }

            Self::FanDivisor(value) | Self::FanPulses(value) | Self::TemperatureOffset(value) => {
                write!(f, "{value}")
            }

            Self::TemperatureType(value) => write!(f, "{value}"),

            Self::VoltageInput(value)
            | Self::VoltageMinimum(value)
            | Self::VoltageMaximum(value)
            | Self::VoltageLCritical(value)
            | Self::VoltageCritical(value)
            | Self::VoltageAverage(value)
            | Self::VoltageLowest(value)
            | Self::VoltageHighest(value)
            | Self::VoltageID(value) => write!(f, "{} {}", value, Unit::Volt),

            Self::FanInput(value) | Self::FanMinimum(value) | Self::FanMaximum(value) => {
                write!(f, "{} {}", value, Unit::RotationPerMinute)
            }

            Self::TemperatureInput(value)
            | Self::TemperatureMaximum(value)
            | Self::TemperatureMaximumHysteresis(value)
            | Self::TemperatureMinimum(value)
            | Self::TemperatureCritical(value)
            | Self::TemperatureCriticalHysteresis(value)
            | Self::TemperatureLCritical(value)
            | Self::TemperatureEmergency(value)
            | Self::TemperatureEmergencyHysteresis(value)
            | Self::TemperatureLowest(value)
            | Self::TemperatureHighest(value)
            | Self::TemperatureMinimumHysteresis(value)
            | Self::TemperatureLCriticalHysteresis(value) => {
                write!(f, "{} {}", value, Unit::Celcius)
            }

            Self::PowerAverage(value)
            | Self::PowerAverageHighest(value)
            | Self::PowerAverageLowest(value)
            | Self::PowerInput(value)
            | Self::PowerInputHighest(value)
            | Self::PowerInputLowest(value)
            | Self::PowerCap(value)
            | Self::PowerCapHysteresis(value)
            | Self::PowerMaximum(value)
            | Self::PowerCritical(value)
            | Self::PowerMinimum(value)
            | Self::PowerLCritical(value) => write!(f, "{} {}", value, Unit::Watt),

            Self::PowerAverageInterval(value) => write!(f, "{} {}", value, Unit::Second),

            Self::EnergyInput(value) => write!(f, "{} {}", value, Unit::Joule),

            Self::CurrentInput(value)
            | Self::CurrentMinimum(value)
            | Self::CurrentMaximum(value)
            | Self::CurrentLCritical(value)
            | Self::CurrentCritical(value)
            | Self::CurrentAverage(value)
            | Self::CurrentLowest(value)
            | Self::CurrentHighest(value) => write!(f, "{} {}", value, Unit::Amp),

            Self::HumidityInput(value) => write!(f, "{} {}", value, Unit::Percentage),

            Self::Unknown { .. } => write!(f, "\u{fffd}"),
        }
    }
}

/// Type of the value of a sensor or actuator.
#[allow(missing_docs)] // Enum variant names are self-explanatory.
#[repr(u32)]
#[non_exhaustive]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
pub enum Kind {
    VoltageInput = SENSORS_SUBFEATURE_IN_INPUT,
    VoltageMinimum = SENSORS_SUBFEATURE_IN_MIN,
    VoltageMaximum = SENSORS_SUBFEATURE_IN_MAX,
    VoltageLCritical = SENSORS_SUBFEATURE_IN_LCRIT,
    VoltageCritical = SENSORS_SUBFEATURE_IN_CRIT,
    VoltageAverage = SENSORS_SUBFEATURE_IN_AVERAGE,
    VoltageLowest = SENSORS_SUBFEATURE_IN_LOWEST,
    VoltageHighest = SENSORS_SUBFEATURE_IN_HIGHEST,
    VoltageAlarm = SENSORS_SUBFEATURE_IN_ALARM,
    VoltageMinimumAlarm = SENSORS_SUBFEATURE_IN_MIN_ALARM,
    VoltageMaximumAlarm = SENSORS_SUBFEATURE_IN_MAX_ALARM,
    VoltageBeep = SENSORS_SUBFEATURE_IN_BEEP,
    VoltageLCriticalAlarm = SENSORS_SUBFEATURE_IN_LCRIT_ALARM,
    VoltageCriticalAlarm = SENSORS_SUBFEATURE_IN_CRIT_ALARM,

    FanInput = SENSORS_SUBFEATURE_FAN_INPUT,
    FanMinimum = SENSORS_SUBFEATURE_FAN_MIN,
    FanMaximum = SENSORS_SUBFEATURE_FAN_MAX,
    FanAlarm = SENSORS_SUBFEATURE_FAN_ALARM,
    FanFault = SENSORS_SUBFEATURE_FAN_FAULT,
    FanDivisor = SENSORS_SUBFEATURE_FAN_DIV,
    FanBeep = SENSORS_SUBFEATURE_FAN_BEEP,
    FanPulses = SENSORS_SUBFEATURE_FAN_PULSES,
    FanMinimumAlarm = SENSORS_SUBFEATURE_FAN_MIN_ALARM,
    FanMaximumAlarm = SENSORS_SUBFEATURE_FAN_MAX_ALARM,

    TemperatureInput = SENSORS_SUBFEATURE_TEMP_INPUT,
    TemperatureMaximum = SENSORS_SUBFEATURE_TEMP_MAX,
    TemperatureMaximumHysteresis = SENSORS_SUBFEATURE_TEMP_MAX_HYST,
    TemperatureMinimum = SENSORS_SUBFEATURE_TEMP_MIN,
    TemperatureCritical = SENSORS_SUBFEATURE_TEMP_CRIT,
    TemperatureCriticalHysteresis = SENSORS_SUBFEATURE_TEMP_CRIT_HYST,
    TemperatureLCritical = SENSORS_SUBFEATURE_TEMP_LCRIT,
    TemperatureEmergency = SENSORS_SUBFEATURE_TEMP_EMERGENCY,
    TemperatureEmergencyHysteresis = SENSORS_SUBFEATURE_TEMP_EMERGENCY_HYST,
    TemperatureLowest = SENSORS_SUBFEATURE_TEMP_LOWEST,
    TemperatureHighest = SENSORS_SUBFEATURE_TEMP_HIGHEST,
    TemperatureMinimumHysteresis = SENSORS_SUBFEATURE_TEMP_MIN_HYST,
    TemperatureLCriticalHysteresis = SENSORS_SUBFEATURE_TEMP_LCRIT_HYST,
    TemperatureAlarm = SENSORS_SUBFEATURE_TEMP_ALARM,
    TemperatureMaximumAlarm = SENSORS_SUBFEATURE_TEMP_MAX_ALARM,
    TemperatureMinimumAlarm = SENSORS_SUBFEATURE_TEMP_MIN_ALARM,
    TemperatureCriticalAlarm = SENSORS_SUBFEATURE_TEMP_CRIT_ALARM,
    TemperatureFault = SENSORS_SUBFEATURE_TEMP_FAULT,
    TemperatureType = SENSORS_SUBFEATURE_TEMP_TYPE,
    TemperatureOffset = SENSORS_SUBFEATURE_TEMP_OFFSET,
    TemperatureBeep = SENSORS_SUBFEATURE_TEMP_BEEP,
    TemperatureEmergencyAlarm = SENSORS_SUBFEATURE_TEMP_EMERGENCY_ALARM,
    TemperatureLCriticalAlarm = SENSORS_SUBFEATURE_TEMP_LCRIT_ALARM,

    PowerAverage = SENSORS_SUBFEATURE_POWER_AVERAGE,
    PowerAverageHighest = SENSORS_SUBFEATURE_POWER_AVERAGE_HIGHEST,
    PowerAverageLowest = SENSORS_SUBFEATURE_POWER_AVERAGE_LOWEST,
    PowerInput = SENSORS_SUBFEATURE_POWER_INPUT,
    PowerInputHighest = SENSORS_SUBFEATURE_POWER_INPUT_HIGHEST,
    PowerInputLowest = SENSORS_SUBFEATURE_POWER_INPUT_LOWEST,
    PowerCap = SENSORS_SUBFEATURE_POWER_CAP,
    PowerCapHysteresis = SENSORS_SUBFEATURE_POWER_CAP_HYST,
    PowerMaximum = SENSORS_SUBFEATURE_POWER_MAX,
    PowerCritical = SENSORS_SUBFEATURE_POWER_CRIT,
    PowerMinimum = SENSORS_SUBFEATURE_POWER_MIN,
    PowerLCritical = SENSORS_SUBFEATURE_POWER_LCRIT,
    PowerAverageInterval = SENSORS_SUBFEATURE_POWER_AVERAGE_INTERVAL,
    PowerAlarm = SENSORS_SUBFEATURE_POWER_ALARM,
    PowerCapAlarm = SENSORS_SUBFEATURE_POWER_CAP_ALARM,
    PowerMaximumAlarm = SENSORS_SUBFEATURE_POWER_MAX_ALARM,
    PowerCriticalAlarm = SENSORS_SUBFEATURE_POWER_CRIT_ALARM,
    PowerMinimumAlarm = SENSORS_SUBFEATURE_POWER_MIN_ALARM,
    PowerLCriticalAlarm = SENSORS_SUBFEATURE_POWER_LCRIT_ALARM,

    EnergyInput = SENSORS_SUBFEATURE_ENERGY_INPUT,

    CurrentInput = SENSORS_SUBFEATURE_CURR_INPUT,
    CurrentMinimum = SENSORS_SUBFEATURE_CURR_MIN,
    CurrentMaximum = SENSORS_SUBFEATURE_CURR_MAX,
    CurrentLCritical = SENSORS_SUBFEATURE_CURR_LCRIT,
    CurrentCritical = SENSORS_SUBFEATURE_CURR_CRIT,
    CurrentAverage = SENSORS_SUBFEATURE_CURR_AVERAGE,
    CurrentLowest = SENSORS_SUBFEATURE_CURR_LOWEST,
    CurrentHighest = SENSORS_SUBFEATURE_CURR_HIGHEST,
    CurrentAlarm = SENSORS_SUBFEATURE_CURR_ALARM,
    CurrentMinimumAlarm = SENSORS_SUBFEATURE_CURR_MIN_ALARM,
    CurrentMaximumAlarm = SENSORS_SUBFEATURE_CURR_MAX_ALARM,
    CurrentBeep = SENSORS_SUBFEATURE_CURR_BEEP,
    CurrentLCriticalAlarm = SENSORS_SUBFEATURE_CURR_LCRIT_ALARM,
    CurrentCriticalAlarm = SENSORS_SUBFEATURE_CURR_CRIT_ALARM,

    HumidityInput = SENSORS_SUBFEATURE_HUMIDITY_INPUT,

    VoltageID = SENSORS_SUBFEATURE_VID,

    IntrusionAlarm = SENSORS_SUBFEATURE_INTRUSION_ALARM,
    IntrusionBeep = SENSORS_SUBFEATURE_INTRUSION_BEEP,

    BeepEnable = SENSORS_SUBFEATURE_BEEP_ENABLE,

    Unknown = SENSORS_SUBFEATURE_UNKNOWN,
}

impl Kind {
    /// Return an instance from one of the `SENSORS_SUBFEATURE_*` values,
    /// *e.g.,* [`SENSORS_SUBFEATURE_TEMP_INPUT`].
    #[must_use]
    pub fn from_raw(kind: c_uint) -> Option<Self> {
        Self::try_from(kind).ok()
    }

    /// Return one of the `SENSORS_SUBFEATURE_*` values
    /// (*e.g.,* [`SENSORS_SUBFEATURE_TEMP_INPUT`]) equivalent to this instance.
    #[must_use]
    pub fn as_raw(self) -> c_uint {
        self.into()
    }

    /// Return the measurement unit of this instance.
    #[must_use]
    pub fn unit(self) -> Unit {
        match self {
            // Voltage
            Self::VoltageAlarm
            | Self::VoltageMinimumAlarm
            | Self::VoltageMaximumAlarm
            | Self::VoltageLCriticalAlarm
            | Self::VoltageCriticalAlarm
            | Self::VoltageBeep
            // Fan
            | Self::FanAlarm
            | Self::FanMinimumAlarm
            | Self::FanMaximumAlarm
            | Self::FanBeep
            | Self::FanFault
            | Self::FanDivisor
            | Self::FanPulses
            // Temperature
            | Self::TemperatureAlarm
            | Self::TemperatureMaximumAlarm
            | Self::TemperatureMinimumAlarm
            | Self::TemperatureCriticalAlarm
            | Self::TemperatureEmergencyAlarm
            | Self::TemperatureLCriticalAlarm
            | Self::TemperatureBeep
            | Self::TemperatureFault
            | Self::TemperatureOffset
            | Self::TemperatureType
            // Power
            | Self::PowerAlarm
            | Self::PowerCapAlarm
            | Self::PowerMaximumAlarm
            | Self::PowerCriticalAlarm
            | Self::PowerMinimumAlarm
            | Self::PowerLCriticalAlarm
            // Current
            | Self::CurrentAlarm
            | Self::CurrentMinimumAlarm
            | Self::CurrentMaximumAlarm
            | Self::CurrentLCriticalAlarm
            | Self::CurrentCriticalAlarm
            | Self::CurrentBeep
            // Intrusion
            | Self::IntrusionAlarm
            | Self::IntrusionBeep
            // Beep
            | Self::BeepEnable
            // Unknown
            | Self::Unknown => Unit::None,

            Self::VoltageInput
            | Self::VoltageMinimum
            | Self::VoltageMaximum
            | Self::VoltageLCritical
            | Self::VoltageCritical
            | Self::VoltageAverage
            | Self::VoltageLowest
            | Self::VoltageHighest
            | Self::VoltageID => Unit::Volt,

            Self::FanInput
            | Self::FanMinimum
            | Self::FanMaximum => Unit::RotationPerMinute,

            Self::TemperatureInput
            | Self::TemperatureMaximum
            | Self::TemperatureMinimum
            | Self::TemperatureMaximumHysteresis
            | Self::TemperatureCritical
            | Self::TemperatureCriticalHysteresis
            | Self::TemperatureLCritical
            | Self::TemperatureEmergency
            | Self::TemperatureEmergencyHysteresis
            | Self::TemperatureLowest
            | Self::TemperatureHighest
            | Self::TemperatureMinimumHysteresis
            | Self::TemperatureLCriticalHysteresis => Unit::Celcius,

            Self::PowerAverage
            | Self::PowerAverageHighest
            | Self::PowerAverageLowest
            | Self::PowerInput
            | Self::PowerInputHighest
            | Self::PowerInputLowest
            | Self::PowerCap
            | Self::PowerCapHysteresis
            | Self::PowerMaximum
            | Self::PowerCritical
            | Self::PowerMinimum
            | Self::PowerLCritical => Unit::Watt,

            Self::PowerAverageInterval => Unit::Second,

            Self::EnergyInput => Unit::Joule,

            Self::CurrentInput
            | Self::CurrentMinimum
            | Self::CurrentMaximum
            | Self::CurrentLCritical
            | Self::CurrentCritical
            | Self::CurrentAverage
            | Self::CurrentLowest
            | Self::CurrentHighest => Unit::Amp,

            Self::HumidityInput => Unit::Percentage,
        }
    }
}

impl Default for Kind {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::VoltageInput => "VoltageInput",
            Self::VoltageMinimum => "VoltageMinimum",
            Self::VoltageMaximum => "VoltageMaximum",
            Self::VoltageLCritical => "VoltageLCritical",
            Self::VoltageCritical => "VoltageCritical",
            Self::VoltageAverage => "VoltageAverage",
            Self::VoltageLowest => "VoltageLowest",
            Self::VoltageHighest => "VoltageHighest",
            Self::VoltageAlarm => "VoltageAlarm",
            Self::VoltageMinimumAlarm => "VoltageMinimumAlarm",
            Self::VoltageMaximumAlarm => "VoltageMaximumAlarm",
            Self::VoltageBeep => "VoltageBeep",
            Self::VoltageLCriticalAlarm => "VoltageLCriticalAlarm",
            Self::VoltageCriticalAlarm => "VoltageCriticalAlarm",
            Self::FanInput => "FanInput",
            Self::FanMinimum => "FanMinimum",
            Self::FanMaximum => "FanMaximum",
            Self::FanAlarm => "FanAlarm",
            Self::FanFault => "FanFault",
            Self::FanDivisor => "FanDivisor",
            Self::FanBeep => "FanBeep",
            Self::FanPulses => "FanPulses",
            Self::FanMinimumAlarm => "FanMinimumAlarm",
            Self::FanMaximumAlarm => "FanMaximumAlarm",
            Self::TemperatureInput => "TemperatureInput",
            Self::TemperatureMaximum => "TemperatureMaximum",
            Self::TemperatureMaximumHysteresis => "TemperatureMaximumHysteresis",
            Self::TemperatureMinimum => "TemperatureMinimum",
            Self::TemperatureCritical => "TemperatureCritical",
            Self::TemperatureCriticalHysteresis => "TemperatureCriticalHysteresis",
            Self::TemperatureLCritical => "TemperatureLCritical",
            Self::TemperatureEmergency => "TemperatureEmergency",
            Self::TemperatureEmergencyHysteresis => "TemperatureEmergencyHysteresis",
            Self::TemperatureLowest => "TemperatureLowest",
            Self::TemperatureHighest => "TemperatureHighest",
            Self::TemperatureMinimumHysteresis => "TemperatureMinimumHysteresis",
            Self::TemperatureLCriticalHysteresis => "TemperatureLCriticalHysteresis",
            Self::TemperatureAlarm => "TemperatureAlarm",
            Self::TemperatureMaximumAlarm => "TemperatureMaximumAlarm",
            Self::TemperatureMinimumAlarm => "TemperatureMinimumAlarm",
            Self::TemperatureCriticalAlarm => "TemperatureCriticalAlarm",
            Self::TemperatureFault => "TemperatureFault",
            Self::TemperatureType => "TemperatureType",
            Self::TemperatureOffset => "TemperatureOffset",
            Self::TemperatureBeep => "TemperatureBeep",
            Self::TemperatureEmergencyAlarm => "TemperatureEmergencyAlarm",
            Self::TemperatureLCriticalAlarm => "TemperatureLCriticalAlarm",
            Self::PowerAverage => "PowerAverage",
            Self::PowerAverageHighest => "PowerAverageHighest",
            Self::PowerAverageLowest => "PowerAverageLowest",
            Self::PowerInput => "PowerInput",
            Self::PowerInputHighest => "PowerInputHighest",
            Self::PowerInputLowest => "PowerInputLowest",
            Self::PowerCap => "PowerCap",
            Self::PowerCapHysteresis => "PowerCapHysteresis",
            Self::PowerMaximum => "PowerMaximum",
            Self::PowerCritical => "PowerCritical",
            Self::PowerMinimum => "PowerMinimum",
            Self::PowerLCritical => "PowerLCritical",
            Self::PowerAverageInterval => "PowerAverageInterval",
            Self::PowerAlarm => "PowerAlarm",
            Self::PowerCapAlarm => "PowerCapAlarm",
            Self::PowerMaximumAlarm => "PowerMaximumAlarm",
            Self::PowerCriticalAlarm => "PowerCriticalAlarm",
            Self::PowerMinimumAlarm => "PowerMinimumAlarm",
            Self::PowerLCriticalAlarm => "PowerLCriticalAlarm",
            Self::EnergyInput => "EnergyInput",
            Self::CurrentInput => "CurrentInput",
            Self::CurrentMinimum => "CurrentMinimum",
            Self::CurrentMaximum => "CurrentMaximum",
            Self::CurrentLCritical => "CurrentLCritical",
            Self::CurrentCritical => "CurrentCritical",
            Self::CurrentAverage => "CurrentAverage",
            Self::CurrentLowest => "CurrentLowest",
            Self::CurrentHighest => "CurrentHighest",
            Self::CurrentAlarm => "CurrentAlarm",
            Self::CurrentMinimumAlarm => "CurrentMinimumAlarm",
            Self::CurrentMaximumAlarm => "CurrentMaximumAlarm",
            Self::CurrentBeep => "CurrentBeep",
            Self::CurrentLCriticalAlarm => "CurrentLCriticalAlarm",
            Self::CurrentCriticalAlarm => "CurrentCriticalAlarm",
            Self::HumidityInput => "HumidityInput",
            Self::VoltageID => "VoltageID",
            Self::IntrusionAlarm => "IntrusionAlarm",
            Self::IntrusionBeep => "IntrusionBeep",
            Self::BeepEnable => "BeepEnable",
            Self::Unknown => "Unknown",
        };
        write!(f, "{name}")
    }
}

/// Unit of a value of a sensor or actuator.
#[allow(missing_docs)] // Enum variant names are self-explanatory.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    None,
    Volt,
    Amp,
    Watt,
    Joule,
    Celcius,
    Second,
    RotationPerMinute,
    Percentage,
}

impl Default for Unit {
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Unit::None => Ok(()),
            Unit::Volt => write!(f, "V"),
            Unit::Amp => write!(f, "A"),
            Unit::Watt => write!(f, "W"),
            Unit::Joule => write!(f, "J"),
            Unit::Celcius => write!(f, "C"),
            Unit::Second => write!(f, "s"),
            Unit::RotationPerMinute => write!(f, "RPM"),
            Unit::Percentage => write!(f, "%"),
        }
    }
}

/// Type of a temperature sensor.
#[allow(missing_docs)] // Enum variant names are self-explanatory.
#[non_exhaustive]
#[repr(i32)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
)]
pub enum TemperatureSensorKind {
    Disabled = 0_i32,
    CPUDiode = 1_i32,
    Transistor = 2_i32,
    ThermalDiode = 3_i32,
    Thermistor = 4_i32,
    AMDAMDSI = 5_i32,
    IntelPECI = 6_i32,
}

impl TemperatureSensorKind {
    /// Return an instance given a raw value, if it is valid.
    #[must_use]
    pub fn from_raw(value: f64) -> Option<Self> {
        if value.is_nan() {
            return None;
        } else if value.is_infinite() {
            return value.is_sign_positive().then_some(Self::Thermistor);
        }

        // Safety: `value` is finite.
        let int_value: i64 = unsafe { value.round().to_int_unchecked() };
        if int_value < 0 {
            None
        } else if int_value > 1000 {
            Some(Self::Thermistor)
        } else {
            // 0 <= value <= 1000
            Self::try_from(int_value as c_int).ok()
        }
    }

    /// Return the raw value equivalent to this instance.
    #[must_use]
    pub fn as_raw(self) -> c_int {
        self.into()
    }
}

impl Default for TemperatureSensorKind {
    fn default() -> Self {
        Self::Disabled
    }
}

impl fmt::Display for TemperatureSensorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Disabled => write!(f, "Disabled"),
            Self::CPUDiode => write!(f, "CPU diode"),
            Self::Transistor => write!(f, "Transistor"),
            Self::ThermalDiode => write!(f, "Thermal diode"),
            Self::Thermistor => write!(f, "Thermistor"),
            Self::AMDAMDSI => write!(f, "AMD AMDSI"),
            Self::IntelPECI => write!(f, "Intel PECI"),
        }
    }
}
