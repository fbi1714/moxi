# Setup Guide

## Prerequisites

- All components from [Bill of Materials](../bom/bill-of-materials.md)
- Computer with USB-C or USB-A port
- Rust development environment set up

## Step 1: Hardware Assembly

### 1.1 Connect micro:bit to BitMaker

1. Align the micro:bit's edge connector with BitMaker's socket
2. Press down firmly until fully seated
3. micro:bit should sit flat on the BitMaker board

### 1.2 Connect SCD-40 Sensor

1. Take the Grove cable (4-wire)
2. Connect one end to SCD-40 breakout board
3. Connect other end to BitMaker's I2C Grove port
4. **Ensure proper orientation**: Yellow wire toward center of board

### 1.3 Power Connection Options

- **For programming**: USB cable to micro:bit only
- **For operation**: USB cable to BitMaker micro USB port
- **For portable use**: Battery pack to JST connector

## Step 2: Software Setup

### 2.1 Clone and Build

```bash
git clone https://github.com/yourusername/moxi.git
cd moxi/ep1_i2csensor
cargo build --release
```
