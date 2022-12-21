#![no_std]
#![no_main]

use arduino_hal::{
    default_serial,
    hal::Atmega,
    port::{Pin, PinMode},
    Pins,
};

use panic_halt as _;

use arduino_hal::delay_ms;
use arduino_hal::prelude::*;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let mut d2 = pins.d2.into_output();
    let mut d3 = pins.d3.into_output();
    let mut d4 = pins.d4.into_output();
    let baselinetemp: f64 = 20.0;
    let sensorpin = pins.a0.into_analog_input(&mut adc);

    d2.set_low();
    d3.set_low();
    d4.set_low();

    loop {
        let sensorval = sensorpin.analog_read(&mut adc);

        ufmt::uwriteln!(&mut serial, "Sensor value is: {}", sensorval).void_unwrap();
        let voltage = (sensorval / 1024) * 5;
        let temptemp = voltage as f64;
        let temp = (temptemp - 0.5) * 100.0;
        ufmt::uwriteln!(&mut serial, "Voltage and temp is: {}", voltage).void_unwrap();


        // TODO: MAKE THIS A LOT BETTER HOLY SHIT WHAT THE FUCK IS THIS (I had 2 minutes it works) FEEL FREE TO KILL ME

        if temp < baselinetemp {
            d2.set_low();
            d3.set_low();
            d4.set_low();
        } else if temp >= baselinetemp + 2.0 && temp < baselinetemp + 4.0 {
            d2.set_high();
            d3.set_low();
            d4.set_low();
        } else if temp >= baselinetemp + 4.0 && temp < baselinetemp + 6.0 {
            d2.set_high();
            d3.set_high();
            d4.set_low();
        } else if temp >= baselinetemp + 6.0 {
            d2.set_high();
            d3.set_high();
            d4.set_high();
        }

        arduino_hal::delay_ms(1);
    }
}
