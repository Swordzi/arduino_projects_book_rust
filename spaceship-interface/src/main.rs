#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut out1 = pins.d3.into_output();
    let mut out2 = pins.d4.into_output();
    let mut out3 = pins.d5.into_output();
    /*
        TODO:
         type annotations neededrustcClick for full compiler diagnostic
    main.rs(18, 22): try using a fully qualified path to specify the expected types: `<avr_hal_generic::port::Pin<Input<Floating>, PD2> as Into<T>>::into(`, `)`

        */
    let in1 = pins.d2.into_floating_input();

    loop {
        if in1.is_low() {
            out1.set_high();
            out2.set_low();
            out3.set_low();
        } else {
            out1.set_low();
            out2.set_low();
            out3.set_high();
        }

        delay_ms(250);
        out2.set_high();
        out3.set_low();
        delay_ms(250)
    }
}
