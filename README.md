[![crates.io](https://img.shields.io/crates/v/lm-sensors.svg)](https://crates.io/crates/lm-sensors)
[![docs.rs](https://docs.rs/lm-sensors/badge.svg)](https://docs.rs/lm-sensors)
[![license](https://img.shields.io/github/license/koutheir/lm-sensors?color=black)](https://raw.githubusercontent.com/koutheir/lm-sensors/master/LICENSE.txt)

# Hardware monitoring in Linux based on LM Sensors

`lm-sensors` provides user-space support for the hardware monitoring drivers
in Linux.

This crate is Linux-specific. Building it for non-Linux platforms, or for
the Linux kernel, results in an empty crate.

## Listing all available sensors

```rust
// Import all useful traits of this crate.
use lm_sensors::prelude::*;

// Initialize LM sensors library.
let sensors = lm_sensors::Initializer::default().initialize()?;

// Print all chips.
for chip in sensors.chip_iter(None) {
    if let Some(path) = chip.path() {
        println!("chip: {} at {} ({})", chip, chip.bus(), path.display());
    } else {
        println!("chip: {} at {}", chip, chip.bus());
    }

    // Print all features of the current chip.
    for feature in chip.feature_iter() {
        let name = feature.name().transpose()?.unwrap_or("N/A");
        println!("    {}: {}", name, feature);

        // Print all sub-features of the current chip feature.
        for sub_feature in feature.sub_feature_iter() {
            if let Ok(value) = sub_feature.value() {
                println!("        {}: {}", sub_feature, value);
            } else {
                println!("        {}: N/A", sub_feature);
            }
        }
    }
}
```

The following is an example output of the sample above:

```text
chip: iwlwifi_1-virtual-0 at Virtual device (/sys/class/hwmon/hwmon8)
    temp1: temp1
        temp1_input: N/A
chip: thinkpad-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon6)
    fan1: fan1
        fan1_input: 2488
    fan2: fan2
        fan2_input: 2262
    temp1: CPU
        temp1_input: 54
    temp2: GPU
        temp2_input: 50
    temp3: temp3
        temp3_input: 57
    temp4: temp4
        temp4_input: 0
    temp5: temp5
        temp5_input: 54
    temp6: temp6
        temp6_input: 58
    temp7: temp7
        temp7_input: 60
    temp8: temp8
        temp8_input: 0
chip: ucsi_source_psy_USBC000:002-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon10)
    in0: in0
        in0_input: 5
        in0_min: 5
        in0_max: 5
    curr1: curr1
        curr1_input: 5
        curr1_max: 5
chip: coretemp-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon4)
    temp1: Package id 0
        temp1_input: 53
        temp1_max: 53
        temp1_crit: 53
        temp1_crit_alarm: 53
    temp2: Core 0
        temp2_input: 100
        temp2_max: 100
        temp2_crit: 100
        temp2_crit_alarm: 100
    temp3: Core 1
        temp3_input: 100
        temp3_max: 100
        temp3_crit: 100
        temp3_crit_alarm: 100
    temp4: Core 2
        temp4_input: 0
        temp4_max: 0
        temp4_crit: 0
        temp4_crit_alarm: 0
    temp5: Core 3
        temp5_input: 58
        temp5_max: 58
        temp5_crit: 58
        temp5_crit_alarm: 58
    temp6: Core 4
        temp6_input: 100
        temp6_max: 100
        temp6_crit: 100
        temp6_crit_alarm: 100
    temp7: Core 5
        temp7_input: 100
        temp7_max: 100
        temp7_crit: 100
        temp7_crit_alarm: 100
    temp8: Core 6
        temp8_input: 0
        temp8_max: 0
        temp8_crit: 0
        temp8_crit_alarm: 0
    temp9: Core 7
        temp9_input: 57
        temp9_max: 57
        temp9_crit: 57
        temp9_crit_alarm: 57
chip: nvme-pci-5500 at PCI adapter (/sys/class/hwmon/hwmon2)
    temp1: Composite
        temp1_input: 46.85
        temp1_max: 46.85
        temp1_min: 46.85
        temp1_crit: 46.85
        temp1_alarm: 46.85
    temp2: Sensor 1
        temp2_input: 83.85
        temp2_max: 83.85
        temp2_min: 83.85
    temp3: Sensor 2
        temp3_input: -273.15
        temp3_max: -273.15
        temp3_min: -273.15
chip: acpitz-acpi-0 at ACPI interface (/sys/class/hwmon/hwmon0)
    temp1: temp1
        temp1_input: 54
        temp1_crit: 54
chip: ucsi_source_psy_USBC000:001-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon9)
    in0: in0
        in0_input: 0
        in0_min: 0
        in0_max: 0
    curr1: curr1
        curr1_input: 0
        curr1_max: 0
chip: pch_cometlake-virtual-0 at Virtual device (/sys/class/hwmon/hwmon7)
    temp1: temp1
        temp1_input: 58
chip: BAT0-acpi-0 at ACPI interface (/sys/class/hwmon/hwmon3)
    in0: in0
        in0_input: 12.221
chip: nvme-pci-0200 at PCI adapter (/sys/class/hwmon/hwmon1)
    temp1: Composite
        temp1_input: 39.85
        temp1_max: 39.85
        temp1_min: 39.85
        temp1_crit: 39.85
        temp1_alarm: 39.85
    temp2: Sensor 1
        temp2_input: 84.85
        temp2_max: 84.85
        temp2_min: 84.85
    temp3: Sensor 2
        temp3_input: -273.15
        temp3_max: -273.15
        temp3_min: -273.15
```

## Custom configuration and behavior of LM sensors library

### Loading a custom configuration file

```rust
// Import all useful traits of this crate.
use lm_sensors::prelude::*;

// Initialize LM sensors library with a custom configuration file.
let sensors = lm_sensors::Initializer::default()
    .config_path("/dev/null")
    .initialize()?;
```

```rust
// Import all useful traits of this crate.
use lm_sensors::prelude::*;

let config_file = File::open("/dev/null").unwrap();

// Initialize LM sensors library with a custom configuration file.
let sensors = lm_sensors::Initializer::default()
    .config_file(config_file)
    .initialize()?;
```

### Setting custom error reporting

```rust
// Import all useful traits of this crate.
use lm_sensors::prelude::*;

#[derive(Debug)]
struct EL;

impl lm_sensors::errors::Listener for EL {
    fn on_lm_sensors_config_error(&self, error: &str,
        file_name: Option<&std::path::Path>, line_number: usize)
    {
        if let Some(file_name) = file_name {
            eprintln!("[ERROR] lm-sensors config: {} @{}:{}",
                      error, file_name.display(), line_number);
        } else {
            eprintln!("[ERROR] lm-sensors config: {} @<config>:{}",
                      error, line_number);
        }
    }

    fn on_lm_sensors_fatal_error(&self, error: &str, procedure: &str) {
        eprintln!("[FATAL] lm-sensors: {} @{}", error, procedure);
    }
}

// Initialize LM sensors library with custom error reporting.
let sensors = lm_sensors::Initializer::default()
    .error_listener(Box::new(EL))
    .initialize()?;
```

## Versioning

This project adheres to [Semantic Versioning].
The `CHANGELOG.md` file details notable changes over time.

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
