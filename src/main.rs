#![no_std]
#![no_main]

use arduino_hal::{clock::MHz16, Delay, I2c};
use dht_sensor::dht11;
use dht_sensor::DhtReading;
use embedded_hal::delay::DelayNs;
use lcd_lcm1602_i2c::sync_lcd::Lcd;
use panic_halt as _;
use ufmt::uwriteln;

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

lcd.write_str("Tomasz Bawor").unwrap();
lcd.set_cursor(1, 0);
lcd.write_str("Weather Station").unwrap();

let mut dht_pin = pins.d4;

    let mut delayw = Delay::new();
    let mut dht_out = dht_pin.into_opendrain_high();
loop {
        // Read DHT11 (temperature and humidity)
        match dht11::Reading::read(&mut delayw, &mut dht_out) {
            Ok(data) => {
                lcd.clear().unwrap();
                uwriteln!(lcd, "Temp: {}°C", data.temperature).unwrap();
                lcd.set_cursor(0, 1).unwrap();
                uwriteln!(lcd, "Humidity: {}%", data.relative_humidity).unwrap();
            }
            Err(e) => {
                lcd.clear().unwrap();
                lcd.write_str("DHT11 Error").unwrap();
            }
        }
        
    }
}
