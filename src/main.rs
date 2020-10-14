#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use stm32f30x_hal as hal;

use hal::delay::Delay;
use hal::i2c::I2c;
use hal::prelude::*;
use hal::stm32f30x;

use cortex_m_rt::entry;

// 0 1 1 1 1 0 SA0 (SA0 pin is default Hi)
// 0 1 1 1 1 0 1
// 011 1101 = (2+1)*16 + (8+4+1)*1 = 0x3d
const OLED_ADDR: u8 = 0x3d;

const USER_CHARACTERS: [[u8; 8]; 7] = [
    // character code 0x00
    [
        0b00000000, // line 1
        0b00000000, // line 2
        0b00000000, // line 3
        0b00000000, // line 4
        0b00000000, // line 5
        0b00000000, // line 6
        0b00011111, // line 7
        0b00000000, // line 8
    ],
    // character code 0x01
    [
        0b00000000, // line 1
        0b00000000, // line 2
        0b00000000, // line 3
        0b00000000, // line 4
        0b00000000, // line 5
        0b00011111, // line 6
        0b00011111, // line 7
        0b00000000, // line 8
    ],
    // character code 0x02
    [
        0b00000000, // line 1
        0b00000000, // line 2
        0b00000000, // line 3
        0b00000000, // line 4
        0b00011111, // line 5
        0b00011111, // line 6
        0b00011111, // line 7
        0b00000000, // line 8
    ],
    // character code 0x03
    [
        0b00000000, // line 1
        0b00000000, // line 2
        0b00000000, // line 3
        0b00011111, // line 4
        0b00011111, // line 5
        0b00011111, // line 6
        0b00011111, // line 7
        0b00000000, // line 8
    ],
    // character code 0x04
    [
        0b00000000, // line 1
        0b00000000, // line 2
        0b00011111, // line 3
        0b00011111, // line 4
        0b00011111, // line 5
        0b00011111, // line 6
        0b00011111, // line 7
        0b00000000, // line 8
    ],
    // character code 0x05
    [
        0b00000000, // line 1
        0b00011111, // line 2
        0b00011111, // line 3
        0b00011111, // line 4
        0b00011111, // line 5
        0b00011111, // line 6
        0b00011111, // line 7
        0b00000000, // line 8
    ],
    // character code 0x06
    [
        0b00011111, // line 1
        0b00011111, // line 2
        0b00011111, // line 3
        0b00011111, // line 4
        0b00011111, // line 5
        0b00011111, // line 6
        0b00011111, // line 7
        0b00000000, // line 8
    ],
];

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    // OLED display I2C interface
    let scl = gpiob.pb8.into_af4(&mut gpiob.moder, &mut gpiob.afrh);
    let sda = gpiob.pb9.into_af4(&mut gpiob.moder, &mut gpiob.afrh);
    let mut display = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    // Wait 100ms until module control, recommended
    delay.delay_ms(100u8);

    // Set CGRAM
    for (i, ch) in USER_CHARACTERS.iter().enumerate() {
        let code = i as u8;
        display
            .write(OLED_ADDR, &[0x00u8, 0x40u8 | (code << 3) | 0])
            .unwrap();
        delay.delay_ms(1u8);
        for v in ch {
            display.write(OLED_ADDR, &[0x40u8, *v]).unwrap();
            delay.delay_ms(1u8);
        }
    }

    // Clear Display
    display.write(OLED_ADDR, &[0x00u8, 0x01u8]).unwrap();
    delay.delay_ms(20u8);
    // Return Home
    display.write(OLED_ADDR, &[0x00u8, 0x02u8]).unwrap();
    delay.delay_ms(2u8);
    // Display ON, Cursor OFF, Blink OFF
    display.write(OLED_ADDR, &[0x00u8, 0x0Cu8]).unwrap();
    delay.delay_ms(2u8);
    // Clear Display
    display.write(OLED_ADDR, &[0x00u8, 0x01u8]).unwrap();
    delay.delay_ms(20u8);
    // Function Set (N = 2, DH = 0, RE = 1, IS = 0)
    display.write(OLED_ADDR, &[0x00u8, 0x2Au8]).unwrap();
    delay.delay_ms(1u8);
    // OLED Characterization (SD = 1)
    display.write(OLED_ADDR, &[0x00u8, 0x79u8]).unwrap();
    delay.delay_ms(1u8);
    // contrast set
    display.write(OLED_ADDR, &[0x00u8, 0x81u8]).unwrap();
    delay.delay_ms(1u8);
    // Set Contrast Control
    display.write(OLED_ADDR, &[0x00u8, 0xFFu8]).unwrap();
    delay.delay_ms(1u8);
    // OLED Characterization (SD = 0)
    display.write(OLED_ADDR, &[0x00u8, 0x78u8]).unwrap();
    delay.delay_ms(2u8);
    // Function Set (N = 2, DH = 0, RE = 0, IS = 0)
    display.write(OLED_ADDR, &[0x00u8, 0x28u8]).unwrap();
    delay.delay_ms(1u8);
 
    //
    // Print to display.
    //
    let message = b"I2C OLED Yellow 20x2Hello World";

    const BUF_SIZE: usize = 1 + 20 * 02; // 1 byte and 20 chars * 02 lines
    let mut write_buf: [u8; BUF_SIZE] = [b' '; BUF_SIZE]; // clear to whitespace

    // first byte is set to D/C bit Hi
    write_buf[0] = 0x40u8;

    // after first bytes
    write_buf[1..=message.len()].copy_from_slice(message);

    // send to display.
    display.write(OLED_ADDR, &write_buf).unwrap();

    // display progress bar
    let progress_bar: [u8; 8] = [b' ', 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    write_buf[32..32 + progress_bar.len()].copy_from_slice(&progress_bar);

    // Idle loop
    let mut i = 0;
    loop {
        write_buf[40] = progress_bar[i];
        i = i + 1 & 7;
        // send to display.
        display.write(OLED_ADDR, &write_buf).unwrap();
        delay.delay_ms(100_u16);
    }
}
