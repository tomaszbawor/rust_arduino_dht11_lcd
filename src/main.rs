#![no_std]
#![no_main]

use arduino_hal::{clock::MHz16, Delay, I2c};
use lcd_lcm1602_i2c::sync_lcd::Lcd;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut i2c = I2c::new(dp.TWI, pins.a4.into_pull_up_input(), pins.a5.into_pull_up_input(), 100000);

    let mut delay = Delay::new();

let mut lcd = Lcd::new(&mut i2c, &mut delay)
    .with_address(0x27)       // I2C address of the LCD (change if not 0x27)
    .with_rows(2)             // 2-line display
    .with_cursor_on(false)    // hide the cursor
    .init().unwrap();         // initialize the LCD (clear it)
    //

lcd.write_str("Rust Arduino").unwrap();


    loop {
        arduino_hal::delay_ms(1000);
    }
}
