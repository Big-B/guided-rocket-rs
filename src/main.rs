#![no_std]
#![no_main]
#![feature(never_type)]

use arduino_mkrzero::clock::GenericClockController;
use arduino_mkrzero::delay::Delay;
use arduino_mkrzero::prelude::*;
use arduino_mkrzero::sercom::*;
use arduino_mkrzero::{entry, CorePeripherals, Peripherals};
use core::fmt::Write;
use mpu6050::{Addr, AfSel, FsSel, Mpu6050, Mpu6050Error};

#[entry]
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
        &mut core.NVIC,
        &mut peripherals.PM,
        UART5Pinout::Rx3Tx2 {
            rx: rx_pin,
            tx: tx_pin,
        },
    );

    // This delay seems to be necessary to ensure the next print statement
    // comes out intact
    delay.delay_ms(250u32);
    writeln!(serial, "=======ENTER=======").unwrap();

    // Setup I2C port
    let i2c = I2CMaster0::new(
        &clocks.sercom0_core(&gclk0).unwrap(),
        400.khz(),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.sda.into_pad(&mut pins.port),
        pins.scl.into_pad(&mut pins.port),
    );

    // Instantiate imu instance
    let imu = Mpu6050::new(i2c, Addr::LOW, AfSel::PlusMinus2G, FsSel::PlusMinus250DPS);
    match imu {
        Ok(mut imu) => loop {
            let gyro = imu.gyro_in_dps().unwrap();
            let accl = imu.accel_in_g().unwrap();
            let temp = imu.temp_in_deg_c().unwrap();

            writeln!(serial, "gyro - x: {}, y: {}, z: {}", gyro.x, gyro.y, gyro.z).unwrap();
            writeln!(serial, "accl - x: {}, y: {}, z: {}", accl.x, accl.y, accl.z).unwrap();
            writeln!(serial, "temp: {}", temp).unwrap();

            led.set_low();
            delay.delay_ms(250u32);
            led.set_high();
            delay.delay_ms(250u32);
        },
        Err(Mpu6050Error::I2c(_)) => writeln!(serial, "I2C Error").unwrap(),
        Err(Mpu6050Error::BadIdentifier) => writeln!(serial, "Bad Identifier").unwrap(),
    };

    panic!();
}
