# `hello-nucleo-so2002`
SO2002 OLED display shown "Hello world".  
The device uses a hardware I2C interface to communicate.

# Required hardwares
- stm32nucleo development board 'NUCLEO-F302R8'
- SO2002 OLED display module [akizukidenshi.com](http://akizukidenshi.com/) 'P-08279'
- breadboard and wires

# Wiring
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

# Build
cargo build --release

# License
Licensed under

Apache License, Version 2.0 (See the 'LICENSE' file).

This project includes software licensed under the Apache License.
