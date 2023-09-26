[![crates.io](https://img.shields.io/crates/v/lm-sensors.svg)](https://crates.io/crates/lm-sensors)
[![docs.rs](https://docs.rs/lm-sensors/badge.svg)](https://docs.rs/lm-sensors)
[![license](https://img.shields.io/github/license/koutheir/lm-sensors?color=black)](https://raw.githubusercontent.com/koutheir/lm-sensors/master/LICENSE.txt)

# Hardware monitoring in Linux based on LM Sensors

`lm-sensors` provides user-space support for the hardware monitoring drivers
in Linux.

This crate is Linux-specific. Building it for non-Linux platforms, or for
the Linux kernel, results in an empty crate.

This crate links to [`libsensors`](https://github.com/lm-sensors/lm-sensors), and requires it to be
installed.
Linking to `libsensors` happens transitively through depending on the
[`sensors-sys`](https://crates.io/crates/sensors-sys) crate.
The [`sensors-sys` crate documentation](https://docs.rs/sensors-sys/) illustrates, among other things,
how to install `libsensors` and control aspects of this linking.

## Listing all available sensors

```rust
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
# Ok::<(), lm_sensors::errors::Error>(())
```

The following is an example output of the sample above:

```text
chip: iwlwifi_1-virtual-0 at Virtual device (/sys/class/hwmon/hwmon8)
    temp1: temp1
        temp1_input: N/A
chip: pch_cometlake-virtual-0 at Virtual device (/sys/class/hwmon/hwmon6)
    temp1: temp1
        temp1_input: 67 C
chip: ucsi_source_psy_USBC000:001-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon10)
    in0: in0
        in0_input: 5 V
        in0_min: 5 V
        in0_max: 5 V
    curr1: curr1
        curr1_input: 0 A
        curr1_max: 1.5 A
chip: coretemp-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon4)
    temp1: Package id 0
        temp1_input: 71 C
        temp1_max: 100 C
        temp1_crit: 100 C
        temp1_crit_alarm:
    temp2: Core 0
        temp2_input: 62 C
        temp2_max: 100 C
        temp2_crit: 100 C
        temp2_crit_alarm:
    temp3: Core 1
        temp3_input: 63 C
        temp3_max: 100 C
        temp3_crit: 100 C
        temp3_crit_alarm:
    temp4: Core 2
        temp4_input: 63 C
        temp4_max: 100 C
        temp4_crit: 100 C
        temp4_crit_alarm:
    temp5: Core 3
        temp5_input: 71 C
        temp5_max: 100 C
        temp5_crit: 100 C
        temp5_crit_alarm:
    temp6: Core 4
        temp6_input: 62 C
        temp6_max: 100 C
        temp6_crit: 100 C
        temp6_crit_alarm:
    temp7: Core 5
        temp7_input: 63 C
        temp7_max: 100 C
        temp7_crit: 100 C
        temp7_crit_alarm:
    temp8: Core 6
        temp8_input: 61 C
        temp8_max: 100 C
        temp8_crit: 100 C
        temp8_crit_alarm:
    temp9: Core 7
        temp9_input: 61 C
        temp9_max: 100 C
        temp9_crit: 100 C
        temp9_crit_alarm:
chip: nvme-pci-0200 at PCI adapter (/sys/class/hwmon/hwmon2)
    temp1: Composite
        temp1_input: 52.85 C
        temp1_max: 84.85 C
        temp1_min: -273.15 C
        temp1_crit: 84.85 C
        temp1_alarm:
    temp2: Sensor 1
        temp2_input: 52.85 C
        temp2_max: 65261.85 C
        temp2_min: -273.15 C
    temp3: Sensor 2
        temp3_input: 45.85 C
        temp3_max: 65261.85 C
        temp3_min: -273.15 C
chip: acpitz-acpi-0 at ACPI interface (/sys/class/hwmon/hwmon0)
    temp1: temp1
        temp1_input: 61 C
        temp1_crit: 128 C
chip: thinkpad-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon7)
    fan1: fan1
        fan1_input: 2657 RPM
    fan2: fan2
        fan2_input: 2583 RPM
    temp1: CPU
        temp1_input: 61 C
    temp2: GPU
        temp2_input: 58 C
    temp3: temp3
        temp3_input: 64 C
    temp4: temp4
        temp4_input: 1 C
    temp5: temp5
        temp5_input: 60 C
    temp6: temp6
        temp6_input: 65 C
    temp7: temp7
        temp7_input: 68 C
    temp8: temp8
        temp8_input: 0 C
chip: ucsi_source_psy_USBC000:002-isa-0000 at ISA adapter (/sys/class/hwmon/hwmon11)
    in0: in0
        in0_input: 0 V
        in0_min: 0 V
        in0_max: 0 V
    curr1: curr1
        curr1_input: 0 A
        curr1_max: 0 A
chip: nvme-pci-5500 at PCI adapter (/sys/class/hwmon/hwmon3)
    temp1: Composite
        temp1_input: 51.85 C
        temp1_max: 83.85 C
        temp1_min: -273.15 C
        temp1_crit: 84.85 C
        temp1_alarm: 
    temp2: Sensor 1
        temp2_input: 51.85 C
        temp2_max: 65261.85 C
        temp2_min: -273.15 C
    temp3: Sensor 2
        temp3_input: 48.85 C
        temp3_max: 65261.85 C
        temp3_min: -273.15 C
chip: BAT0-acpi-0 at ACPI interface (/sys/class/hwmon/hwmon1)
    in0: in0
        in0_input: 12.255 V
```

## Versioning

This project adheres to [Semantic Versioning].
The `CHANGELOG.md` file details notable changes over time.

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
