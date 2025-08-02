# Moxi Hardware Documentation

## Overview

The Moxi project implements environmental monitoring using a BBC micro:bit v2 with CO2 sensing capabilities. This system combines embedded Rust programming with modern sensor technology for real-time air quality monitoring.

### System Architecture

```
┌─────────────────┐       ┌──────────────┐       ┌─────────────┐
│   Computer      │ USB   │  micro:bit   │ Edge  │  BitMaker   │
│  (Programming)  │ ←───→ │      v2      │ ←───→ │     V2      │
└─────────────────┘       └──────────────┘       └─────────────┘
                                                         │
                                                    Grove I2C
                                                         ▼
                                                  ┌─────────────┐
                                                  │   SCD-40    │
                                                  │ CO2 Sensor  │
                                                  └─────────────┘
```

## Core Components

| Component           | Model                       | Function                        | Documentation                                                     |
| ------------------- | --------------------------- | ------------------------------- | ----------------------------------------------------------------- |
| **Microcontroller** | BBC micro:bit v2 (nRF52833) | Main processor, Rust runtime    | [schematic](schematics/microbit-v2-schematic.pdf)                 |
| **Expansion Board** | Seeed Studio BitMaker V2    | Power, Grove connectors         | [connections](schematics/connections.md)                          |
| **CO2 Sensor**      | Adafruit SCD-40             | True CO2, humidity, temperature | [datasheet](datasheets/Sensirion_CO2_Sensors_SCD4x_Datasheet.pdf) |
| **Interface**       | Grove I2C Cable             | Sensor connectivity             | [wiring guide](schematics/connections.md#grove-i2c-port-wiring)   |

## Quick Start

### 1. Assembly

- Mount micro:bit v2 on BitMaker V2 expansion board
- Connect SCD-40 to BitMaker's I2C Grove port with Grove cable
- Verify connections per [connection guide](schematics/connections.md)

### 2. Programming

- Connect micro:bit **directly** to computer (not through BitMaker)
- Flash Rust firmware: `cargo run --release`
- See [troubleshooting](assembly/troubleshooting.md) if probe-rs issues occur

### 3. Operation

- Disconnect from computer
- Power via BitMaker's micro USB port
- System automatically starts sensor readings every 5 seconds

## Current Capabilities

✅ **Working Features:**

- CO2 measurement (400-5000 ppm range)
- Humidity sensing (0-100% RH)
- Temperature monitoring (-40°C to +70°C)
- I2C communication with SCD-40
- Real-time sensor data via RTT logging
- Battery-powered portable operation

🚧 **In Development:**

- Data logging to storage
- Display output on micro:bit LED matrix
- WiFi connectivity for remote monitoring
- Multiple sensor support

📋 **Planned Features:**

- Web dashboard
- Alert thresholds
- Historical data analysis
- Multi-device coordination

## Hardware Documentation

### Bill of Materials

- **Complete BOM**: [bill-of-materials.md](bom/bill-of-materials.md) (~$82 total)
- **micro:bit v2 BOM**: [microbit-v2-bom.csv](bom/microbit-v2-bom.csv) (official components)

### Schematics & Wiring

- **Connection Guide**: [connections.md](schematics/connections.md) (I2C wiring, power modes)
- **micro:bit Schematic**: [microbit-v2-schematic.pdf](schematics/microbit-v2-schematic.pdf) (official)

### Datasheets & References

- **SCD-40 Datasheet**: [Sensirion_CO2_Sensors_SCD4x_Datasheet.pdf](datasheets/Sensirion_CO2_Sensors_SCD4x_Datasheet.pdf)
- **Adafruit SCD-40 Guide**: [adafruit-scd-40-and-scd-41.pdf](datasheets/adafruit-scd-40-and-scd-41.pdf)
- **Additional References**: [datasheet-links.md](datasheets/datasheet-links.md)
- **About BitMaker V2 Expansion Board**
  - [3.2 BitMaker](https://www.yuque.com/tinkergen-help-en/microbit/bitmaker)

  - [BitMaker | Seeed Studio Wiki](https://wiki.seeedstudio.com/BitMaker/)

  - [Search results for: 'BitMaker JST 2.0'](https://www.seeedstudio.com/catalogsearch/result/?q=BitMaker+JST+2.0&warehouse=CN+Warehouse+In+Stock%2CCN+or+US+Warehouse+In+Stock%2CCN+or+DE+Warehouse+In+Stock)
  - [CH_BitMaker_V2 - JST 2.0 battery connector & 6 Grove connectors - Seeed Studio](https://www.seeedstudio.com/CH-BitMaker-V2-p-5330.html)
  - [BitMaker - Grove Expansion Board for Micro:bit (6 Grove ports) - Seeed Studio](https://www.seeedstudio.com/BitMaker-p-4353.html)
  - [BitMaker Lite - Grove Expansion Board for Micro:bit (3 Grove Ports) - Seeed Studio](https://www.seeedstudio.com/BitMaker-Lite-p-4354.html)
  - [CH_BitMaker_V2 - JST 2.0 battery connector & 6 Grove connectors - Seeed Studio](https://www.seeedstudio.com/CH-BitMaker-V2-p-5330.html)

### Assembly & Support

- **Setup Guide**: [setup-guide.md](assembly/setup-guide.md) (step-by-step assembly)
- **Troubleshooting**: [troubleshooting.md](assembly/troubleshooting.md) (common issues & solutions)

## Technical Specifications

### System Requirements

- **Power**: 5V USB or 3.7V Li-Po battery
- **Operating Temperature**: 0°C to +50°C (limited by micro:bit)
- **Humidity Range**: 0-95% RH non-condensing
- **Communication**: I2C (100 kHz), USB for programming

### Performance Characteristics

- **CO2 Accuracy**: ±(40 ppm + 5% of reading)
- **CO2 Resolution**: 1 ppm
- **Measurement Interval**: 5 seconds (configurable)
- **Warm-up Time**: <60 seconds
- **Response Time**: <60 seconds (63% step change)

### Pin Usage

| micro:bit Pin | Function | BitMaker Connection     |
| ------------- | -------- | ----------------------- |
| P19           | I2C SCL  | Grove I2C (white wire)  |
| P20           | I2C SDA  | Grove I2C (yellow wire) |
| 3V3           | Power    | Grove I2C (red wire)    |
| GND           | Ground   | Grove I2C (black wire)  |

## Known Issues & Limitations

### Hardware Limitations

- **Programming interference**: BitMaker blocks CMSIS-DAP when powered
- **Single I2C bus**: Limited to one sensor per Grove I2C port
- **Power switching**: Manual workflow for programming vs operation modes

### Software Limitations

- **No persistent storage**: Data lost on power cycle
- **RTT-only output**: No local display of readings yet
- **Fixed polling rate**: 5-second interval hardcoded

### Workarounds

- Use direct micro:bit connection for all programming/debugging
- Power through BitMaker only during operation
- Monitor via RTT: `probe-rs attach --chip nRF52833_xxAA`

## Getting Help

### Documentation Order

1. **Start here**: hardware/README.md (this file)
2. **Assembly**: [setup-guide.md](assembly/setup-guide.md)
3. **Connections**: [connections.md](schematics/connections.md)
4. **Problems**: [troubleshooting.md](assembly/troubleshooting.md)

### External Resources

- **Original Project**: [therustybits/moxi](https://github.com/therustybits/moxi)
- **micro:bit Foundation**: [microbit.org](https://microbit.org/)
- **Rust Embedded**: [docs.rs/microbit](https://docs.rs/microbit/)
- **SCD-40 Support**: [sensirion.com](https://sensirion.com/scd40/)

### Community

- **Issues**: Report hardware issues on the main repo
- **Discussions**: Share setup photos and improvements
- **Contributions**: Hardware documentation improvements welcome

---

**Project Status**: Active development | **Last Updated**: August 2025 | **Version**: 0.1.0
