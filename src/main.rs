#![no_std]
#![no_main]
#![feature(never_type)]

use arduino_mkrzero::clock::GenericClockController;
use arduino_mkrzero::delay::Delay;
use arduino_mkrzero::prelude::*;
use arduino_mkrzero::{entry, CorePeripherals, Peripherals};
use arduino_mkrzero::sercom::*;
use core::fmt::Write;

entry!(main);
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = arduino_mkrzero::Pins::new(peripherals.PORT);
    let mut led = pins.led_builtin.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let gclk0 = clocks.gclk0();
    let rx_pin = pins.rx.into_pull_down_input(&mut pins.port).into_pad(&mut pins.port);
    let tx_pin = pins.tx.into_push_pull_output(&mut pins.port).into_pad(&mut pins.port);
    let mut serial = UART5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        115200.hz(),
        peripherals.SERCOM5,
        &mut core.NVIC,
        &mut peripherals.PM,
        UART5Pinout::Rx3Tx2 {rx: rx_pin, tx: tx_pin});

    loop {
        led.set_low();
        serial.write_str("Loop\n").unwrap();
        delay.delay_ms(250u32);
        led.set_high();
        delay.delay_ms(250u32);
        led.set_low();
    }
}
