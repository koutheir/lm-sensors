# Change log

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2023-10-09

### Removed

> ⚠️ **All the following are breaking changes**.

- `lm_sensors::bus::ExclusiveBus`
- `lm_sensors::bus::SharedBus`
- `lm_sensors::BusMut`
- `lm_sensors::BusRef`
- `lm_sensors::chip::SharedChip`
- `lm_sensors::LMSensors::new_bus_mut`
- `lm_sensors::LMSensors::new_bus_ref`
- `lm_sensors::prelude`

### Changed

> ⚠️ **All the following are breaking changes**.

- `AsMut<sensors_chip_name> for Chip` replaced by `lm_sensors::chip::Chip::raw_mut`.
- `AsRef<sensors_chip_name> for Chip` replaced by `lm_sensors::chip::Chip::raw_ref`.
- `AsRef<sensors_chip_name> for ChipRef` replaced by `lm_sensors::chip::ChipRef::raw_ref`.
- `AsRef<sensors_feature> for FeatureRef` replaced by `lm_sensors::feature::FeatureRef::raw_ref`.
- `AsRef<sensors_subfeature> for SubFeatureRef` replaced by `lm_sensors::sub_feature::SubFeatureRef::raw_ref`.
- `lm_sensors::Chip::bus_mut` replaced by `lm_sensors::chip::Chip::set_bus`.
- `lm_sensors::Chip::bus` return value is now `lm_sensors::bus::Bus`.
- `lm_sensors::ChipRef::bus` return value is now `lm_sensors::bus::Bus`.
- `lm_sensors::ChipRef` methods now accept `self` instead of `&self`.
- `lm_sensors::FeatureRef` methods now accept `self` instead of `&self`.
- `lm_sensors::SubFeatureRef` methods now accept `self` instead of `&self`.

## [0.1.7] - 2023-09-26

### Added

- Documentation on installing `libsensors`.

Thank you very much, *Carter*.

### Changed

- Updated dependencies.

## [0.1.6] - 2023-08-09

### Changed

- Updated dependencies.

## [0.1.5] - 2023-04-18

### Changed

- Updated dependencies.

## [0.1.4] - 2022-02-08

### Changed

- Failures were not correctly reported by `SubFeatureRef::raw_value()`.
- `SubFeatureRef::raw_value()` provided incorrect value to `sensors_get_value()`
  as sub-feature number.
- `SubFeatureRef::set_raw_value()` provided incorrect value
  to `sensors_set_value()` as sub-feature number.
- Updated dependencies.

Thank you very much, *Ian Douglas Scott*.

## [0.1.3] - 2021-11-27

### Changed

- Updated documentation.

## [0.1.2] - 2021-11-27

### Changed

- Updated documentation.

## [0.1.1] - 2021-11-27

### Changed

- Updated documentation.

## [0.1.0] - 2021-11-27

### Added

- Initial release.
