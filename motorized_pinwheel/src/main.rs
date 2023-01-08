#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let switchpin = pins.d2.into_floating_input();
    let mut motorpin = pins.d9.into_output();

    loop {
        if switchpin.is_high() {
            motorpin.set_high();
        } else {
            motorpin.set_low();
        }
    }
}
