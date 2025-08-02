# Troubleshooting Guide

## Programming Issues

### "No connected probes were found"

**Symptoms**: `probe-rs list` shows nothing
**Solutions**:

1. ✅ Connect micro:bit **directly** to computer (not through BitMaker)
2. ✅ Try different USB cable (must support data, not just power)
3. ✅ Reset micro:bit: Hold reset button while plugging in USB
4. ✅ Try different USB port on computer

### BitMaker interferes with programming

**Symptoms**: Works direct, fails with BitMaker
**Solutions**:

1. ✅ This is normal - use direct connection for programming only
2. ✅ BitMaker is for operation, not programming
3. ✅ Follow programming → operation workflow

## Sensor Issues

### Sensor not detected

**Symptoms**: Sensor serial number fails or returns error
**Solutions**:

1. ✅ Check Grove cable connection and orientation
2. ✅ Verify I2C pins: SCL=P19, SDA=P20
3. ✅ Ensure SCD-40 is powered (3.3V)
4. ✅ Try different Grove cable

### Unrealistic readings

**Symptoms**: CO2 shows 0, very high values, or doesn't change
**Solutions**:

1. ✅ Wait 5-10 minutes for sensor warm-up
2. ✅ Check for loose connections
3. ✅ Verify sensor isn't covered or blocked
4. ✅ SCD-40 needs airflow to function properly

## Power Issues

### Setup doesn't power on

**Symptoms**: No LEDs, no activity
**Solutions**:

1. ✅ Check BitMaker switch position
2. ✅ Try different power source (USB vs battery)
3. ✅ Verify all connections are secure
4. ✅ Test micro:bit alone first

### Intermittent operation

**Symptoms**: Sometimes works, sometimes doesn't
**Solutions**:

1. ✅ Check for loose Grove connections
2. ✅ Verify power supply stability
3. ✅ Look for physical stress on connections

## Development Issues

### Build errors

**Symptoms**: Cargo compilation fails
**Solutions**:

1. ✅ Ensure Rust embedded toolchain installed
2. ✅ Check target architecture: `thumbv7em-none-eabihf`
3. ✅ Update dependencies: `cargo update`

### RTT/Logging not working

**Symptoms**: No debug output visible
**Solutions**:

1. ✅ Use `probe-rs attach --chip nRF52833_xxAA` for RTT
2. ✅ Ensure debug symbols: `debug = 2` in Cargo.toml
3. ✅ Check defmt configuration

## Getting More Help

- Check connections against [wiring diagrams](../schematics/connections.md)
- Review [setup guide](setup-guide.md) step by step
- Compare your setup to [reference photos](../images/setup-photos/)
- Post issues on original repo: [therustybits/moxi](https://github.com/therustybits/moxi)
