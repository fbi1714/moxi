# Connection Guide

## System Overview

The Moxi system connects three main components:

- **BBC micro:bit v2** (nRF52833 microcontroller)
- **Seeed Studio BitMaker V2** (expansion board)
- **Adafruit SCD-40** (CO2/humidity/temperature sensor)

## Hardware Reference Documents

For detailed schematics and specifications, see:

- **micro:bit v2 schematic**: [microbit-v2-schematic.pdf](microbit-v2-schematic.pdf)
- **micro:bit v2 BOM**: [../bom/microbit-v2-bom.csv](../bom/microbit-v2-bom.csv)
- **SCD-40 datasheet**: [../datasheets/Sensirion_CO2_Sensors_SCD4x_Datasheet.pdf](../datasheets/Sensirion_CO2_Sensors_SCD4x_Datasheet.pdf)
- **Adafruit SCD-40 guide**: [../datasheets/adafruit-scd-40-and-scd-41.pdf](../datasheets/adafruit-scd-40-and-scd-41.pdf)

## I2C Connection (SCD-40 to BitMaker V2)

### Grove I2C Port Wiring

The SCD-40 connects to the BitMaker V2 via the I2C Grove port using a standard Grove cable:

```
SCD-40 Breakout    Grove Cable    BitMaker V2 I2C Port
───────────────    ───────────    ────────────────────
VIN (3.3V)    ←→   Red      ←→   VCC (3.3V)
GND           ←→   Black    ←→   GND
SCL           ←→   White    ←→   SCL
SDA           ←→   Yellow   ←→   SDA
```

### micro:bit I2C Pin Mapping

When using the BitMaker V2, the I2C signals are routed to specific micro:bit pins:

| Signal | micro:bit Pin | BitMaker Grove | Function     |
| ------ | ------------- | -------------- | ------------ |
| SDA    | P20 (Pin 20)  | Yellow wire    | I2C Data     |
| SCL    | P19 (Pin 19)  | White wire     | I2C Clock    |
| VCC    | 3V3           | Red wire       | Power (3.3V) |
| GND    | GND           | Black wire     | Ground       |

### SCD-40 I2C Address

- **Default I2C Address**: `0x62` (98 decimal)
- **7-bit addressing**: Standard I2C protocol
- **Speed**: Supports up to 100 kHz (standard mode)

## Power Configuration

### Programming Mode (Development)

```
Computer ──USB──→ micro:bit v2
                      │
                  (no power to BitMaker)
```

- **Use for**: Flashing firmware, debugging
- **Connection**: USB cable directly to micro:bit
- **BitMaker**: Disconnected or unpowered
- **Reason**: BitMaker interferes with CMSIS-DAP debug interface

### Operation Mode (Running)

```
Power Source ──USB──→ BitMaker V2 ──Edge Connector──→ micro:bit v2
                          │                              │
                     Grove I2C                      (runs firmware)
                          │
                      SCD-40 Sensor
```

- **Use for**: Normal operation, sensor readings
- **Connection**: USB/battery to BitMaker micro USB port
- **Power flow**: BitMaker → micro:bit → sensors
- **Switch position**: Set BitMaker switch appropriately

### Portable Mode (Battery)

```
Battery Pack ──JST──→ BitMaker V2 ──Edge Connector──→ micro:bit v2
                          │                              │
                     Grove I2C                      (runs firmware)
                          │
                      SCD-40 Sensor
```

- **Power source**: 3xAA or Li-Po battery via JST connector
- **Runtime**: Depends on battery capacity and sensor polling rate
- **Voltage**: 3.0-4.5V input range (check BitMaker specs)

## Physical Assembly

### Step 1: micro:bit to BitMaker Connection

```
        ┌─────────────────────────────────────┐
        │           micro:bit v2              │
        │  ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○ ○  │
        └─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┘
          │ │ │ │ │ │ │ │ │ │ │ │ │ │ │ │ │ │ │ │ │
        ┌─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┐
        │              BitMaker V2               │
        │  [Grove] [Grove] [Grove] [Grove]       │
        │     I2C    GPIO    GPIO    GPIO        │
        └─────────────────────────────────────────┘
```

### Step 2: SCD-40 to Grove I2C Connection

```
Grove Cable Color Coding:
┌─────────────┐    4-wire Grove Cable    ┌─────────────┐
│   SCD-40    │                          │ BitMaker V2 │
│             │ Red    (VCC) ←──────→    │ I2C Grove   │
│ Breakout    │ Black  (GND) ←──────→    │   Port      │
│   Board     │ White  (SCL) ←──────→    │             │
│             │ Yellow (SDA) ←──────→    │             │
└─────────────┘                          └─────────────┘
```

**Important**: Grove connector orientation matters! Yellow wire should be toward the center/inside of the BitMaker board.

## Signal Integrity Notes

### I2C Pull-up Resistors

- **SCD-40 breakout**: Has onboard pull-up resistors (typically 10kΩ)
- **BitMaker V2**: May have additional pull-ups on I2C lines
- **micro:bit**: Internal pull-ups available but not required with external ones
- **Result**: Should work without additional external pull-ups

### Power Supply Considerations

- **SCD-40 operating voltage**: 2.4V to 5.5V (3.3V recommended)
- **micro:bit I/O voltage**: 3.3V logic levels
- **BitMaker Grove ports**: 3.3V supply
- **Compatibility**: Perfect match, no level shifting needed

### Cable Length Limitations

- **Grove cable**: Standard 20cm length is fine for I2C
- **I2C maximum**: Generally <1 meter for reliable communication
- **Recommendation**: Keep connections short and secure

## Troubleshooting Connections

### Common Connection Issues

1. **Grove cable orientation**: Yellow wire positioned incorrectly
2. **Loose connections**: Grove connectors not fully seated
3. **Wrong Grove port**: Using GPIO port instead of I2C port
4. **Power conflicts**: Both micro:bit and BitMaker powered simultaneously

### Testing Connections

```rust
// In your Rust code, verify I2C communication:
let serial_number = sensor.serial_number();
match serial_number {
    Ok(sn) => info!("Sensor detected: {}", sn),
    Err(_) => error!("Sensor not detected - check connections"),
}
```

### Expected Sensor Response

- **Serial number**: Should return a valid number (e.g., 3286774528792)
- **First reading delay**: SCD-40 needs ~5 seconds for first measurement
- **Typical indoor values**: CO2: 400-1000ppm, Temp: 18-25°C, Humidity: 30-70%

## Pin Reference Summary

| Function | micro:bit Pin | BitMaker Connection   | Notes            |
| -------- | ------------- | --------------------- | ---------------- |
| I2C SDA  | P20           | Grove I2C Yellow      | Data line        |
| I2C SCL  | P19           | Grove I2C White       | Clock line       |
| Power    | 3V3           | Grove I2C Red         | 3.3V supply      |
| Ground   | GND           | Grove I2C Black       | Common ground    |
| USB Data | USB+/-        | (blocked by BitMaker) | Programming only |

For additional GPIO usage and advanced connections, refer to the complete micro:bit schematic in this directory.
