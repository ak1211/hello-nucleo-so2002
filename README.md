# `hello-nucleo-so2002`
SO2002 OLED display shown "Hello world".  
The device uses a hardware I2C interface to communicate.

# Required hardwares
- stm32nucleo development board 'NUCLEO-F302R8'
- SO2002 OLED display module [akizukidenshi.com](http://akizukidenshi.com/) 'P-08279'
- breadboard and wires

# Wiring
**Note: I2C bus SCL and SDA with 2.7k pull-up resistor.**

| 'NUCLEO-F302R8' arduino connector pins | 'SO2002 OLED display' pins |
----|----
| GND (CN6)     | 1 VSS |
| 3V3 (CN6)     | 2 VDD |
| GND (CN6)     | 3 /CS |
| 3V3 (CN6)     | 4 SA0 |
| -             | 5 NC |
| -             | 6 NC |
| D15 (CN5) SCL | 7 SCL |
| D14 (CN5) SDA | 8 SDA_in |
| D14 (CN5) SDA | 9 SDA_out |
| -             | 10-14 NC|

![wiring](https://user-images.githubusercontent.com/17291748/76162385-97e07800-6180-11ea-9947-4f1edd26f7cf.JPG)
![Hello, World](https://user-images.githubusercontent.com/17291748/76162387-9d3dc280-6180-11ea-8a4f-f5ffe425a7d8.JPG)

# Build
cargo build --release

# License
Licensed under

Apache License, Version 2.0 (See the 'LICENSE' file).

This project includes software licensed under the Apache License.
