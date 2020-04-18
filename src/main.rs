#![no_std]
#![no_main]
#![feature(never_type)]
extern crate arduino_mkrzero as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::sercom::*;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use core::fmt::Write;
use mpu6050::{Mpu6050, Steps};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut led = pins.led_builtin.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let gclk0 = clocks.gclk0();
    let rx_pin = pins
        .rx
        .into_pull_down_input(&mut pins.port)
        .into_pad(&mut pins.port);
    let tx_pin = pins
        .tx
        .into_push_pull_output(&mut pins.port)
        .into_pad(&mut pins.port);

    // Setup UART device
    let mut serial = UART5::new(
        &clocks.sercom5_core(&gclk0).unwrap(),
        115_200.hz(),
        peripherals.SERCOM5,
        &mut peripherals.PM,
        (rx_pin, tx_pin)
    );

    // This delay seems to be necessary to ensure the next print statement
    // comes out intact
    delay.delay_ms(250u32);
    writeln!(serial, "=======ENTER=======").unwrap();

    // Setup I2C port
    let i2c: I2CMaster0<
        hal::sercom::Sercom0Pad0<hal::gpio::Pa8<hal::gpio::PfC>>,
        hal::sercom::Sercom0Pad1<hal::gpio::Pa9<hal::gpio::PfC>>,
    > = I2CMaster0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        400.khz(),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.sda.into_pad(&mut pins.port),
        pins.scl.into_pad(&mut pins.port),
    );

    // Instantiate imu instance
    let mut imu = Mpu6050::new(i2c);
    imu.init().unwrap();
    imu.soft_calib(Steps(100)).unwrap();

    led.set_low().unwrap();
    delay.delay_ms(250u32);
    led.set_high().unwrap();
    delay.delay_ms(250u32);

    panic!();
}
